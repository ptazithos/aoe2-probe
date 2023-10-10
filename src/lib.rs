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
pub mod utils;

mod scenario;
mod tweak;
mod platform;

pub use io::Source;
pub use scenario::{ExportFormat, Scenario};
pub use tweak::*;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn init(){
    platform::bind_console()
}
