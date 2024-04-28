# thin-boxed-slice

`ThinBoxedSlice` Stores the size of the slice before the content of the slice, so that `size_of::<ThinBoxedSlice>` is only the size of a pointer.

## Example usage

```rs
assert_eq!(size_of::<ThinBoxedSlice<u8>>(), size_of::<*mut u8>());

let data = &[1, 2, 3];
let result = ThinBoxedSlice::from(data);
assert_eq!(result.len(), 3);
assert_eq!(result.as_slice(), data);
```

## Similar projects

### [thin-vec](https://docs.rs/thin-vec)

### [thin-slice](https://docs.rs/thin-slice)

Note that `thin-slice` stores the fat pointer of the slice into a new heap memory if the slice length is large.
