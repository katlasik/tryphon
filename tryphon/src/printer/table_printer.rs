use crate::printer::field_path::FieldPath;
use crate::{ConfigError, ConfigFieldError};

pub(crate) struct TablePrinter {
    rows: Vec<(String, String, String)>,
}

impl TablePrinter {
    pub(crate) fn new() -> Self {
        TablePrinter { rows: vec![] }
    }

    fn collect_errors_as_rows(
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
                    self.collect_errors_as_rows(field_errors, field_path);
                }
                ConfigFieldError::ParsingError {
                    field_name,
                    field_idx,
                    raw,
                    message,
                    env_var_name,
                } => {
                    let field_path = parent_field_path
                        .with_segment(field_name.clone().unwrap_or(field_idx.to_string()).as_str());
                    self.rows.push((
                        field_path.dotted_path(),
                        env_var_name.clone(),
                        format!("{} (raw value: '{}')", message, raw),
                    ));
                }
                ConfigFieldError::MissingValue {
                    field_name,
                    field_idx,
                    env_vars,
                } => {
                    let field_path = parent_field_path
                        .with_segment(field_name.clone().unwrap_or(field_idx.to_string()).as_str());
                    self.rows.push((
                        field_path.dotted_path(),
                        env_vars.join(", "),
                        "Required variable not set".to_string(),
                    ));
                }
                ConfigFieldError::Other {
                    field_name,
                    field_idx,
                    message,
                } => {
                    let field_path = parent_field_path
                        .with_segment(field_name.clone().unwrap_or(field_idx.to_string()).as_str());
                    self.rows
                        .push((field_path.dotted_path(), "-".to_string(), message.clone()));
                }
            }
        }
    }

    pub(crate) fn print(&mut self, errors: &Vec<ConfigFieldError>) -> String {
        self.collect_errors_as_rows(errors, FieldPath::root());

        if self.rows.is_empty() {
            return "No configuration errors\n".to_string();
        }

        let headers = ["Field Name", "Environment Variables", "Error Details"];

        format_ascii_table(&headers, &self.rows)
    }
}

fn calculate_column_widths(headers: &[&str; 3], rows: &[(String, String, String)]) -> [usize; 3] {
    let mut widths = [headers[0].len(), headers[1].len(), headers[2].len()];

    for row in rows {
        widths[0] = widths[0].max(row.0.len());
        widths[1] = widths[1].max(row.1.len());
        widths[2] = widths[2].max(row.2.len());
    }

    widths
}

fn top_border(buffer: &mut String, widths: &[usize; 3]) {
    buffer.push_str(&format_border(widths, "┌", "┬", "┐"));
}

fn header_row(buffer: &mut String, headers: &[&str; 3], widths: &[usize; 3]) {
    buffer.push_str(&format_row(&[headers[0], headers[1], headers[2]], widths));
}

fn header_separator(buffer: &mut String, widths: &[usize; 3]) {
    buffer.push_str(&format_border(widths, "├", "┼", "┤"));
}

fn data_rows(buffer: &mut String, rows: &[(String, String, String)], widths: &[usize; 3]) {
    for row in rows {
        let row_strs = [row.0.as_str(), row.1.as_str(), row.2.as_str()];
        buffer.push_str(&format_row(&row_strs, widths));
    }
}

fn bottom_border(buffer: &mut String, widths: &[usize; 3]) {
    buffer.push_str(&format_border(widths, "└", "┴", "┘"));
}

fn format_ascii_table(headers: &[&str; 3], rows: &[(String, String, String)]) -> String {
    let widths = calculate_column_widths(headers, rows);
    let mut output = String::new();

    top_border(&mut output, &widths);
    header_row(&mut output, headers, &widths);
    header_separator(&mut output, &widths);
    data_rows(&mut output, rows, &widths);
    bottom_border(&mut output, &widths);

    output
}

