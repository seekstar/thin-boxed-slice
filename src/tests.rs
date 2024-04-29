#[cfg(test)]
mod tests {
    use core::mem::size_of;
    use std::collections::HashSet;
    use std::ops::Deref;

    use crate::ThinBoxedSlice;

    #[test]
    fn option_size() {
        assert_eq!(
            size_of::<Option<ThinBoxedSlice<i32>>>(),
            size_of::<ThinBoxedSlice<i32>>()
        );
    }
    #[test]
    fn hash_set_basic() {
        let data = &[1, 2, 3];
        let mut s: HashSet<ThinBoxedSlice<i32>> = HashSet::new();
        s.insert(ThinBoxedSlice::from(data));
        let mut it = s.iter();
        assert_eq!(it.next().unwrap().deref(), data);
        assert_eq!(it.next(), None);
    }
}
