pub fn datetime_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    use schemars::schema::{Schema, SchemaObject, InstanceType, Metadata};
    use serde_json::json;

    let mut schema = SchemaObject {
        instance_type: Some(InstanceType::String.into()),
        format: Some("date-time".to_string()),
        ..Default::default()
    };

    schema.metadata = Some(Box::new(Metadata {
        description: Some("ISO 8601/RFC 3339 date-time string".to_string()),
        examples: vec![json!("2023-05-15T14:30:00Z")],
        ..Default::default()
    }));

    Schema::Object(schema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use schemars::gen::SchemaGenerator;
    use schemars::schema::{InstanceType, Schema};

    #[test]
    fn test_datetime_schema() {
        let mut generator = SchemaGenerator::default();
        let schema = datetime_schema(&mut generator);

        match schema {
            Schema::Object(obj) => {
                assert_eq!(obj.instance_type, Some(InstanceType::String.into()));
                assert_eq!(obj.format, Some("date-time".to_string()));
                assert_eq!(obj.metadata.as_ref().unwrap().description, Some("ISO 8601/RFC 3339 date-time string".to_string()));
                assert_eq!(obj.metadata.as_ref().unwrap().examples, vec![serde_json::json!("2023-05-15T14:30:00Z")]);
            }
            _ => panic!("Expected Schema::Object"),
        }
    }
}