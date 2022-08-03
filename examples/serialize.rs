use aoe2_probe::parse::serde::Serialize;
use aoe2_probe::parse::Token;
use aoe2_probe::utils::{DynString, LinkedHashMap};

fn main() {
    let mut top_layer = LinkedHashMap::new();
    top_layer.push_back("name", Token::Int16(1));
    let mut second_layer = LinkedHashMap::new();
    second_layer.push_back("score", Token::Int8(-1));
    second_layer.push_back("name", Token::Int8(2));
    top_layer.push_back("def", Token::Union(second_layer));
    top_layer.push_back("score", Token::Int16(1));
    top_layer.push_back("content", Token::Str32(DynString::with_capacity(4, "abcd")));

    let bytes = top_layer.to_le_vec();

    println!("{:?}", bytes);
}
