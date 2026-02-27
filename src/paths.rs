use std::path::PathBuf;

/// Base Claude directory (~/.claude or PRAXIS_ECHO_HOME override).
pub fn claude_dir() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("PRAXIS_ECHO_HOME") {
        return Ok(PathBuf::from(p));
    }
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    Ok(home.join(".claude"))
}

/// Home directory for documents (~/ or PRAXIS_ECHO_DOCS override).
pub fn docs_dir() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("PRAXIS_ECHO_DOCS") {
        return Ok(PathBuf::from(p));
    }
    dirs::home_dir().ok_or("Could not determine home directory".to_string())
}

pub fn praxis_dir() -> Result<PathBuf, String> {
    Ok(claude_dir()?.join("praxis"))
}

pub fn state_file() -> Result<PathBuf, String> {
    Ok(praxis_dir()?.join("state.json"))
}

pub fn checkpoints_dir() -> Result<PathBuf, String> {
    Ok(praxis_dir()?.join("checkpoints"))
}

pub fn settings_file() -> Result<PathBuf, String> {
    Ok(claude_dir()?.join("settings.json"))
}

pub fn rules_dir() -> Result<PathBuf, String> {
    Ok(claude_dir()?.join("rules"))
}

pub fn protocol_file() -> Result<PathBuf, String> {
    Ok(rules_dir()?.join("praxis-echo.md"))
}

// Document paths
pub fn learning_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("LEARNING.md"))
}

pub fn thoughts_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("THOUGHTS.md"))
}

pub fn curiosity_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("CURIOSITY.md"))
}

pub fn reflections_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("REFLECTIONS.md"))
}

pub fn praxis_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("PRAXIS.md"))
}

pub fn self_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("SELF.md"))
}

pub fn session_log_file() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("SESSION-LOG.md"))
}

// Archive directories
pub fn archives_dir() -> Result<PathBuf, String> {
    Ok(docs_dir()?.join("archives"))
}

// Intent queue
pub fn intent_queue_file() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("PRAXIS_ECHO_QUEUE") {
        return Ok(PathBuf::from(p));
    }
    Ok(docs_dir()?.join("intent-queue.json"))
}
