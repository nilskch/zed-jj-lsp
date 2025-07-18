use zed_extension_api as zed;

const JJ_LSP_COMMAND: &str = "jj-lsp";
const JJ_LSP_REPO: &str = "nilskch/jj-lsp";

struct JJ;

impl zed::Extension for JJ {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let command = match worktree.which(JJ_LSP_COMMAND) {
            Some(path) => path,
            None => get_cached_or_download(language_server_id)?,
        };

        Ok(zed::Command {
            command,
            args: vec![],
            env: vec![],
        })
    }
}

fn get_cached_or_download(language_server_id: &zed::LanguageServerId) -> zed::Result<String> {
    let release = zed::latest_github_release(
        JJ_LSP_REPO,
        zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;

    let (os, arch) = zed::current_platform();

    let downloaded_binary_name = match &os {
        zed::Os::Windows => format!("{JJ_LSP_COMMAND}.exe"),
        zed::Os::Mac | zed::Os::Linux => JJ_LSP_COMMAND.to_string(),
    };
    let cached_binary_name = format!("{}-{downloaded_binary_name}", release.version);

    if std::env::current_dir()
        .map_err(|err| format!("Failed to get current directory: {err}"))?
        .join(&cached_binary_name)
        .is_file()
    {
        // cache hit, return the cached binary name
        return Ok(cached_binary_name);
    }

    // cache miss, delete all cached entries and download the asset
    delete_all_current_dir()?;

    download_release_asset(
        language_server_id,
        &os,
        &arch,
        &release,
        &downloaded_binary_name,
        &cached_binary_name,
    )?;

    Ok(cached_binary_name)
}

fn delete_all_current_dir() -> zed::Result<()> {
    let current_directory =
        std::env::current_dir().map_err(|err| format!("Failed to get current directory: {err}"))?;

    let entries = current_directory
        .read_dir()
        .map_err(|err| format!("Failed to read directory: {err}"))?;

    for entry in entries {
        let path = entry
            .map_err(|err| format!("Failed to get entry of current directory: {err}"))?
            .path();

        if path.is_dir() {
            std::fs::remove_dir_all(path)
                .map_err(|err| format!("Failed to remove directory: {err}"))?;
        } else {
            std::fs::remove_file(path).map_err(|err| format!("Failed to remove file: {err}"))?;
        }
    }

    Ok(())
}

fn get_release_asset_name(os: &zed::Os, arch: &zed::Architecture) -> zed::Result<String> {
    let asset_name = format!(
        "{JJ_LSP_COMMAND}-{arch}-{os}.{extension}",
        arch = match arch {
            zed::Architecture::Aarch64 => "aarch64",
            zed::Architecture::X8664 => "x86_64",
            _ => return Err(format!("unsupported architecture: {arch:?}")),
        },
        os = match os {
            zed::Os::Mac => "apple-darwin",
            zed::Os::Linux => "unknown-linux-gnu",
            zed::Os::Windows => "pc-windows-msvc",
        },
        extension = match os {
            zed::Os::Mac | zed::Os::Linux => "tar.gz",
            zed::Os::Windows => "zip",
        }
    );

    Ok(asset_name)
}

fn download_release_asset(
    language_server_id: &zed::LanguageServerId,
    os: &zed::Os,
    arch: &zed::Architecture,
    release: &zed::GithubRelease,
    downloaded_binary_name: &str,
    cached_binary_name: &str,
) -> zed::Result<()> {
    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
    );

    let asset_name = get_release_asset_name(os, arch)?;
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == asset_name)
        .ok_or_else(|| format!("No asset '{asset_name}' found in latest GitHub release"))?;

    zed::download_file(
        &asset.download_url,
        ".",
        match os {
            zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
            zed::Os::Windows => zed::DownloadedFileType::Zip,
        },
    )
    .map_err(|err| format!("failed to download file: {err}"))?;

    std::fs::rename(downloaded_binary_name, cached_binary_name)
        .map_err(|err| format!("Failed to rename lsp bianry: {err}"))?;

    zed::make_file_executable(cached_binary_name)
        .map_err(|err| format!("Failed to make lsp binary executable: {err}"))?;

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::None,
    );

    Ok(())
}

zed::register_extension!(JJ);
