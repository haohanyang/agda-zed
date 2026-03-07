use zed_extension_api::{self as zed, Result, settings::LspSettings};

struct AgdaExtension;

impl zed::Extension for AgdaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {

        let settings = LspSettings::for_worktree("als", worktree)?;

        if let Some(binary) = settings.binary {
            return Ok(
                zed::Command {
                    command: binary.path.expect("Error in opening binary for Agda LSP"),
                    args: vec![],
                    env: vec![]
                }
            )
        }

        // get where als is located
        let als_path = worktree
            .which("als")
            .ok_or_else(|| format!(
                "Agda Language Server has not been found. Install it via
                cabal and set 'binary' settings.json or add it to your PATH"
            ))?;


        return Ok(
            zed::Command {
                command: als_path,
                args: vec![],
                env: vec![],
            }
        )
    }
}

zed::register_extension!(AgdaExtension);
