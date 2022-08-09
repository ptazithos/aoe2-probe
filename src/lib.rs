//! A rust library for editing aoe2scenario files from AoE2 DE.
//!
//! # Examples
//!
//! ```
//! use aoe2_probe::Scenario;
//!
//! let scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
//! println!("{:?}", &scenario.versio);
//! scenario.to_file("./resources/temp.aoe2scenario");
//!
//! ```

pub mod io;
pub mod parse;
mod prebuilt;
mod proxy;
mod scenario;

pub mod utils;
pub use io::Source;
pub use prebuilt::ver1_46;
pub use prebuilt::ver1_47;
pub use scenario::Scenario;
