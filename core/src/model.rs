/// Type of entity in a DFD
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityType {
    /// Process entity (transforms data)
    Process,
    /// Data store (persistent data storage)
    DataStore,
    /// External entity (outside the system boundary)
    ExternalEntity,
}

/// Represents an entity in the domain model
/// An entity can be a process, data store, or external entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Internal identifier (e.g., class name, function name)
    pub id: String,
    /// Display name
    pub display_name: String,
    /// Type of entity
    pub entity_type: EntityType,
}

impl Entity {
    pub fn new(id: String, display_name: String, entity_type: EntityType) -> Self {
        Self {
            id,
            display_name,
            entity_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_new() {
        let entity = Entity::new(
            "user".to_string(),
            "User".to_string(),
            EntityType::DataStore,
        );

        assert_eq!(entity.id, "user");
        assert_eq!(entity.display_name, "User");
        assert_eq!(entity.entity_type, EntityType::DataStore);
    }

    #[test]
    fn test_entity_process() {
        let entity = Entity::new(
            "create_user".to_string(),
            "Create User".to_string(),
            EntityType::Process,
        );

        assert_eq!(entity.entity_type, EntityType::Process);
    }

    #[test]
    fn test_entity_external_entity() {
        let entity = Entity::new(
            "external_api".to_string(),
            "External API".to_string(),
            EntityType::ExternalEntity,
        );

        assert_eq!(entity.entity_type, EntityType::ExternalEntity);
    }

    #[test]
    fn test_entity_equality() {
        let entity1 = Entity::new(
            "user".to_string(),
            "User".to_string(),
            EntityType::DataStore,
        );
        let entity2 = Entity::new(
            "user".to_string(),
            "User".to_string(),
            EntityType::DataStore,
        );
        let entity3 = Entity::new(
            "user".to_string(),
            "User Entity".to_string(),
            EntityType::DataStore,
        );

        assert_eq!(entity1, entity2);
        assert_ne!(entity1, entity3);
    }
}
