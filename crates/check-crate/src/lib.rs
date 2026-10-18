//! Crate placement rules.
//!
//! A placement spec is a TOML contract stored next to `Cargo.toml`
//! (`placement.toml` by default). It records the crate-level boundary, the
//! file-level responsibilities, and the paths that may hold code. This crate
//! loads those specs, maps repository-relative paths to workspace packages,
//! and produces diagnostics for the canon gate and for the member-side
//! `xtask-crate` report.

pub mod catalog;
pub mod kind;
pub mod spec;

pub use catalog::{
    Assignment, Catalog, DiagnoseOptions, Diagnostic, LoadConfig, LoadError, LoadedSpec,
    SpecProblem,
};
pub use kind::CratePlacementFactory;
pub use spec::{FileRule, PlacementSpec, Redirect};

use check_core::kinds::KindFactory;
use check_core::rule::Rule;

/// No static rules: placement checks are parameterized by `checks.toml`.
pub fn rules() -> Vec<Box<dyn Rule>> {
    Vec::new()
}

/// Rule kinds provided by this crate.
pub fn factories() -> Vec<Box<dyn KindFactory>> {
    vec![Box::new(CratePlacementFactory)]
}
