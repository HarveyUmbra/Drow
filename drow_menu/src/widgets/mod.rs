use bevy::prelude::*;

use super::style::DEFAULT_COLOR;

#[derive(Debug, Bundle)]
pub struct ButtonBundle {
    button: Button,
    background_color: BackgroundColor,
    node: Node,
    text: Text,
    text_layout: TextLayout,
}

impl ButtonBundle {
    pub fn new(string: String) -> Self {
        return ButtonBundle {
            button: Button,
            background_color: BackgroundColor(DEFAULT_COLOR),
            node: Node {
                width: px(100.0),
                height: px(20.0),
                ..Default::default()
            },
            text: Text(string),
            text_layout: TextLayout::new_with_justify(Justify::Center),
        };
    }
}
