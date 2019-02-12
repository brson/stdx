extern crate encoding_rs;
use encoding_rs::*;

fn main() {
    let expected = "\u{30CF}\u{30ED}\u{30FC}\u{30FB}\u{30EF}\u{30FC}\u{30EB}\u{30C9}";
    let encoded = b"\x83n\x83\x8D\x81[\x81E\x83\x8F\x81[\x83\x8B\x83h";

    let (decoded, encoding_used, had_errors) = SHIFT_JIS.decode(encoded);

    assert_eq!(&decoded[..], expected);
    assert_eq!(encoding_used, SHIFT_JIS);
    assert!(!had_errors);

    println!("Decoded result: {}", decoded);
}
