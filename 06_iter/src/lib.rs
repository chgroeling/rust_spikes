
#[test]
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // iter() returns an iterator of slices.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn into_iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.into_iter();

    // into_iter() returns an iterator from a value.
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iter_mut_demo() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();

    // iter_mut() returns an iterator that allows modifying each value.
    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);
}