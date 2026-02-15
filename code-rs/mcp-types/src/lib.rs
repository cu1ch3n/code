//! Re-export the upstream MCP types so the fork stays in lockstep with codex-rs.
pub use upstream_mcp_types::*;

/// Preferred MCP schema version for new handshakes.
pub const MCP_LATEST_SCHEMA_VERSION: &str = "2025-11-25";

/// MCP schema versions this workspace accepts from servers.
pub const MCP_SUPPORTED_SCHEMA_VERSIONS: &[&str] =
    &[MCP_LATEST_SCHEMA_VERSION, MCP_SCHEMA_VERSION];

pub fn is_supported_schema_version(version: &str) -> bool {
    MCP_SUPPORTED_SCHEMA_VERSIONS.contains(&version)
}
