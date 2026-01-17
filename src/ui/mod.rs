use crate::components::*;
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d::default());

    // Background
    commands.spawn((
        Sprite::from_image(asset_server.load("bg_space_hex.png")),
        Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
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
                        height: Val::Px(80.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        border: UiRect::bottom(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.8)),
                    BorderColor(Color::srgb(0.0, 1.0, 1.0)),
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
                                TextColor(Color::srgb(0.0, 1.0, 1.0)),
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
                                TextColor(Color::srgb(0.5, 0.0, 0.5)),
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
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    GeneratorList,
                ))
                .with_children(|body| {
                    body.spawn((
                        Text::new("Production Online"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
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
    if let Ok(list_entity) = query_list.get_single() {
        for (entity, generator) in query_generators.iter() {
            let container = commands
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        margin: UiRect::bottom(Val::Px(10.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.8)),
                    BorderColor(Color::WHITE),
                ))
                .set_parent(list_entity)
                .id();

            commands.entity(container).with_children(|parent| {
                // Name
                parent.spawn((
                    Text::new(&generator.name),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // Production Output
                parent.spawn((
                    Text::new("Output: 0.00/s"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    GeneratorOutputText(entity),
                ));

                // Count
                parent.spawn((
                    Text::new(format!("x{}", generator.count)),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    GeneratorCountText(entity),
                ));

                // Buy Button
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.5, 0.0, 1.0)),
                        BuyGeneratorButton(entity),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("Buy"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            GeneratorCostText(entity),
                        ));
                    });
            });
        }
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
            let cost_mult = generator
                .cost_coefficient
                .powf(generator.purchased_count.to_f64());
            let cost = generator.base_cost * cost_mult;
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
                "Gen"
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
            let cost_mult = generator
                .cost_coefficient
                .powf(generator.purchased_count.to_f64());
            let cost = generator.base_cost * cost_mult;

            let mut can_afford = false;
            for resource in resources.iter() {
                if resource.name == generator.cost_resource {
                    // Check affordability
                    let diff = resource.amount.exponent - cost.exponent;
                    if diff > 0 {
                        can_afford = true;
                    } else if diff == 0 && resource.amount.mantissa >= cost.mantissa {
                        can_afford = true;
                    }
                    break; // Found the resource, no need to check others
                }
            }

            if can_afford {
                match *interaction {
                    Interaction::Pressed => *bg = BackgroundColor(Color::srgba(0.0, 0.3, 0.0, 1.0)),
                    Interaction::Hovered => *bg = BackgroundColor(Color::srgba(0.0, 0.6, 0.0, 1.0)),
                    Interaction::None => *bg = BackgroundColor(Color::srgba(0.0, 0.5, 0.0, 1.0)),
                }
            } else {
                // Dimmed red if can't afford
                *bg = BackgroundColor(Color::srgba(0.5, 0.0, 0.0, 0.5));
            }
        }
    }
}
