use aoe2_probe::Scenorio;

fn main() {
    //This example scenario file contains several texts encoded by utf8-cn, enable this locate if you want to see more details.
    let mut scenorio = Scenorio::from_file("./resources/chapter_1.aoe2scenario");
    let file_header = scenorio.versio.try_mut_map()["file_header"].try_mut_map();
    //Print the file header.
    println!(
        "Original file header with author's Chinese id: \n {:?} \n",
        &file_header
    );

    let creator = file_header["creator_name"].try_mut_str32();

    //Update the creator name.
    creator.set_content("Arian");

    //Print the file header.
    println!(
        "Updated file header with the author's English id: \n {:?} \n",
        &file_header
    );
}
