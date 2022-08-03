use aoe2_probe::io::Source;
use aoe2_probe::parse::serde::Serialize;
use aoe2_probe::parse::TokenBuilder;
use aoe2_probe::prebuilt::ver1_46::FileHeader;

fn main() {
    let template = FileHeader::template();
    let original = template.to_le_vec();
    let mock = TokenBuilder::create_from_template(&template, &mut Source::new(original.clone()));

    println!("original {:?}", &original);
    println!("mock {:?}", mock);
}
