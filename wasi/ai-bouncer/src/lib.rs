#[allow(warnings)]
mod bindings;

mod providers;
mod session;

use ai_bouncer_types::{TaskInput, TaskOutput, TaskOutputSuccess};
use bindings::{Guest, Output, TaskQueueInput};
use layer_wasi::{block_on, Reactor};
use providers::Provider;

struct Component;

impl Guest for Component {
    fn run_task(request: TaskQueueInput) -> Output {
        let provider =
            std::env::var("PROVIDER").or(Err("missing env var `PROVIDER`".to_string()))?;
        let lcd = std::env::var("LCD").or(Err("missing env var `LCD`".to_string()))?;

        let env = Env { provider, lcd };

        match serde_json::from_slice(&request.request) {
            Ok(input) => block_on(|reactor| get_output(reactor, env, input)),
            Err(e) => serde_json::to_vec(&TaskOutput::Error(format!(
                "Could not deserialize input request from JSON: {}",
                e
            )))
            .map_err(|e| e.to_string()),
        }
    }
}

async fn get_output(reactor: Reactor, env: Env, input: TaskInput) -> Result<Vec<u8>, String> {
    let session = match env.provider.as_str() {
        providers::ollama::OllamaProvider::NAME => {
            let provider = providers::ollama::OllamaProvider::new()?;

            provider.process(&reactor, &env, &input).await
        }
        providers::groq::GroqProvider::NAME => {
            let provider = providers::groq::GroqProvider::new()?;

            provider.process(&reactor, &env, &input).await
        }
        _ => Err(format!("unknown provider: {}", env.provider)),
    };

    let output = session
        .and_then(|session| {
            session.save()?;

            Ok(TaskOutput::Success(TaskOutputSuccess {
                dao: session.dao,
                address: session.address,
                message_id: input.message_id,
                response: session.messages.last().unwrap().content.clone(),
                decision: session.messages.last().unwrap().decision,
            }))
        })
        .unwrap_or_else(TaskOutput::Error);

    serde_json::to_vec(&output).map_err(|e| e.to_string())
}

pub struct Env {
    pub provider: String,
    pub lcd: String,
}

bindings::export!(Component with_types_in bindings);
