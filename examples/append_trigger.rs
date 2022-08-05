use aoe2_probe::{ver1_46::Trigger, Scenorio};

fn main() {
    let mut scenorio = Scenorio::from_file("./resources/chapter_1.aoe2scenario");
    println!(
        "trigger count: {:?}",
        scenorio.versio.get_by_path("/file_header/trigger_count")
    );

    let mut proxy = scenorio.triggers_proxy();
    proxy.push(Trigger::template());

    println!(
        "trigger count: {:?}",
        scenorio.versio.get_by_path("/file_header/trigger_count")
    );
}
