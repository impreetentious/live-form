//! Record shapes.

/// Stable shape identifier. Never reused within a runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ShapeId(u32);

impl ShapeId {
    /// Wrap a raw identifier.
    #[must_use]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Raw identifier.
    #[must_use]
    pub fn get(self) -> u32 {
        self.0
    }
}

/// Named type version and field layout.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Shape {
    type_name: String,
    version: u32,
    fields: Vec<String>,
}

impl Shape {
    /// Construct a shape. Field order is layout order.
    #[must_use]
    pub fn new(type_name: impl Into<String>, version: u32, fields: Vec<String>) -> Self {
        Self {
            type_name: type_name.into(),
            version,
            fields,
        }
    }

    /// Type name.
    #[must_use]
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Shape version.
    #[must_use]
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Layout fields.
    #[must_use]
    pub fn fields(&self) -> &[String] {
        &self.fields
    }
}
