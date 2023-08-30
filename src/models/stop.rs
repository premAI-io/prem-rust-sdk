#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stop {}

impl Stop {
    pub fn new() -> Stop {
        Stop {}
    }
}

impl Default for Stop {
    fn default() -> Stop {
        Stop::new()
    }
}
