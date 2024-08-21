use bevy::prelude::*;

// Event for storing the clicked entity's ID
#[derive(Event)]
pub struct EntityClickedEvent {
    pub entity: Entity,
}

pub fn detect_click_interaction(
    mut interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut ev_writer: EventWriter<EntityClickedEvent>,
) {
    for (entity, interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Send the event with the entity's ID
            ev_writer.send(EntityClickedEvent { entity });
            println!("Entity clicked: {:?}", entity);
        }
    }
}

pub fn handle_click_events(mut event_reader: EventReader<EntityClickedEvent>) {
    for event in event_reader.read() {
        // println!("Entity clicked: {:?}", event.entity);
    }
}
