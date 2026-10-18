//! Placement spec: TOML schema and parsing for per-crate placement contracts.
//!
//! A spec lives next to `Cargo.toml` (default file name `placement.toml`). It
//! describes the crate boundary, the paths that may hold code, and the
//! responsibility of each source file. All human-readable values are English:
//! the placement rules are consumed by agents, and mixed-language contracts
//! make matching and review harder.

use std::path::Path;

use serde::Deserialize;

pub const SCHEMA_VERSION: u32 = 1;
pub const DEFAULT_FILES_SCOPE: &[&str] = &["src/**", "tests/**"];

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlacementSpec {
    pub schema: u32,
    #[serde(rename = "crate")]
    pub crate_name: String,
    pub responsibility: String,
    #[serde(default)]
    pub belongs: Vec<String>,
    #[serde(default)]
    pub does_not_belong: Vec<Redirect>,
    /// Files that must be covered by a `[[files]]` entry. `None` means the
    /// default (`src/**` and `tests/**`); an empty list disables file checks.
    #[serde(default)]
    pub files_scope: Option<Vec<String>>,
    #[serde(default)]
    pub files: Vec<FileRule>,
    #[serde(default)]
    pub paths: Paths,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Redirect {
    pub concern: String,
    pub to: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileRule {
    pub path: String,
    pub responsibility: String,
    #[serde(default)]
    pub entry: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Paths {
    #[serde(default)]
    pub allow: Vec<String>,
    #[serde(default)]
    pub deny: Vec<String>,
}

impl PlacementSpec {
    /// File globs that must have a responsibility entry. `None` means the
    /// default scope; `Some([])` disables the file-level check.
    pub fn effective_files_scope(&self) -> Vec<String> {
        match &self.files_scope {
            Some(scope) => scope.clone(),
            None => DEFAULT_FILES_SCOPE
                .iter()
                .map(|pattern| (*pattern).to_string())
                .collect(),
        }
    }
}

/// A parsed spec plus the raw facts that are not part of the typed model.
#[derive(Debug)]
pub struct ParsedSpec {
    pub spec: PlacementSpec,
    /// First line containing CJK characters; `None` when the file is English.
    pub cjk_line: Option<u32>,
}

/// Fatal spec problems. Validation problems that still allow the spec to be
/// displayed are recorded by the catalog instead of aborting the load.
#[derive(Debug)]
pub enum SpecError {
    Missing {
        path: String,
    },
    Io {
        path: String,
        message: String,
    },
    Parse {
        path: String,
        line: u32,
        message: String,
    },
    Schema {
        path: String,
        found: u32,
    },
    CrateEmpty {
        path: String,
    },
}

impl std::fmt::Display for SpecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing { path } => write!(f, "missing placement spec: {path}"),
            Self::Io { path, message } => write!(f, "cannot read {path}: {message}"),
            Self::Parse {
                path,
                line,
                message,
            } => {
                write!(f, "invalid placement spec at {path}:{line}: {message}")
            }
            Self::Schema { path, found } => write!(
                f,
                "unsupported placement spec schema {found} at {path}; expected {SCHEMA_VERSION}"
            ),
            Self::CrateEmpty { path } => write!(f, "empty crate name at {path}"),
        }
    }
}

impl std::error::Error for SpecError {}

/// Read and parse one spec file. The crate name is checked by the catalog so a
/// mismatch can still be displayed; only malformed TOML and an unsupported
/// schema abort the load.
pub fn load(root: &Path, relative: &Path) -> Result<ParsedSpec, SpecError> {
    let path = root.join(relative);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(SpecError::Missing {
                path: relative.display().to_string(),
            });
        }
        Err(error) => {
            return Err(SpecError::Io {
                path: relative.display().to_string(),
                message: error.to_string(),
            });
        }
    };

    let cjk_line = first_cjk_line(&text);
    let spec: PlacementSpec = toml::from_str(&text).map_err(|error| SpecError::Parse {
        path: relative.display().to_string(),
        line: error_line(&text, &error),
        message: error.to_string(),
    })?;

    if spec.schema != SCHEMA_VERSION {
        return Err(SpecError::Schema {
            path: relative.display().to_string(),
            found: spec.schema,
        });
    }
    if spec.crate_name.trim().is_empty() {
        return Err(SpecError::CrateEmpty {
            path: relative.display().to_string(),
        });
    }

    Ok(ParsedSpec { spec, cjk_line })
}

fn error_line(text: &str, error: &toml::de::Error) -> u32 {
    let Some(span) = error.span() else {
        return 0;
    };
    let prefix = text.get(..span.start).unwrap_or("");
    u32::try_from(prefix.lines().count().max(1)).unwrap_or(u32::MAX)
}

/// First line containing a CJK character or CJK punctuation.
pub fn first_cjk_line(text: &str) -> Option<u32> {
    for (index, line) in text.lines().enumerate() {
        if line.chars().any(is_cjk) {
            return Some(u32::try_from(index + 1).unwrap_or(u32::MAX));
        }
    }
    None
}

fn is_cjk(character: char) -> bool {
    matches!(
        character as u32,
        0x3000..=0x303F // CJK punctuation
            | 0x3040..=0x30FF // kana
            | 0x3400..=0x4DBF // CJK extension A
            | 0x4E00..=0x9FFF // CJK unified ideographs
            | 0xAC00..=0xD7AF // hangul syllables
            | 0xF900..=0xFAFF // CJK compatibility ideographs
            | 0xFF00..=0xFFEF // fullwidth forms
    )
}
