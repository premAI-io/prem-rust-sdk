#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<crate::models::LocationInner>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ValidationError {
    pub fn new(
        loc: Vec<crate::models::LocationInner>,
        msg: String,
        r#type: String,
    ) -> ValidationError {
        ValidationError { loc, msg, r#type }
    }
}
