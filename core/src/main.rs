mod model;

use model::{DataFlow, Entity, EntityType};

fn main() {
    println!("dojigiri-core");

    // Tracer bullet: Create and display entities
    let user_entity = Entity::new(
        "User".to_string(),
        "User Repository".to_string(),
        EntityType::DataStore,
    );

    let create_user_process = Entity::new(
        "CreateUser".to_string(),
        "Create User".to_string(),
        EntityType::Process,
    );

    let external_api = Entity::new(
        "ExternalAPI".to_string(),
        "External API".to_string(),
        EntityType::ExternalEntity,
    );

    println!("\nEntities created:");
    println!(
        "  ID: {}, Display: {}, Type: {:?}",
        user_entity.id, user_entity.display_name, user_entity.entity_type
    );
    println!(
        "  ID: {}, Display: {}, Type: {:?}",
        create_user_process.id, create_user_process.display_name, create_user_process.entity_type
    );
    println!(
        "  ID: {}, Display: {}, Type: {:?}",
        external_api.id, external_api.display_name, external_api.entity_type
    );

    // Tracer bullet: Create and display data flow
    let data_flow = DataFlow::new(
        "User".to_string(),
        "CreateUser".to_string(),
        "user_data".to_string(),
    );

    println!("\nData flow created:");
    println!(
        "  From: {} -> To: {}, Label: {}",
        data_flow.from, data_flow.to, data_flow.label
    );
}
