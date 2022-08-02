# **A much much better new implementation will come soon! Don't use main branch at this moment please.**
## What is aoe2-probe?
This is a rust library for editing aoe2scenorio files from AoE2 DE.

## WARNING
* aoe2-probe is currently in the very early stage. APIs may change dramatically in following updates. Please don't use this project in serious development.
* Due to the zlib implementation difference,  the exported file cannot be as same as the imported file, while the content of files is constant(Don't worry, AoE2 DE still understand it).Backing up the original is always recommended.

## Design Goals
* Full ability to access every bit in aoe2scenorio files.
* Editing every bit with a reliable correctness check.
* Provides constant API compatibility across game versions.

## Getting Started
Under directory **./examples/**, you can find several simple showcases.
```rust
use aoe2_probe::scenorio::Scenorio;
//Reading scenorio content from the .aoe2scenorio file
let scenorio = Scenorio::from_file("./resources/chapter_1.aoe2scenario");
//saving content to a new .aoe2scenorio file
scenorio.to_file("./temp/temp.aoe2scenario");
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


