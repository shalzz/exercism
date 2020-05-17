use std::cmp::Ordering;

pub fn find<R: AsRef<[T]>, T: PartialOrd>(array: R, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mid = array.len() / 2;

    match array.get(mid)?.partial_cmp(&key).unwrap() {
        Ordering::Equal => Some(mid),
        Ordering::Greater => find(&array[..mid], key),
        Ordering::Less => find(&array[(mid + 1)..], key).map(|index| mid + 1 + index),
    }
}
