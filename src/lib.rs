//! A rust library for editing aoe2scenario files from AoE2 DE.
//!
//! # Examples
//!
//! ```
//! use aoe2_probe::{Scenario, ExportFormat};
//!
//! let scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
//! println!("{:?}", &scenario.versio);
//! scenario.to_file("./resources/temp.aoe2scenario", ExportFormat::AoE2Scenario);
//!
//! ```

pub mod io;
pub mod parse;
pub mod prebuilt;
mod scenario;
mod tweak;

pub mod utils;
pub use io::Source;
pub use scenario::{ExportFormat, Scenario};
pub use tweak::*;
