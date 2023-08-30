#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationInner {}

impl LocationInner {
    pub fn new() -> LocationInner {
        LocationInner {}
    }
}

impl Default for LocationInner {
    fn default() -> LocationInner {
        LocationInner::new()
    }
}
