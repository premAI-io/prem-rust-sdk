use prem as lib;
use serde_json::json;

#[tokio::test]
async fn test_chat_completions() {
    let config = lib::apis::configuration::Configuration {
        base_path: "https://vicuna-7b-q4.prem.ninja".to_owned(),
        client: reqwest::Client::new(),
    };

    let chat_completion_input = lib::models::chat_completion_input::ChatCompletionInput {
        model: "model".to_owned(),
        messages: vec![json!({
            "content": "Hello, world!", "role": "user"
        })],
        temperature: None,
        top_p: None,
        n: None,
        stream: Some(false),
        stop: None,
        max_tokens: None,
        presence_penalty: None,
        frequence_penalty: None,
        logit_bias: None,
        user: None,
    };

    let result = lib::apis::default_api::chat(&config, chat_completion_input).await;

    match result {
        Ok(response) => {
            assert_eq!(response.model, "./ml/models/vicuna-7b-q4.bin".to_owned());
        }
        Err(e) => panic!("API call failed: {:?}", e),
    }
}
