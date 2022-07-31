use aoe2_probe::element::serde::Serialize;
use aoe2_probe::element::Versio;
use aoe2_probe::utils::map::LinkedHashMap;
fn main() {
    let mut top_layer = LinkedHashMap::new();
    top_layer.push_back("name", Versio::Int16(1));
    let mut second_layer = LinkedHashMap::new();
    second_layer.push_back("score", Versio::Int8(-1));
    second_layer.push_back("name", Versio::Int8(2));
    top_layer.push_back("def", Versio::Union(second_layer));
    top_layer.push_back("score", Versio::Int16(1));

    let bytes = top_layer.to_le_vec();
    println!("{:?}", bytes)
}
