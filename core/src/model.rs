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

/// Represents a data flow between entities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFlow {
    /// Source entity ID
    pub from: String,
    /// Target entity ID
    pub to: String,
    /// Data flow label/description
    pub label: String,
}

impl DataFlow {
    pub fn new(from: String, to: String, label: String) -> Self {
        Self { from, to, label }
    }
}

/// Source location of an annotation in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationSource {
    /// File path where the annotation was found
    pub file_path: String,
    /// Line number where the annotation was found (1-based)
    pub line_number: usize,
    /// Column number where the annotation starts (1-based, optional)
    pub column_number: Option<usize>,
}

impl AnnotationSource {
    pub fn new(file_path: String, line_number: usize) -> Self {
        Self {
            file_path,
            line_number,
            column_number: None,
        }
    }

    pub fn with_column(file_path: String, line_number: usize, column_number: usize) -> Self {
        Self {
            file_path,
            line_number,
            column_number: Some(column_number),
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

    #[test]
    fn test_dataflow_new() {
        let flow = DataFlow::new(
            "User".to_string(),
            "CreateUser".to_string(),
            "user_data".to_string(),
        );

        assert_eq!(flow.from, "User");
        assert_eq!(flow.to, "CreateUser");
        assert_eq!(flow.label, "user_data");
    }

    #[test]
    fn test_dataflow_equality() {
        let flow1 = DataFlow::new(
            "User".to_string(),
            "CreateUser".to_string(),
            "user_data".to_string(),
        );
        let flow2 = DataFlow::new(
            "User".to_string(),
            "CreateUser".to_string(),
            "user_data".to_string(),
        );
        let flow3 = DataFlow::new(
            "User".to_string(),
            "CreateUser".to_string(),
            "different_label".to_string(),
        );

        assert_eq!(flow1, flow2);
        assert_ne!(flow1, flow3);
    }

    #[test]
    fn test_annotationsource_new() {
        let source = AnnotationSource::new("src/model.rs".to_string(), 10);

        assert_eq!(source.file_path, "src/model.rs");
        assert_eq!(source.line_number, 10);
        assert_eq!(source.column_number, None);
    }

    #[test]
    fn test_annotationsource_with_column() {
        let source = AnnotationSource::with_column("src/model.rs".to_string(), 10, 5);

        assert_eq!(source.file_path, "src/model.rs");
        assert_eq!(source.line_number, 10);
        assert_eq!(source.column_number, Some(5));
    }

    #[test]
    fn test_annotationsource_equality() {
        let source1 = AnnotationSource::new("src/model.rs".to_string(), 10);
        let source2 = AnnotationSource::new("src/model.rs".to_string(), 10);
        let source3 = AnnotationSource::new("src/model.rs".to_string(), 20);
        let source4 = AnnotationSource::with_column("src/model.rs".to_string(), 10, 5);

        assert_eq!(source1, source2);
        assert_ne!(source1, source3);
        assert_ne!(source1, source4);
    }
}
