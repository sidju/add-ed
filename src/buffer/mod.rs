// Common imports for buffer components
use std::fmt::Debug;
use crate::history::Snapshot;

/// We export the [`Buffer`] iterators in their own module, as most users won't
/// need them.
pub mod iters;
use iters::*;

// The other modules we keep private and export their contents directly

mod line;
pub use line::*;

mod buffer;
pub use buffer::*;
