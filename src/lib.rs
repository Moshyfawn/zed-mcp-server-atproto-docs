use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

struct AtprotoDocsMcpExtension;

impl zed::Extension for AtprotoDocsMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        if zed::npm_package_installed_version("mcp-remote")?.is_none() {
            zed::npm_install_package("mcp-remote", "latest")?;
        }

        Ok(Command {
            command: "node_modules/.bin/mcp-remote".to_string(),
            args: vec!["https://mcp-atproto-docs.immber.workers.dev/sse".to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(AtprotoDocsMcpExtension);
