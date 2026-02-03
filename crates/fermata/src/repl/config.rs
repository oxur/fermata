//! REPL configuration management.
//!
//! Uses confyg for TOML-based configuration with support for:
//! - Config files in standard locations
//! - Environment variable overrides
//! - Sensible defaults

use std::path::PathBuf;

use confyg::Confygery;
use confyg::searchpath::Finder;
use serde::Deserialize;

use super::error::ReplResult;

/// Embedded default banner (compiled into the binary).
const EMBEDDED_BANNER: &str = include_str!("../../assets/repl/banner.txt");

/// REPL configuration.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ReplConfig {
    /// Path to the banner file (absolute or relative to search paths).
    pub banner_file: Option<String>,
}

impl ReplConfig {
    /// Load configuration from standard locations.
    ///
    /// Confyg searches for `fermata.toml` in configured paths.
    /// Environment variables can override with prefix `FERMATA_`.
    pub fn load() -> ReplResult<Self> {
        let conf_opts = confyg::conf::Options::with_project("fermata");
        let env_opts = confyg::env::Options::with_top_level("FERMATA");

        let result = Confygery::new().and_then(|mut c| {
            c.with_opts(conf_opts)?;
            c.add_file("fermata.toml")?;
            c.add_env(env_opts)?;
            c.build::<ReplConfig>()
        });

        match result {
            Ok(config) => Ok(config),
            Err(e) => {
                log::debug!("Config load error (using defaults): {}", e);
                Ok(ReplConfig::default())
            }
        }
    }

    /// Get the banner text.
    ///
    /// Reads from the configured banner file, or falls back to the embedded default.
    pub fn banner_text(&self) -> String {
        if let Some(ref banner_path) = self.banner_file {
            // Try absolute path first
            let path = PathBuf::from(banner_path);
            if path.is_absolute() {
                if let Ok(contents) = std::fs::read_to_string(&path) {
                    return contents;
                }
            }

            // Use confyg's Finder to search for the banner file
            if let Ok(found_path) = Finder::new().find(banner_path) {
                if let Ok(contents) = std::fs::read_to_string(&found_path) {
                    return contents;
                }
            }

            log::warn!("Banner file not found: {}", banner_path);
        }

        // Fall back to embedded banner
        EMBEDDED_BANNER.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ReplConfig::default();
        assert!(config.banner_file.is_none());
    }

    #[test]
    fn test_banner_text_fallback() {
        let config = ReplConfig::default();
        let banner = config.banner_text();
        // The ASCII art banner contains these distinctive characters
        assert!(banner.contains(".'|."));
        assert!(banner.contains(".||."));
    }

    #[test]
    fn test_load_returns_defaults_when_no_config() {
        // Should not error, just return defaults
        let config = ReplConfig::load().unwrap();
        assert!(config.banner_file.is_none());
    }
}
