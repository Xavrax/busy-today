use cucumber_rust::{async_trait, World, WorldInit};
use cucumber_rust::{given, then, when};
use std::convert::Infallible;

pub mod days;

#[derive(WorldInit)]
pub struct ScenarioEnvironment {}

#[async_trait(?Send)]
impl World for ScenarioEnvironment {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

#[given(regex = "configured (.*) endpoint")]
async fn configure_endpoint(env: &mut ScenarioEnvironment, endpoint: String) {}

#[when("request is sent")]
async fn send_request(env: &mut ScenarioEnvironment) {}

#[then("response is valid")]
fn check_request(env: &mut ScenarioEnvironment) {}
