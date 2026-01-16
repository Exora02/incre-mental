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
                .spawn((Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
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
