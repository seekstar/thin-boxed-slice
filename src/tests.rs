#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::ops::Deref;

    use crate::ThinBoxedSlice;

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
