#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthResponse {
    #[serde(rename = "status")]
    pub status: bool,
}

impl HealthResponse {
    pub fn new(status: bool) -> HealthResponse {
        HealthResponse {
            status,
        }
    }
}


