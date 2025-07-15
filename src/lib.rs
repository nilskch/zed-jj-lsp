use zed_extension_api as zed;

const JJ_LSP_COMMAND: &str = "jj-lsp";

struct JJ;

impl zed::Extension for JJ {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let command = match worktree.which(JJ_LSP_COMMAND) {
            Some(path) => path,
            None => return Err(format!("'{JJ_LSP_COMMAND}' is not in PATH")),
        };

        Ok(zed::Command {
            command,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(JJ);
