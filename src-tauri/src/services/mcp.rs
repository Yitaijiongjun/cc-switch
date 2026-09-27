use indexmap::IndexMap;
use std::collections::HashMap;

use crate::app_config::{AppType, McpServer};
use crate::error::AppError;
use crate::store::AppState;

/// MCP management is intentionally disabled in this fork.
///
/// CC Switch must not discover, persist, inject, remove, or reconcile MCP
/// servers. MCP configuration is owned exclusively by each upstream client.
pub struct McpService;

impl McpService {
    pub const BUILTIN_MANAGEMENT_DISABLED: bool = true;

    pub fn is_builtin_management_disabled() -> bool {
        Self::BUILTIN_MANAGEMENT_DISABLED
    }

    pub fn get_all_servers(_state: &AppState) -> Result<IndexMap<String, McpServer>, AppError> {
        Ok(IndexMap::new())
    }

    pub fn upsert_server(_state: &AppState, _server: McpServer) -> Result<(), AppError> {
        Ok(())
    }

    pub fn delete_server(_state: &AppState, _id: &str) -> Result<bool, AppError> {
        Ok(false)
    }

    pub fn toggle_app(
        _state: &AppState,
        _server_id: &str,
        _app: AppType,
        _enabled: bool,
    ) -> Result<(), AppError> {
        Ok(())
    }

    pub fn sync_all_enabled(_state: &AppState) -> Result<(), AppError> {
        Ok(())
    }

    pub fn sync_enabled_for_app(_state: &AppState, _app: &AppType) -> Result<(), AppError> {
        Ok(())
    }

    #[deprecated(since = "3.7.0", note = "Built-in MCP management is disabled")]
    pub fn get_servers(
        _state: &AppState,
        _app: AppType,
    ) -> Result<HashMap<String, serde_json::Value>, AppError> {
        Ok(HashMap::new())
    }

    #[deprecated(since = "3.7.0", note = "Built-in MCP management is disabled")]
    pub fn set_enabled(
        _state: &AppState,
        _app: AppType,
        _id: &str,
        _enabled: bool,
    ) -> Result<bool, AppError> {
        Ok(false)
    }

    #[deprecated(since = "3.7.0", note = "Built-in MCP management is disabled")]
    pub fn sync_enabled(_state: &AppState, _app: AppType) -> Result<(), AppError> {
        Ok(())
    }

    pub fn import_from_claude(_state: &AppState) -> Result<usize, AppError> {
        Ok(0)
    }

    pub fn import_from_codex(_state: &AppState) -> Result<usize, AppError> {
        Ok(0)
    }

    pub fn import_from_gemini(_state: &AppState) -> Result<usize, AppError> {
        Ok(0)
    }

    pub fn import_from_grokbuild(_state: &AppState) -> Result<usize, AppError> {
        Ok(0)
    }

    pub fn import_from_opencode(_state: &AppState) -> Result<usize, AppError> {
        Ok(0)
    }

    pub fn import_from_hermes(_state: &AppState) -> Result<usize, AppError> {
        Ok(0)
    }

    pub fn import_from_all_apps(_state: &AppState) -> Result<usize, AppError> {
        Ok(0)
    }
}
