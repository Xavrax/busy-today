use cucumber_rust::WorldInit;

mod dsl;

#[tokio::test]
async fn specification() {
    dsl::ScenarioEnvironment::init(&["./features"])
        .run_and_exit()
        .await;
}
