use crate::components::*;
use crate::systems::{can_afford, generator_cost};
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UiTab {
    #[default]
    Generators,
    Upgrades,
}

#[derive(Component)]
pub struct TabButton(pub UiTab);

#[derive(Component)]
pub struct UpgradeList;

#[derive(Component)]
pub struct UpgradeCostText(Entity);

#[derive(Component)]
pub struct UpgradeNameText(Entity);


struct UiTheme;

impl UiTheme {
    const HEADER_HEIGHT: f32 = 84.0;
    const CARD_HEIGHT: f32 = 72.0;
    const CARD_BORDER: f32 = 1.0;
    const CONTENT_PADDING: f32 = 12.0;
    const CARD_MARGIN: f32 = 10.0;
    const BUTTON_WIDTH: f32 = 126.0;
    const BUTTON_HEIGHT: f32 = 42.0;

    fn panel_bg() -> Color {
        Color::srgba(0.06, 0.08, 0.13, 0.86)
    }

    fn card_bg() -> Color {
        Color::srgba(0.10, 0.13, 0.20, 0.92)
    }

    fn accent() -> Color {
        Color::srgb(0.27, 0.83, 0.94)
    }

    fn accent_soft() -> Color {
        Color::srgb(0.64, 0.70, 0.86)
    }

    fn button_ready() -> Color {
        Color::srgba(0.20, 0.64, 0.48, 1.0)
    }

    fn button_ready_hover() -> Color {
        Color::srgba(0.24, 0.72, 0.54, 1.0)
    }

    fn button_ready_pressed() -> Color {
        Color::srgba(0.16, 0.52, 0.40, 1.0)
    }

    fn button_locked() -> Color {
        Color::srgba(0.42, 0.18, 0.22, 0.65)
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);

