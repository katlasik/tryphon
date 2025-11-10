use crate::printer::field_path::FieldPath;
use crate::{ConfigError, ConfigFieldError};

pub(crate) struct ListPrinter {
    buffer: Vec<String>,
}

impl ListPrinter {
    pub(crate) fn new() -> Self {
        ListPrinter { buffer: vec![] }
    }

    fn print_errors_as_list(
        &mut self,
        errors: &Vec<ConfigFieldError>,
        parent_field_path: FieldPath,
    ) {
        for error in errors {
            match error {
                ConfigFieldError::Nested {
                    field_name,
                    field_idx,
                    error: ConfigError { field_errors },
                } => {
                    let field_path = parent_field_path
                        .with_segment(field_name.clone().unwrap_or(field_idx.to_string()).as_str());
                    self.print_errors_as_list(field_errors, field_path);
                }
                ConfigFieldError::ParsingError {
                    field_name,
                    field_idx,
                    message,
                    env_var_name,
                    raw,
                } => {
                    let field_path = parent_field_path
                        .with_segment(field_name.clone().unwrap_or(field_idx.to_string()).as_str());
                    self.buffer.push(format!(
                        "Parsing error for env var '{}' for field '{}': {} (raw value: {})",
                        env_var_name,
                        field_path.dotted_path(),
                        message,
                        raw
                    ));
                }
                ConfigFieldError::MissingValue {
                    field_name,
                    field_idx,
                    env_vars,
                } => {
                    let field_path = parent_field_path
                        .with_segment(field_name.clone().unwrap_or(field_idx.to_string()).as_str());

                    self.buffer.push(format!(
                        "Missing value for field '{}', tried env vars: {}",
                        field_path,
                        env_vars.join(", ")
                    ));
                }
                ConfigFieldError::Other {
                    field_name,
                    field_idx,
                    message,
                } => {
                    let field_path = parent_field_path
                        .with_segment(field_name.clone().unwrap_or(field_idx.to_string()).as_str());
                    self.buffer.push(format!(
                        "Missing value for field '{}': {}",
                        field_path, message
                    ));
                }
            }
        }
    }

    pub(crate) fn print(&mut self, errors: &Vec<ConfigFieldError>) -> String {
        self.print_errors_as_list(errors, FieldPath::root());
        let header = format!("Found {} configuration error(s):", self.buffer.len());
        header + "\n" + self.buffer.join("\n").as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_error_list() {
        let mut printer = ListPrinter::new();
        let errors = vec![];
        let result = printer.print(&errors);

        assert_eq!(result, "Found 0 configuration error(s):\n");
    }

    #[test]
    fn test_single_parsing_error() {
        let mut printer = ListPrinter::new();
        let errors = vec![ConfigFieldError::ParsingError {
            field_idx: 0,
            field_name: Some("port".to_string()),
            raw: "invalid".to_string(),
            message: "invalid digit found in string".to_string(),
            env_var_name: "PORT".to_string(),
        }];

        let result = printer.print(&errors);

        assert!(result.contains("Found 1 configuration error(s):"));
        assert!(result.contains("Parsing error for env var 'PORT' for field 'port'"));
        assert!(result.contains("invalid digit found in string"));
    }

    #[test]
    fn test_single_missing_value_error() {
        let mut printer = ListPrinter::new();
        let errors = vec![ConfigFieldError::MissingValue {
            field_name: Some("database_url".to_string()),
            field_idx: 0,
            env_vars: vec!["DATABASE_URL".to_string(), "DB_URL".to_string()],
        }];

        let result = printer.print(&errors);

        assert!(result.contains("Found 1 configuration error(s):"));
        assert!(result.contains("Missing value for field 'database_url'"));
        assert!(result.contains("tried env vars: DATABASE_URL, DB_URL"));
    }

    #[test]
    fn test_single_other_error() {
        let mut printer = ListPrinter::new();
        let errors = vec![ConfigFieldError::Other {
            field_idx: 0,
            field_name: Some("custom_field".to_string()),
            message: "custom validation failed".to_string(),
        }];

        let result = printer.print(&errors);

        assert!(result.contains("Found 1 configuration error(s):"));
        assert!(
            result.contains("Missing value for field 'custom_field': custom validation failed")
        );
    }

    #[test]
    fn test_deeply_nested_errors() {
        let mut printer = ListPrinter::new();

        // Create a deeply nested structure: app.database.connection.pool_size
        let deepest_error = vec![ConfigFieldError::ParsingError {
            field_idx: 0,
            field_name: Some("pool_size".to_string()),
            raw: "not_a_number".to_string(),
            message: "invalid digit found in string".to_string(),
            env_var_name: "POOL_SIZE".to_string(),
        }];

        let connection_error = vec![ConfigFieldError::Nested {
            field_idx: 0,
            field_name: Some("connection".to_string()),
            error: ConfigError {
                field_errors: deepest_error,
            },
        }];

        let database_error = vec![ConfigFieldError::Nested {
            field_idx: 0,
            field_name: Some("database".to_string()),
            error: ConfigError {
                field_errors: connection_error,
            },
        }];

        let errors = vec![ConfigFieldError::Nested {
            field_idx: 0,
            field_name: Some("app".to_string()),
            error: ConfigError {
                field_errors: database_error,
            },
        }];

        let result = printer.print(&errors);

        assert!(result.contains("Found 1 configuration error(s):"));
        assert!(result.contains("app.database.connection.pool_size"));
    }

    #[test]
    fn test_fields_without_names_use_index() {
        let mut printer = ListPrinter::new();

        let nested_errors = vec![ConfigFieldError::ParsingError {
            field_idx: 2,
            field_name: None,
            raw: "invalid".to_string(),
            message: "parse error".to_string(),
            env_var_name: "FIELD_2".to_string(),
        }];

        let inner_errors = vec![ConfigFieldError::Nested {
            field_idx: 3,
            field_name: None,
            error: ConfigError {
                field_errors: nested_errors,
            },
        }];

        let outer_errors = vec![ConfigFieldError::Nested {
            field_idx: 3,
            field_name: Some("values".to_string()),
            error: ConfigError {
                field_errors: inner_errors,
            },
        }];

        let result = printer.print(&outer_errors);

        assert!(result.contains("Found 1 configuration error(s):"));
        assert!(result.contains("field 'values.3.2'"));
    }
}
