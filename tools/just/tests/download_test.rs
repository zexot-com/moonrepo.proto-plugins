use proto_pdk_test_utils::*;

generate_download_install_tests!("just-test", "1.36.0");

#[tokio::test(flavor = "multi_thread")]
async fn supports_linux_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("just-test", |config| {
            config.host(HostOS::Linux, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.36.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("just".into()),
            checksum_url: Some("https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-aarch64-unknown-linux-musl.tar.gz.sha256sum".into()),
            download_name: Some("just-1.36.0-aarch64-unknown-linux-musl.tar.gz".into()),
            download_url: "https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-aarch64-unknown-linux-musl.tar.gz".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_linux_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("just-test", |config| {
            config.host(HostOS::Linux, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.36.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("just".into()),
            checksum_url: Some("https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-x86_64-unknown-linux-musl.tar.gz.sha256sum".into()),
            download_name: Some("just-1.36.0-x86_64-unknown-linux-musl.tar.gz".into()),
            download_url: "https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-x86_64-unknown-linux-musl.tar.gz".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_macos_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("just-test", |config| {
            config.host(HostOS::MacOS, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.36.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("just".into()),
            checksum_url: Some("https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-aarch64-apple-darwin.tar.gz.sha256sum".into()),
            download_name: Some("just-1.36.0-aarch64-apple-darwin.tar.gz".into()),
            download_url: "https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-aarch64-apple-darwin.tar.gz".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_macos_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("just-test", |config| {
            config.host(HostOS::MacOS, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.36.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("just".into()),
            checksum_url: Some("https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-x86_64-apple-darwin.tar.gz.sha256sum".into()),
            download_name: Some("just-1.36.0-x86_64-apple-darwin.tar.gz".into()),
            download_url: "https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-x86_64-apple-darwin.tar.gz".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_windows_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("just-test", |config| {
            config.host(HostOS::Windows, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.36.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("just".into()),
            checksum_url: Some("https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-aarch64-pc-windows-msvc.zip.sha256sum".into()),
            download_name: Some("just-1.36.0-aarch64-pc-windows-msvc.zip".into()),
            download_url: "https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-aarch64-pc-windows-msvc.zip".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_windows_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("just-test", |config| {
            config.host(HostOS::Windows, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.36.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("just".into()),
            checksum_url: Some("https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-x86_64-pc-windows-msvc.zip.sha256sum".into()),
            download_name: Some("just-1.36.0-x86_64-pc-windows-msvc.zip".into()),
            download_url: "https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-x86_64-pc-windows-msvc.zip".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn locates_unix_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("just-test", |config| {
            config.host(HostOS::Linux, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.36.0").unwrap(),
                    ..Default::default()
                },
            })
            .await
            .exes
            .get("just")
            .unwrap()
            .exe_path,
        Some("just".into())
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn locates_windows_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("just-test", |config| {
            config.host(HostOS::Windows, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.36.0").unwrap(),
                    ..Default::default()
                },
            })
            .await
            .exes
            .get("just")
            .unwrap()
            .exe_path,
        Some("just.exe".into())
    );
}
