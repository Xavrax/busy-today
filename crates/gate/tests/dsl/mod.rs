use anyhow::{anyhow, Result};
use cucumber_rust::{async_trait, World, WorldInit};
use cucumber_rust::{given, then, when};
use reqwest::{Response, StatusCode, Url};
use serde_json::Value;
use std::convert::Infallible;

pub mod days;

#[derive(WorldInit)]
pub struct ScenarioEnvironment {
    pub endpoint: String,
    pub port: u16,
    pub request: Option<Value>,
    pub response: Option<Response>,
    pub expected_response: Option<Value>,
}

#[async_trait(?Send)]
impl World for ScenarioEnvironment {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            endpoint: "".into(),
            port: portpicker::pick_unused_port().unwrap(),
            request: None,
            response: None,
            expected_response: None,
        })
    }
}

#[given(regex = "configured (.*) endpoint")]
async fn configure_endpoint(env: &mut ScenarioEnvironment, endpoint: String) {
    env.endpoint = endpoint
}

#[when("request is sent")]
async fn send_request(env: &mut ScenarioEnvironment) {
    let client = reqwest::Client::new();
    env.response = Some(
        client
            .post(Url::parse(&format!("http://localhost:{}/{}", env.port, env.endpoint)).unwrap())
            .json(&env.request.take().unwrap())
            .send()
            .await
            .unwrap(),
    )
}

#[then("response is valid")]
fn check_response(env: &mut ScenarioEnvironment) {
    assert_eq!(env.response.take().unwrap().status(), StatusCode::OK);
}
