#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::mem::size_of;

    use crate::ThinBoxedSlice;

    #[test]
    fn example_usage() {
        assert_eq!(size_of::<ThinBoxedSlice<u8>>(), size_of::<*mut u8>());

        let data = &[1, 2, 3];
        let result = ThinBoxedSlice::<i32>::from(data);
        assert_eq!(result.len(), 3);
        assert_eq!(result.as_slice(), data);
    }

    #[test]
    fn hash_set_basic() {
        let data = &[1, 2, 3];
        let mut s: HashSet<ThinBoxedSlice<i32>> = HashSet::new();
        s.insert(ThinBoxedSlice::from(data));
        let mut it = s.iter();
        assert_eq!(it.next().unwrap().as_slice(), data);
        assert_eq!(it.next(), None);
    }
}
