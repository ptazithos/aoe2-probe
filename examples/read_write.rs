use aoe2_probe::{ExportFormat, Scenario};

fn main() {
    let scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    println!("{:?}", &scenario.versio);
    scenario
        .to_file("./resources/temp.aoe2scenario", ExportFormat::AoE2Scenario)
        .unwrap();
}
