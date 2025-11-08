use std::fmt::Display;
use std::str::FromStr;

/// Internal helper for tracking nested field paths in error messages.
///
/// Used by error printers to display hierarchical field names like "database.host" or "server.port".
#[derive(Clone, Debug)]
pub(crate) struct FieldPath {
    segments: Vec<String>,
}

impl FieldPath {
    /// Creates a new empty path representing the root level.
    pub fn root() -> Self {
        FieldPath {
            segments: Vec::new(),
        }
    }

    /// Creates a new path by appending a segment to this path.
    pub fn with_segment(&self, rhs: &str) -> FieldPath {
        let mut new_path = self.clone();
        new_path.segments.push(rhs.to_string());
        new_path
    }

    /// Converts the path to a dot-separated string representation.
    pub fn dotted_path(&self) -> String {
        self.segments.join(".")
    }
}

impl FromStr for FieldPath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(FieldPath::root())
        } else {
            let segments = s
                .split('.')
                .map(|segment| segment.to_string())
                .collect::<Vec<String>>();
            Ok(FieldPath { segments })
        }
    }
}

impl Display for FieldPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dotted_path())
    }
}
