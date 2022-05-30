mod main

#[test]
fn test_gcd() {
    println!("Testing main::gcd fu");
    assert_eq!(main::gcd(14, 15), 1);
    assert_eq!(main::gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}