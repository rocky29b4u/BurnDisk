//! BurnDisk core library.
//!
//! Pure-Rust disc-burning domain logic ported from the DiscBurner (C#) project:
//! disc types, file enumeration, multi-disc spanning planner, project persistence.

#[cfg(feature = "gui")]
pub mod gui;
pub mod drives;
pub mod models;
pub mod planner;
pub mod project;
pub mod scanner;

pub use models::*;
pub use planner::DiscSpanningPlanner;
pub use project::BurnProject;
