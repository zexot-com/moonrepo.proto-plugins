# Just plugin

[Just](https://just.systems/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

This plugin is built-in to proto, but if you want to override it with an explicit version, add the following to `.prototools`.

```toml
[plugins]
just = "https://github.com/moonrepo/tools/releases/download/just_tool-vX.Y.Z/just_tool.wasm"
```

## Configuration

Just plugin can be configured with a `.prototools` file.

- `dist-url` (string) - The distribution URL to download Just archives from. Supports `{version}` and `{file}` tokens.

```toml
[tools.just]
dist-url = "https://..."
```

## Hooks

Just plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install just-test
proto list-remote just-test
```
