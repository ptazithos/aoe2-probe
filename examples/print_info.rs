use aoe2_probe::scenorio::Scenorio;

fn main() {
    //This example scenario file contains several texts encoded by utf8-cn, enable this locate if you want to see more details.
    let scenorio = Scenorio::from_file("./resources/chapter_1.aoe2scenario");

    //Print all information that is contained in this scenorio file.
    //println!("{:?}", &scenorio.versio);

    //Print the file header.
    println!("{:?}", &scenorio.versio.try_map()["file_header"]);

    //Print the triggers.
    //println!("{:?}", &scenorio.versio.try_map()["triggers"]);
}
