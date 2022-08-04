[![CircleCI](https://img.shields.io/circleci/build/github/ptazithos/aoe2-probe/main)](https://dl.circleci.com/status-badge/redirect/gh/ptazithos/aoe2-probe/tree/main)
[![Crates.io](https://img.shields.io/crates/v/aoe2-probe.svg)](https://crates.io/crates/aoe2-probe)
[![GPL-3.0 licensed](https://img.shields.io/badge/license-GPL-blue.svg)](./LICENSE)
## What is aoe2-probe?
This is a rust library for editing aoe2scenorio files from AoE2 DE.

## WARNING
* aoe2-probe is currently in the very early stage. APIs may change dramatically in following updates. Please don't use this project in serious development.
* Due to the zlib implementation difference,  the exported file cannot be as same as the imported file, while the content of files is constant(Don't worry, AoE2 DE still understand it).Backing up the original is always recommended.

## Design Goals
* Full ability to access every byte in aoe2scenorio files.
* Editing every bit with a reliable correctness check. todo!()
* Provides constant API compatibility across game versions. todo!()

## Getting Started
Under the directory **./examples/**, you can find several simple showcases.

**Import and export files:**
```rust
use aoe2_probe::scenorio::Scenorio;

//Reading scenorio content from the .aoe2scenorio file
let scenorio = Scenorio::from_file("./resources/chapter_1.aoe2scenario");
//saving content to a new .aoe2scenorio file
scenorio.to_file("./resources/temp.aoe2scenario");
```

**Update attributes:**
```rust
use aoe2_probe::scenorio::Scenorio;

//versio's structure definition can be found in the folder /src/prebuilt/ver1_46/versio.rs
let mut scenorio = Scenorio::from_file("./resources/chapter_1.aoe2scenario");
let file_header = scenorio.versio.try_mut_map()["file_header"].try_mut_map();
let creator = file_header["creator_name"].try_mut_str32();
//Update the creator name.
creator.set_content("Arian");
```

**customize a structure:**
```rust
//Define a socre record
let mut root = LinkedHashMap::new();
root.push_back("score", Token::Int16(100));
root.push_back("name", Token::Str32(DynString::with_capacity(12, "anonymous")));

//serialize
root.to_le_vec();

//deserialize
root.from_le_vec();
```

Run exampes with the following command:
```shell
cargo run --example read_write
```
Every member of **versio** and itself implements fmt::Debug trait. Print them if you want to know more.
```rust
let scenorio = Scenorio::from_file("./resources/chapter_1.aoe2scenario");
println!("{:?}", &scenorio.versio())
```
## Supports
|Version|Support|
|----|----|
|ver.1.46|Support|

Currently, only version 1.46 and newer will be firstly supported.

## Libraries Used
* [miniz_oxide](https://github.com/Frommi/miniz_oxide): Pure rust Rust replacement for the miniz deflate/zlib encoder/decoder using no unsafe code.
* [log](https://github.com/rust-lang/log): A lightweight logging facade.
* [env_logger](https://github.com/env-logger-rs/env_logger/): Implements a logger that can be configured via environment variables.

## Acknowledgment
This library is inpired by [AoE2ScenarioParser](https://github.com/KSneijders/AoE2ScenarioParser) and [Trigger Craft](https://github.com/MegaDusknoir/AoE2TriggerCraft)


