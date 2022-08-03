use aoe2_probe::scenorio::Scenorio;

fn main() {
    let scenorio = Scenorio::from_file("./resources/temp.aoe2scenario");
    println!("{:?}", scenorio.versio);
    scenorio.to_file("./resources/temp.aoe2scenario");
}
