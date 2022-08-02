use aoe2_probe::scenorio::Scenorio;

fn main() {
    let scenorio = Scenorio::from_file("./resources/chapter_1.aoe2scenario");

    println!("{:?}", scenorio.versio)
}
