#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionInput {
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "messages")]
    pub messages: Vec<serde_json::Value>,
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(rename = "top_p", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(rename = "stop", skip_serializing_if = "Option::is_none")]
    pub stop: Option<Box<crate::models::Stop>>,
    #[serde(rename = "max_tokens", skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(rename = "presence_penalty", skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(rename = "frequence_penalty", skip_serializing_if = "Option::is_none")]
    pub frequence_penalty: Option<f32>,
    #[serde(rename = "logit_bias", skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<serde_json::Value>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>
}

impl ChatCompletionInput {
    pub fn new(model: String, messages: Vec<serde_json::Value>) -> ChatCompletionInput {
        ChatCompletionInput {
            model,
            messages,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequence_penalty: None,
            logit_bias: None,
            user: None
        }
    }
}


