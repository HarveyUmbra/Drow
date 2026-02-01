use super::{
    logic::*,
    style::*,
    widgets::*,
};
use bevy::prelude::*;
use drow_core::prelude::*;

pub fn spawn_menu(mut commands: Commands) {
    commands.spawn((Camera3d::default(), DespawnOnExit(AppState::Menu)));
    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(5.0),
                ..default()
            },
            //BackgroundColor(BACKGROUND_COLOR.into()),
            DespawnOnExit(AppState::Menu),
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle::new("Play".to_string()))
                .observe(on_play);
            parent.spawn(ButtonBundle::new("Load".to_string()));
            parent.spawn(ButtonBundle::new("Save".to_string()));
            parent
                .spawn(ButtonBundle::new("Quit".to_string()))
                .observe(on_quit);
        });
}

pub fn spawn_stop_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(5.0),
                ..default()
            },
            DespawnOnExit(GameState::Stop),
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle::new("Back".to_string()))
                .observe(on_back);
            parent
                .spawn(ButtonBundle::new("Menu".to_string()))
                .observe(on_menu);
        });
}

pub fn button_system(mut query: Query<(&mut BackgroundColor, &Interaction), Changed<Interaction>>) {
    for (mut background, interaction) in query.iter_mut() {
        *background = match interaction {
            Interaction::Hovered => HOVERED_COLOR.into(),
            Interaction::Pressed => PRESSED_COLOR.into(),
            Interaction::None => DEFAULT_COLOR.into(),
        };
    }
}
