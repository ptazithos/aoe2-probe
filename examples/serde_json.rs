use aoe2_probe::Scenario;

fn main() {
    let scenario = Scenario::from_file("./resources/chapter_3.aoe2scenario");
    let json = serde_json::to_string(&scenario.versio).unwrap();
    println!("{}", json);
}
