use aoe2_probe::parse::Encode;
use aoe2_probe::parse::Token;
use aoe2_probe::utils::{DynString, PatchedMap};

fn main() {
    let mut top_layer = PatchedMap::new();
    top_layer.push_back("name", Token::Int16(1));
    let mut second_layer = PatchedMap::new();
    second_layer.push_back("score", Token::Int8(-1));
    second_layer.push_back("name", Token::Int8(2));
    top_layer.push_back("def", Token::Union(second_layer));
    top_layer.push_back("score", Token::Int16(1));
    top_layer.push_back("content", Token::Str32(DynString::with_capacity(4, "abcd")));

    let bytes = top_layer.to_le_vec();

    println!("{:?}", bytes);
}
