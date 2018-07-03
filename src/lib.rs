#[cfg(test)]
extern crate ppv;

#[cfg(test)]
use ppv::i128x1;

#[test]
pub fn foo() {
    let z = i128x1::splat(0 as i128);
    let o = i128x1::splat(1 as i128);
    let t = i128x1::splat(2 as i128);

    // shr
    assert_eq!(z >> z, z);
    assert_eq!(z >> o, z);
    assert_eq!(z >> t, z);
    assert_eq!(z >> t, z);

    assert_eq!(o >> z, o);
}
