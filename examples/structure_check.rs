use aoe2_probe::{parse::Censor, ver1_46::Versio, Scenario};

fn main() {
    let scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario");
    let versio = &scenario.versio;
    let mut template = Versio::template();

    template.try_mut_map().update("file_header", 1_u32);

    let layer_1 = Censor::is_template(versio, &template, 1);
    let layer_2 = Censor::is_template(versio, &template, 2);

    println!(
        "Does versio equal to the template? at layer_1: {},  at layer_2: {}",
        layer_1, layer_2
    );
}
