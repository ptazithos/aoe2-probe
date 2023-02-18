[![CircleCI](https://img.shields.io/circleci/build/github/ptazithos/aoe2-probe/main)](https://dl.circleci.com/status-badge/redirect/gh/ptazithos/aoe2-probe/tree/main)
[![Crates.io](https://img.shields.io/crates/v/aoe2-probe.svg)](https://crates.io/crates/aoe2-probe)
![Crates.io](https://img.shields.io/crates/d/aoe2-probe)
[![GPL-3.0 licensed](https://img.shields.io/badge/license-GPLv3-brightgreen.svg)](./LICENSE)

## What is aoe2-probe?
This is a rust library for editing aoe2scenario files from AoE2 DE.

## WARNING
* Due to the zlib implementation difference,  the exported file cannot be as same as the imported file, while the content of files is constant(Don't worry, AoE2 DE still understand it).Backing up the original is always recommended.

## Design Goals
* Full ability to access every byte in aoe2scenario files.
* Editing every bit with a reliable correctness check. todo!()
* Provides constant API compatibility across game versions.

## Getting Started
Under the directory **./examples/**, you can find several simple showcases.

**Import and export files:**
```rust
use aoe2_probe::scenario::{Scenario, ExportFormat};

//Reading scenario content from the .aoe2scenario file
let scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
//saving content to a new .aoe2scenario file
scenario.to_file("./resources/temp.aoe2scenario", ExportFormat::AoE2Scenario);
```

**Update attributes:**
```rust
use aoe2_probe::scenario::Scenario;

//versio's structure definition can be found in the folder /src/prebuilt/ver1_46/versio.rs
let mut scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();

let author = scenario.versio.get_by_path_mut("/file_header/creator_name");
//Update the creator name.
author.try_mut_str32().set_content("Arian");
```

**Customize a structure:**
```rust
//Define a score record
let mut root = PatchedMap::new();
root.push_back("score", Token::Int16(100));
root.push_back("name", Token::Str32(DynString::with_capacity(12, "anonymous")));

//serialize
root.to_le_vec();

//deserialize
root.from_le_vec();
```

**Serialize/Deserialize to any formats that serde support:**
```rust
let scenario = Scenario::from_file("./resources/chapter_3.aoe2scenario").unwrap();
let json = serde_json::to_string(&scenario.versio).unwrap();
println!("{}", json);
```

Run examples with the following command:
```shell
cargo run --example read_write
```
Every member of **versio** and itself implements fmt::Debug trait. Print them if you want to know more.
```rust
let scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
println!("{:?}", &scenario.versio())
```
Advanced examples can be found in [misc-probe](https://github.com/ptazithos/misc-probe).
Detailed information can be found in [Docs](https://docs.rs/aoe2-probe/latest/aoe2_probe/).
## Supports
|Version|Support|
|----|----|
|ver.1.46|Support|
|ver.1.47|Experimental|

Currently, only version 1.46 and newer will be firstly supported.

## Libraries Used
* [miniz_oxide](https://github.com/Frommi/miniz_oxide): Pure rust Rust replacement for the miniz deflate/zlib encoder/decoder using no unsafe code.
* [serde](https://github.com/serde-rs/serde): Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
* [linked-hash-map](https://github.com/contain-rs/linked-hash-map): A HashMap wrapper that holds key-value pairs in insertion order.
* [lazy-static](https://github.com/rust-lang-nursery/lazy-static.rs): A macro for declaring lazily evaluated statics in Rust.
* [log](https://github.com/rust-lang/log): A lightweight logging facade.


## Acknowledgment
This library is inspired by [AoE2ScenarioParser](https://github.com/KSneijders/AoE2ScenarioParser) and [Trigger Craft](https://github.com/MegaDusknoir/AoE2TriggerCraft)


