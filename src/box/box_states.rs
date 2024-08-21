use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum EntityActiveState {
    Active,
    #[default]
    Inactive,
}
