//! Historical terrain research helpers for Immersive Landscapes, by dormyu.
//!
//! The playable Horizon mod lives in `mixins/immersive_landscapes`. These older
//! height-field experiments do not drive its installed vanilla-based terrain.
//! The actual native watershed module is also exercised by the integration tests.
pub mod drainage;
pub mod sampling;
pub mod terrain;

pub use drainage::{Basin, Cell, Delta, RiverSegment};
pub use sampling::{Landscape, Surface};
pub use terrain::{Climate, Config, HeightField};
