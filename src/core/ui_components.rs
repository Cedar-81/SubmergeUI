use bevy::{math::Vec2, prelude::Component};

#[derive(Component, Clone, Debug)]
pub struct UiComponent {
    pub id: String,
    pub children: Vec<String>,
}

/// This system makes is possible to use custom id's with any ui bundle even those that comes with bevy itself.
#[derive(Component, Clone, Debug)]
pub struct SubmergeId(pub String);

impl Default for UiComponent {
    fn default() -> Self {
        Self {
            id: String::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Draggable;

#[derive(Component, Clone, Debug)]
pub struct Dragging {
    pub offset: Vec2,
}
