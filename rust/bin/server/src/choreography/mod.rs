//! Choreography of lights

/// Light choreography
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Choreography {
    /// Choreography name.
    pub name: String,

    /// Format
    pub format: Format,

    /// Data
    pub data: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Format {
    /// Python3 light choreography.
    Python,
}
