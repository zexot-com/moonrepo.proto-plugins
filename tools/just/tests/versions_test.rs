use proto_pdk_test_utils::*;

generate_resolve_versions_tests!("just-test", {
    "1.36" => "1.36.0",
    "1.36.0" => "1.36.0",
});

#[tokio::test(flavor = "multi_thread")]
async fn loads_versions_from_git() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("just-test").await;

    let output = plugin.load_versions(LoadVersionsInput::default()).await;

    assert!(!output.versions.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn sets_latest_alias() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("just-test").await;

    let output = plugin.load_versions(LoadVersionsInput::default()).await;

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}
