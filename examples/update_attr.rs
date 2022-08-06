use aoe2_probe::Scenario;

fn main() {
    //This example scenario file contains several texts encoded by utf8-cn, enable this locate if you want to see more details.
    let mut scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario");

    let file_header = scenario.versio.get_by_path("/file_header");
    println!("THe original file header: {:?}", file_header);

    let author = scenario.versio.get_by_path_mut("/file_header/creator_name");
    author.try_mut_str32().set_content("Arian");

    let file_header = scenario.versio.get_by_path("/file_header");
    println!("THe original file header: {:?}", file_header);
}
