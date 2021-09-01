use crate::dsl::ScenarioEnvironment;
use cucumber_rust::{given, then};

#[given("mocked storage provider with empty store")]
fn create_storage_provider_with_empty_store(_env: &mut ScenarioEnvironment) {}

#[given("mocked storage provider with some days in store")]
fn create_storage_provider_with_some_days_in_store(_env: &mut ScenarioEnvironment) {}

#[then("body contains empty days list")]
fn check_if_body_is_empty(_env: &mut ScenarioEnvironment) {}

#[then("body contains days list")]
fn check_if_body_contains_some_days(_env: &mut ScenarioEnvironment) {}
