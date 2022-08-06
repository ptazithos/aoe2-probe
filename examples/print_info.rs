use aoe2_probe::Scenario;

fn main() {
    //This example scenario file contains several texts encoded by utf8-cn, enable this locate if you want to see more details.
    let scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario");

    //Print all information that is contained in this scenario file.
    //println!("{:?}", &scenario.versio);

    //Print the file header.
    println!("{:?}", &scenario.versio.try_map()["file_header"]);

    //Print the triggers.
    //println!("{:?}", &scenario.versio.try_map()["triggers"]);
}
