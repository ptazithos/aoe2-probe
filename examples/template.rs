use aoe2_probe::element::serde::Serialize;
use aoe2_probe::element::wrap::Wrappable;
use aoe2_probe::element::{Versio, VersioBuilder};
use aoe2_probe::io::Source;
use aoe2_probe::utils::{LinkedHashMap, PrefixString};

fn main() {
    let mut template = LinkedHashMap::new();
    template.push_back("name", Versio::Int16(1));
    let mut union = LinkedHashMap::new();
    union.push_back("score", Versio::Int8(-1));
    union.push_back("name", Versio::Int8(2));
    template.push_back("def", Versio::Union(union));
    template.push_back("score", Versio::Int16(1));
    template.push_back("content", Versio::Str32(PrefixString::new(4, "abcd")));

    let bytes = template.to_le_vec();
    let original = bytes.clone();

    let mock = VersioBuilder::create_from_template(&template.wrap(), &mut Source::new(bytes));

    println!("original {:?}", original);
    println!("mock {:?}", mock.to_le_vec());
}
