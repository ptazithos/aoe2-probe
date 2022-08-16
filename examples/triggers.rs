use aoe2_probe::{prebuilt::ver1_46::Trigger, ExportFormat, Scenario, TriggersTweak};

fn main() {
    let mut scenario = Scenario::from_file("./resources/disorder_2.aoe2scenario").unwrap();
    let trigger = Trigger::default();
    TriggersTweak::push(&mut scenario, trigger).unwrap();

    //TriggersTweak::sort_by_display_order(&mut scenario).unwrap();

    scenario
        .to_file("./resources/temp.aoe2scenario", ExportFormat::AoE2Scenario)
        .unwrap();
}
