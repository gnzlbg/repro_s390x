#[cfg(test)]
extern crate ppv;

#[cfg(test)]
use ppv::i128x1;

#[test]
pub fn foo() {
    let x = i128x1::splat(0);
    let y = i128x1::splat(1);

    assert_eq!(y >> x, y);
}
