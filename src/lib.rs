//! praxis-echo — Pipeline enforcement engine for AI self-evolution.
//!
//! Tracks document pipeline health (LEARNING → THOUGHTS → REFLECTIONS → SELF/PRAXIS),
//! enforces thresholds, detects stale items, and provides session-level diffs.
//! Designed for integration with echo-system as a core plugin.

pub mod archive;
pub mod checkpoint;
pub mod init;
pub mod nudge;
pub mod parser;
pub mod paths;
pub mod pulse;
pub mod review;
pub mod runtime;
pub mod scan;
pub mod state;
pub mod status;

use std::any::Any;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use echo_system_types::plugin::{Plugin, PluginContext, PluginResult, PluginRole};
use echo_system_types::{HealthStatus, PluginMeta, SetupPrompt};

/// The praxis-echo plugin. Manages document pipeline enforcement.
pub struct PraxisEcho {
    claude_dir: PathBuf,
    docs_dir: PathBuf,
}

impl PraxisEcho {
    /// Create a new PraxisEcho with specific directories.
    ///
    /// `claude_dir` is where config/state lives (e.g. ~/.claude).
    /// `docs_dir` is where identity documents live (e.g. ~/).
    pub fn new(claude_dir: PathBuf, docs_dir: PathBuf) -> Self {
        Self {
            claude_dir,
            docs_dir,
        }
    }

    /// Create a PraxisEcho using default path resolution
    /// (~/.claude or PRAXIS_ECHO_HOME, ~/ or PRAXIS_ECHO_DOCS).
    pub fn from_default() -> Result<Self, String> {
        Ok(Self::new(paths::claude_dir()?, paths::docs_dir()?))
    }

    /// Base directory for config and state.
    pub fn claude_dir(&self) -> &PathBuf {
        &self.claude_dir
    }

    /// Base directory for identity documents.
    pub fn docs_dir(&self) -> &PathBuf {
        &self.docs_dir
    }

    /// Report health status based on pipeline state.
    fn health_check(&self) -> HealthStatus {
        if !self.claude_dir.exists() {
            return HealthStatus::Down("config directory not found".into());
        }

        let praxis_dir = self.claude_dir.join("praxis");
        if !praxis_dir.exists() {
            return HealthStatus::Degraded("praxis state directory not found".into());
        }

        let state_file = praxis_dir.join("state.json");
        if !state_file.exists() {
            return HealthStatus::Degraded("state.json not found — run init".into());
        }

        // Check pipeline frozen status
        if let Ok(st) = state::load_from(&state_file) {
            if st.pipeline.frozen_session_count >= 3 {
                return HealthStatus::Degraded(format!(
                    "pipeline frozen for {} sessions",
                    st.pipeline.frozen_session_count
                ));
            }
        }

        HealthStatus::Healthy
    }

    /// Configuration prompts for the echo-system init wizard.
    fn get_setup_prompts() -> Vec<SetupPrompt> {
        vec![
            SetupPrompt {
                key: "claude_dir".into(),
                question: "Pipeline config directory:".into(),
                required: true,
                secret: false,
                default: Some("~/.claude".into()),
            },
            SetupPrompt {
                key: "docs_dir".into(),
                question: "Identity documents directory:".into(),
                required: true,
                secret: false,
                default: Some("~/".into()),
            },
        ]
    }
}

/// Factory function — creates a fully initialized plugin.
pub async fn create(
    config: &serde_json::Value,
    ctx: &PluginContext,
) -> Result<Box<dyn Plugin>, Box<dyn std::error::Error + Send + Sync>> {
    let claude_dir = config
        .get("claude_dir")
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| ctx.entity_root.join("monitoring"));

    let docs_dir = config
        .get("docs_dir")
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| ctx.entity_root.clone());

    Ok(Box::new(PraxisEcho::new(claude_dir, docs_dir)))
}

impl Plugin for PraxisEcho {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "praxis-echo".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            description: "Pipeline enforcement and behavioral policies".into(),
        }
    }

    fn role(&self) -> PluginRole {
        PluginRole::Pipeline
    }

    fn start(&mut self) -> PluginResult<'_> {
        Box::pin(async { Ok(()) })
    }

    fn stop(&mut self) -> PluginResult<'_> {
        Box::pin(async { Ok(()) })
    }

    fn health(&self) -> Pin<Box<dyn Future<Output = HealthStatus> + Send + '_>> {
        Box::pin(async move { self.health_check() })
    }

    fn setup_prompts(&self) -> Vec<SetupPrompt> {
        Self::get_setup_prompts()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
