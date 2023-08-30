use bevy::prelude::*;

#[derive(Component)]
pub struct MyNode(pub NodeBundle);

impl Default for MyNode {
    fn default() -> Self {
        Self {
            0: NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct MyButton(pub ButtonBundle);

impl Default for MyButton {
    fn default() -> Self {
        Self {
            0: ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                //background_color: NORMAL_BUTTON.into(),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct MyText(pub TextBundle);

impl Default for MyText {
    fn default() -> Self {
        Self {
            0: TextBundle::from_section(
                "Button",
                TextStyle {
                    font_size: 15.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
        }
    }
}
