#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpValidationError {
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<crate::models::ValidationError>>,
}

impl HttpValidationError {
    pub fn new() -> HttpValidationError {
        HttpValidationError { detail: None }
    }
}

impl Default for HttpValidationError {
    fn default() -> HttpValidationError {
        HttpValidationError::new()
    }
}
