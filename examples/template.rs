use aoe2_probe::io::Source;
use aoe2_probe::parse::serde::Serialize;
use aoe2_probe::parse::wrap::Wrappable;
use aoe2_probe::parse::{Token, TokenBuilder};
use aoe2_probe::utils::{DynString, LinkedHashMap};

fn main() {
    let mut template = LinkedHashMap::new();
    template.push_back("name", Token::Int16(1));
    let mut union = LinkedHashMap::new();
    union.push_back("score", Token::Int8(-1));
    union.push_back("name", Token::Int8(2));
    template.push_back("def", Token::Union(union));
    template.push_back("score", Token::Int16(1));
    template.push_back("content", Token::Str32(DynString::new(4, "abcd")));

    let bytes = template.to_le_vec();
    let original = bytes.clone();

    let mock = TokenBuilder::create_from_template(&template.wrap(), &mut Source::new(bytes));

    println!("original {:?}", original);
    println!("mock {:?}", mock.to_le_vec());
}