fn format_border(widths: &[usize; 3], left: &str, mid: &str, right: &str) -> String {
    format!(
        "{}{}{}{}{}{}{}\n",
        left,
        "─".repeat(widths[0] + 2),
        mid,
        "─".repeat(widths[1] + 2),
        mid,
        "─".repeat(widths[2] + 2),
        right
    )
}

fn format_row(cells: &[&str; 3], widths: &[usize; 3]) -> String {
    format!(
        "│ {:<width0$} │ {:<width1$} │ {:<width2$} │\n",
        cells[0],
        cells[1],
        cells[2],
        width0 = widths[0],
        width1 = widths[1],
        width2 = widths[2]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_error_list() {
        let mut printer = TablePrinter::new();
        let errors = vec![];
        let result = printer.print(&errors);

        assert_eq!(result, "No configuration errors\n");
    }

    #[test]
    fn test_single_parsing_error() {
        let mut printer = TablePrinter::new();
        let errors = vec![ConfigFieldError::ParsingError {
            field_idx: 0,
            field_name: Some("port".to_string()),
            raw: "invalid".to_string(),
            message: "invalid digit found in string".to_string(),
            env_var_name: "PORT".to_string(),
        }];

        let result = printer.print(&errors);

        assert!(result.contains("Field Name"));
        assert!(result.contains("Environment Variables"));
        assert!(result.contains("Error Details"));
        assert!(result.contains("port"));
        assert!(result.contains("PORT"));
        assert!(result.contains("invalid digit found in string"));
        assert!(result.contains("(raw value: 'invalid')"));
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
    }

    #[test]
    fn test_single_missing_value_error() {
        let mut printer = TablePrinter::new();
        let errors = vec![ConfigFieldError::MissingValue {
            field_name: Some("database_url".to_string()),
            field_idx: 0,
            env_vars: vec!["DATABASE_URL".to_string(), "DB_URL".to_string()],
        }];

        let result = printer.print(&errors);

        assert!(result.contains("database_url"));
        assert!(result.contains("DATABASE_URL, DB_URL"));
        assert!(result.contains("Required variable not set"));
    }

    #[test]
    fn test_single_other_error() {
        let mut printer = TablePrinter::new();
        let errors = vec![ConfigFieldError::Other {
            field_idx: 0,
            field_name: Some("custom_field".to_string()),
            message: "custom validation failed".to_string(),
        }];

        let result = printer.print(&errors);

        assert!(result.contains("custom_field"));
        assert!(result.contains("-"));
        assert!(result.contains("custom validation failed"));
    }

    #[test]
    fn test_nested_error() {
        let mut printer = TablePrinter::new();
        let nested_errors = vec![ConfigFieldError::ParsingError {
            field_idx: 0,
            field_name: Some("host".to_string()),
            raw: "".to_string(),
            message: "empty string not allowed".to_string(),
            env_var_name: "DB_HOST".to_string(),
        }];

        let errors = vec![ConfigFieldError::Nested {
            field_idx: 0,
            field_name: Some("database".to_string()),
            error: ConfigError {
                field_errors: nested_errors,
            },
        }];

        let result = printer.print(&errors);

        assert!(result.contains("database.host"));
        assert!(result.contains("DB_HOST"));
    }

    #[test]
    fn test_multiple_errors_at_same_level() {
        let mut printer = TablePrinter::new();
        let errors = vec![
            ConfigFieldError::MissingValue {
                field_name: Some("api_key".to_string()),
                field_idx: 0,
                env_vars: vec!["API_KEY".to_string()],
            },
            ConfigFieldError::ParsingError {
                field_idx: 1,
                field_name: Some("timeout".to_string()),
                raw: "abc".to_string(),
                message: "invalid digit found in string".to_string(),
                env_var_name: "TIMEOUT".to_string(),
            },
            ConfigFieldError::Other {
                field_idx: 2,
                field_name: Some("region".to_string()),
                message: "unsupported region".to_string(),
            },
        ];

        let result = printer.print(&errors);

        assert!(result.contains("api_key"));
        assert!(result.contains("timeout"));
        assert!(result.contains("region"));
        // Check for proper table structure with 3 rows
        let border_count = result.matches("├").count();
        assert_eq!(border_count, 1); // Only header separator
    }

    #[test]
    fn test_deeply_nested_errors() {
        let mut printer = TablePrinter::new();

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

        assert!(result.contains("app.database.connection.pool_size"));
    }

    #[test]
    fn test_field_without_name_uses_index() {
        let mut printer = TablePrinter::new();
        let errors = vec![ConfigFieldError::ParsingError {
            field_idx: 2,
            field_name: None,
            raw: "invalid".to_string(),
            message: "parse error".to_string(),
            env_var_name: "FIELD_2".to_string(),
        }];

        let result = printer.print(&errors);

        assert!(result.contains("2"));
    }

    #[test]
    fn test_multiple_nested_structures() {
        let mut printer = TablePrinter::new();

        let db_errors = vec![ConfigFieldError::MissingValue {
            field_name: Some("host".to_string()),
            field_idx: 0,
            env_vars: vec!["DB_HOST".to_string()],
        }];

        let cache_errors = vec![ConfigFieldError::ParsingError {
            field_idx: 0,
            field_name: Some("ttl".to_string()),
            raw: "forever".to_string(),
            message: "invalid duration".to_string(),
            env_var_name: "CACHE_TTL".to_string(),
        }];

        let errors = vec![
            ConfigFieldError::Nested {
                field_idx: 0,
                field_name: Some("database".to_string()),
                error: ConfigError {
                    field_errors: db_errors,
                },
            },
            ConfigFieldError::Nested {
                field_idx: 1,
                field_name: Some("cache".to_string()),
                error: ConfigError {
                    field_errors: cache_errors,
                },
            },
        ];

        let result = printer.print(&errors);

        assert!(result.contains("database.host"));
        assert!(result.contains("cache.ttl"));
    }

    #[test]
    fn test_table_structure() {
        let mut printer = TablePrinter::new();
        let errors = vec![ConfigFieldError::Other {
            field_idx: 0,
            field_name: Some("test".to_string()),
            message: "test error".to_string(),
        }];

        let result = printer.print(&errors);

        // Check for proper box drawing characters
        assert!(result.contains("┌")); // Top-left corner
        assert!(result.contains("┐")); // Top-right corner
        assert!(result.contains("└")); // Bottom-left corner
        assert!(result.contains("┘")); // Bottom-right corner
        assert!(result.contains("├")); // Left T-junction
        assert!(result.contains("┤")); // Right T-junction
        assert!(result.contains("┼")); // Cross
        assert!(result.contains("─")); // Horizontal line
        assert!(result.contains("│")); // Vertical line
    }

    #[test]
    fn test_column_width_adjustment() {
        let mut printer = TablePrinter::new();
        let errors = vec![
            ConfigFieldError::Other {
                field_idx: 0,
                field_name: Some("short".to_string()),
                message: "short".to_string(),
            },
            ConfigFieldError::Other {
                field_idx: 1,
                field_name: Some("very_long_field_name_that_should_adjust_width".to_string()),
                message: "This is a very long error message that should cause the column to expand"
                    .to_string(),
            },
        ];

        let result = printer.print(&errors);

        // Verify both errors are present
        assert!(result.contains("short"));
        assert!(result.contains("very_long_field_name_that_should_adjust_width"));
        assert!(result.contains("This is a very long error message"));

        // Verify table structure is maintained
        let lines: Vec<&str> = result.lines().collect();
        if lines.len() > 2 {
            // All content rows should have the same width
            let first_line_len = lines[0].chars().count();
            for line in &lines {
                assert_eq!(
                    line.chars().count(),
                    first_line_len,
                    "All rows should have equal width"
                );
            }
        }
    }
}
