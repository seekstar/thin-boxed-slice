#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use crate::ThinBoxedSlice;

    #[test]
    fn option_size() {
        assert_eq!(
            size_of::<Option<ThinBoxedSlice<i32>>>(),
            size_of::<ThinBoxedSlice<i32>>()
        );
    }
}
