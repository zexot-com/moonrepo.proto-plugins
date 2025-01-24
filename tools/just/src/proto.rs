use crate::config::JustPluginConfig;
use extism_pdk::*;
use proto_pdk::*;
use schematic::SchemaBuilder;
use std::collections::HashMap;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

static NAME: &str = "Just";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::CommandLine,
        config_schema: Some(SchemaBuilder::build_root::<JustPluginConfig>()),
        minimum_proto_version: Some(Version::new(0, 42, 0)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/casey/just")?
        .into_iter()
        .filter_map(|tag| {
            if tag.starts_with("v") {
                None
            } else {
                Some(tag)
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64, HostArch::Arm64, HostArch::Arm],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64, HostArch::X86, HostArch::Arm64],
        ],
    )?;

    let version = &input.context.version;
    let host = get_tool_config::<JustPluginConfig>()?.dist_url;

    let arch = match env.arch {
        HostArch::Arm => "arm",
        HostArch::Arm64 => "aarch64",
        HostArch::X64 => "x86_64",
        HostArch::X86 => "x86_64",
        _ => unreachable!(),
    };

    if version.is_canary() {
        return Err(plugin_err!(PluginError::UnsupportedCanary {
            tool: NAME.into()
        }));
    }

    let prefix = match env.os {
        HostOS::Linux => format!("just-{version}-{arch}-unknown-linux-musl"),
        HostOS::MacOS => format!("just-{version}-{arch}-apple-darwin"),
        HostOS::Windows => format!("just-{version}-{arch}-pc-windows-msvc"),
        _ => unreachable!(),
    };

    let filename = if env.os.is_windows() {
        format!("{prefix}.zip")
    } else {
        format!("{prefix}.tar.gz")
    };

    let version_spec = VersionSpec::parse("1.37.0").expect("Failed to parse version spec");

    let checksum_file = if version >= &version_spec {
        "SHA256SUMS".to_string()
    } else {
        format!("{filename}.sha256sum")
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(prefix),
        download_url: host
            .replace("{version}", &version.to_string())
            .replace("{file}", &filename),
        download_name: Some(filename),
        checksum_url: Some(
            host.replace("{version}", &version.to_string())
                .replace("{file}", &checksum_file),
        ),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([(
            "just".into(),
            ExecutableConfig::new_primary(if env.os.is_windows() {
                "just.exe"
            } else {
                "just"
            }),
        )]),
        exes_dir: Some(".".into()),
        ..LocateExecutablesOutput::default()
    }))
}
