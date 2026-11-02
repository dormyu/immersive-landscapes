//! Deterministic regional terrain and drainage prototype by dormyu.
//!
//! This is a generator research core, not yet a drop-in Horizon mod. Drainage is
//! compiled once per region; it is not recomputed per chunk or render frame.
pub mod drainage;
pub mod sampling;
pub mod terrain;

pub use drainage::{Basin, Cell, Delta, RiverSegment};
pub use sampling::{Landscape, Surface};
pub use terrain::{Climate, Config, HeightField};
