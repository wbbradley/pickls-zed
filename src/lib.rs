#[macro_use]
extern crate zed_extension_api;
use zed_extension_api::{Command, Extension, LanguageServerId, Result, Worktree};

struct PicklsExtension {}

impl Extension for PicklsExtension {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        log::info!(
            "[pickls] fetching language server command for {}",
            language_server_id
        );
        Ok(Command {
            command: String::from("pickls"),
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

register_extension!(PicklsExtension);
