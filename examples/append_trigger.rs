use aoe2_probe::{ver1_46::Trigger, Scenario};

fn main() {
    let mut scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    println!(
        "trigger count: {:?}",
        scenario.versio.get_by_path("/file_header/trigger_count")
    );

    let mut proxy = scenario.triggers_proxy();
    proxy.push(Trigger::template());

    println!(
        "trigger count: {:?}",
        scenario.versio.get_by_path("/file_header/trigger_count")
    );
}
