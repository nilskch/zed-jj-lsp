# zed-jj-lsp

This is the Zed extension for the [jj-lsp](https://github.com/nilskch/jj-lsp).

## How it works

### `jj-lsp` is installed in `PATH`

The extension will automatically detect if `jj-lsp` is installed in your PATH and use it for the LSP
if it is available.

### `jj-lsp` is not installed in `PATH`

The extension will fetch the [latest release](https://github.com/nilskch/jj-lsp/releases) of the
`jj-lsp` and cache it locally in the Zed extension directory. It will clean up old versions of the
lsp when a new version is downloaded.

## License

This project is licensed under the [MIT License](LICENSE).
