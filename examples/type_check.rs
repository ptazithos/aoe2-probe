use aoe2_probe::parse::Token;
// In short, the trait PartialEq for Token is used for type check.
// Don't expect it for a value check!
fn main() {
    let uin16_token = Token::UInt16(0);
    let another_uint16_token = Token::UInt16(1);
    let uin8_token = Token::UInt8(0);

    //type compare
    println!(
        "uint16_token's type equals to another_uint16_token? Answer {}",
        uin16_token == another_uint16_token
    );

    println!(
        "uint16_token's type doesn't equal to uint8_token? Answer {}",
        uin16_token != uin8_token
    );

    //value compare
    println!(
        "uint16_token's value doesn't equal to another_uint16_token? Answer {}",
        uin16_token.try_u16() == another_uint16_token.try_u16()
    );

    println!(
        "uint16_token's value equals to uint8_token? Answer {}",
        *uin16_token.try_u16() != (*uin8_token.try_u8() as u16)
    );
}
