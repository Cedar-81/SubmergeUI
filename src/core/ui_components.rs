use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub struct UiComponent {
    pub id: String,
    pub children: Vec<String>,
}

impl Default for UiComponent {
    fn default() -> Self {
        Self {
            id: String::new(),
            children: Vec::new(),
        }
    }
}
