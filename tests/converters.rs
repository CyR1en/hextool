use hextool;
use hextool::{Convert, Hex, UnHex};

#[test]
fn test_hex() {
    let hex = Hex::convert("hello", false, false);
    assert_eq!(hex, "68656c6c6f");

    let hex = Hex::convert("hello", false, true);
    assert_eq!(hex, "68 65 6c 6c 6f");

    // Convert a string with numeric flag. This should return an error.
    let hex = Hex::convert("hello", true, false);
    assert_eq!(hex, "Input is not valid for 'hex' with numeric flag (-n).");

    // Convert 3405691582 base 10 to base 16
    let hex = Hex::convert("3405691582", true, false);
    assert_eq!(hex, "cafebabe");

    let hex = Hex::convert("3405691582", true, true);
    assert_eq!(hex, "ca fe ba be");
}

#[test]
fn test_unhex() {
    //write unit test
    let unhex = UnHex::convert("68656c6c6f", false, false);
    assert_eq!(unhex, "hello");

    // Convert string over letter `f`. This should return an error.
    let unhex = UnHex::convert("abcdefgabc", false, false);
    assert_eq!(unhex, "The highlighted chars can't be converted:\nabcdef\u{1b}[31mg\u{1b}[0mabc.");

    // Convert valid string with numeric flag.
    let unhex = UnHex::convert("ff", true, false);
    assert_eq!(unhex, "255");

    // Convert valid string with numeric flag with 0x prefix.
    let unhex = UnHex::convert("0xff", true, false);
    assert_eq!(unhex, "255");

    // Convert 0x41 string input with numeric flag.
    let unhex = UnHex::convert("0x41", true, false);
    assert_eq!(unhex, "65");

    // Convert 0x41 string input with numeric flag set to false.
    let unhex = UnHex::convert("0x41", false, false);
    assert_eq!(unhex, "A");

    // Convert cafebabe base 16 to base 10
    let unhex = UnHex::convert("cafebabe", true, false);
    assert_eq!(unhex, "3405691582");
}