    // Background
    commands.spawn((
        Sprite::from_image(asset_server.load("bg_space_hex.png")),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    // Root node
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            ..default()
        },))
        .with_children(|parent| {
            // Header
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(UiTheme::HEADER_HEIGHT),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        border: UiRect::bottom(Val::Px(UiTheme::CARD_BORDER)),
                        padding: UiRect::horizontal(Val::Px(UiTheme::CONTENT_PADDING)),
                        ..default()
                    },
                    BackgroundColor(UiTheme::panel_bg()),
                    BorderColor::all(UiTheme::accent()),
                ))
                .with_children(|header| {
                    // Energy Display
                    header
                        .spawn((Node {
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|container| {
                            // Icon
                            container.spawn((
                                ImageNode::new(asset_server.load("icon_energy.png")),
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            // Text
                            container.spawn((
                                Text::new("0"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(UiTheme::accent()),
                                ResourceDisplay {
                                    resource_name: "Energy".to_string(),
                                },
                            ));
                        });

                    // Matter Display
                    header
                        .spawn((Node {
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|container| {
                            // Icon
                            container.spawn((
                                ImageNode::new(asset_server.load("icon_matter.png")),
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            // Text
                            container.spawn((
                                Text::new("0"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(UiTheme::accent_soft()),
                                ResourceDisplay {
                                    resource_name: "Matter".to_string(),
                                },
                            ));
                        });
                });

            // Main Content Area
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(UiTheme::CONTENT_PADDING)),
                        ..default()
                    },
                    GeneratorList,
                ))
                .with_children(|body| {
                    body.spawn((
                        Text::new("Production Grid"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(UiTheme::accent()),
                    ));
                });
        });
}

#[derive(Component)]
pub struct GeneratorList;

#[derive(Component)]
pub struct GeneratorCostText(Entity);

#[derive(Component)]
pub struct GeneratorCountText(Entity);

#[derive(Component)]
pub struct GeneratorOutputText(Entity);

#[derive(Component)]
pub struct ResourceDisplay {
    pub resource_name: String,
}

pub fn update_resource_ui(
    resources: Query<&GameResource>,
    mut ui_text: Query<(&mut Text, &ResourceDisplay)>,
) {
    for (mut text, display) in ui_text.iter_mut() {
        for resource in resources.iter() {
            if resource.name == display.resource_name {
                text.0 = format!("{:.2}", resource.amount);
            }
        }
    }
}

pub fn spawn_generator_ui(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    query_generators: Query<(Entity, &Generator), Added<Generator>>,
    query_list: Query<Entity, With<GeneratorList>>,
) {
    if let Ok(list_entity) = query_list.single() {
        commands.entity(list_entity).with_children(|list| {
            for (entity, generator) in query_generators.iter() {
                list.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(UiTheme::CARD_HEIGHT),
                        margin: UiRect::bottom(Val::Px(UiTheme::CARD_MARGIN)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(Val::Px(UiTheme::CONTENT_PADDING)),
                        border: UiRect::all(Val::Px(UiTheme::CARD_BORDER)),
                        ..default()
                    },
                    BackgroundColor(UiTheme::card_bg()),
                    BorderColor::all(UiTheme::accent_soft()),
                    children![
                        (
                            Text::new(&generator.name),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ),
                        (
                            Text::new("Output: 0.00/s"),
                            TextFont {
                                font_size: 13.0,
                                ..default()
                            },
                            TextColor(UiTheme::accent_soft()),
                            GeneratorOutputText(entity),
                        ),
                        (
                            Text::new(format!("x{}", generator.count)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            GeneratorCountText(entity),
                        ),
                        (
                            Button,
                            Node {
                                width: Val::Px(UiTheme::BUTTON_WIDTH),
                                height: Val::Px(UiTheme::BUTTON_HEIGHT),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(UiTheme::CARD_BORDER)),
                                ..default()
                            },
                            BorderColor::all(UiTheme::accent_soft()),
                            BackgroundColor(UiTheme::button_ready()),
                            BuyGeneratorButton(entity),
                            children![
                                (
                                    Text::new("Buy"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                    GeneratorCostText(entity),
                                )
                            ],
                        ),
                    ],
                ));
            }
        });
    }
}

pub fn update_generator_text(
    generators: Query<&Generator>,
    mut cost_text: Query<
        (&mut Text, &GeneratorCostText),
        (Without<GeneratorCountText>, Without<GeneratorOutputText>),
    >,
    mut count_text: Query<
        (&mut Text, &GeneratorCountText),
        (Without<GeneratorCostText>, Without<GeneratorOutputText>),
    >,
    mut output_text: Query<
        (&mut Text, &GeneratorOutputText),
        (Without<GeneratorCostText>, Without<GeneratorCountText>),
    >,
) {
    for (mut text, link) in cost_text.iter_mut() {
        if let Ok(generator) = generators.get(link.0) {
            let cost = generator_cost(generator);
            text.0 = format!("Buy: {:.1}", cost);
        }
    }

    for (mut text, link) in count_text.iter_mut() {
        if let Ok(generator) = generators.get(link.0) {
            text.0 = format!("x{}", generator.count);
        }
    }

    for (mut text, link) in output_text.iter_mut() {
        if let Ok(generator) = generators.get(link.0) {
            let production = generator.base_production * generator.count.to_f64();
            let output_name = if generator.output_resource == "Energy" {
                "Energy"
            } else {
                "Lower Tier"
            };
            text.0 = format!("+{:.2} {}/s", production, output_name);
        }
    }
}

pub fn update_generator_ui(
    generators: Query<&Generator>,
    resources: Query<&GameResource>,
    mut buttons: Query<(&BuyGeneratorButton, &mut BackgroundColor, &Interaction), With<Button>>,
) {
    for (link, mut bg, interaction) in buttons.iter_mut() {
        if let Ok(generator) = generators.get(link.0) {
            let cost = generator_cost(generator);

            let mut is_affordable = false;
            for resource in resources.iter() {
                if resource.name == generator.cost_resource {
                    is_affordable = can_afford(resource.amount, cost);
                    break; // Found the resource, no need to check others
                }
            }

            if is_affordable {
                match *interaction {
                    Interaction::Pressed => *bg = BackgroundColor(UiTheme::button_ready_pressed()),
                    Interaction::Hovered => *bg = BackgroundColor(UiTheme::button_ready_hover()),
                    Interaction::None => *bg = BackgroundColor(UiTheme::button_ready()),
                }
            } else {
                *bg = BackgroundColor(UiTheme::button_locked());
            }
        }
    }
}


pub fn handle_tab_switches(
    mut next_state: ResMut<NextState<UiTab>>,
    interactions: Query<(&Interaction, &TabButton), Changed<Interaction>>,
) {
    for (interaction, tab_button) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(tab_button.0);
        }
    }
}

pub fn update_tab_ui(
    current_tab: Res<State<UiTab>>,
    mut tab_buttons: Query<(&TabButton, &mut BackgroundColor)>,
    mut lists: Query<(Entity, &mut Node), Or<(With<GeneratorList>, With<UpgradeList>)>>,
    has_gen: Query<(), With<GeneratorList>>,
) {
    if !current_tab.is_changed() { return; }

    // Update button colors
    for (button, mut bg) in tab_buttons.iter_mut() {
        if button.0 == *current_tab.get() {
            *bg = BackgroundColor(UiTheme::button_ready());
        } else {
            *bg = BackgroundColor(UiTheme::button_locked());
        }
    }

    // Toggle list visibility
    for (entity, mut node) in lists.iter_mut() {
        let is_gen_list = has_gen.get(entity).is_ok();
        if (is_gen_list && *current_tab.get() == UiTab::Generators) ||
           (!is_gen_list && *current_tab.get() == UiTab::Upgrades) {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }
}

pub fn spawn_upgrade_ui(
    mut commands: Commands,
    query_upgrades: Query<(Entity, &Upgrade), Added<Upgrade>>,
    query_list: Query<Entity, With<UpgradeList>>,
) {
    if let Ok(list_entity) = query_list.get_single() {
        commands.entity(list_entity).with_children(|list| {
            for (entity, upgrade) in query_upgrades.iter() {
                list.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(UiTheme::CARD_HEIGHT * 1.2), // slightly taller
                        margin: UiRect::bottom(Val::Px(UiTheme::CARD_MARGIN)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(Val::Px(UiTheme::CONTENT_PADDING)),
                        border: UiRect::all(Val::Px(UiTheme::CARD_BORDER)),
                        ..default()
                    },
                    BackgroundColor(UiTheme::card_bg()),
                    BorderColor(UiTheme::accent_soft().into()),
                )).with_children(|card| {
                    card.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    }).with_children(|info| {
                        info.spawn((Text::new(&upgrade.name), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE), UpgradeNameText(entity)));
                        info.spawn((Text::new(&upgrade.description), TextFont { font_size: 14.0, ..default() }, TextColor(UiTheme::accent_soft())));
                    });

                    card.spawn((
                        Button,
                        Node {
                            width: Val::Px(UiTheme::BUTTON_WIDTH),
                            height: Val::Px(UiTheme::BUTTON_HEIGHT),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(UiTheme::CARD_BORDER)),
                            ..default()
                        },
                        BorderColor(UiTheme::accent_soft().into()),
                        BackgroundColor(UiTheme::button_ready()),
                        BuyUpgradeButton(entity),
                    )).with_children(|btn| {
                        btn.spawn((Text::new(format!("Cost: {}", upgrade.cost_amount)), TextFont { font_size: 14.0, ..default() }, TextColor(Color::WHITE), UpgradeCostText(entity)));
                    });
                });
            }
        });
    }
}
