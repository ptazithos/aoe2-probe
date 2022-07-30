use aoe2_probe::scenorio::Scenorio;
fn main() {
    let scenorio = Scenorio::from_file("./resources/chapter_1.aoe2scenario");
    let versio = scenorio.versio();
    println!("{:?}", versio)
}
