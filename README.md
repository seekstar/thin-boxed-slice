# thin-boxed-slice

`ThinBoxedSlice` stores the size of the slice before the content of the slice, so that `size_of::<ThinBoxedSlice>` is only the size of a pointer.

I mainly use it as the key of hash tables, therefore not all traits of `Box` are implemented for `ThinBoxedSlice`. If you need some additional traits, you may create an issue or PR.

## Similar projects

### [thin-vec](https://docs.rs/thin-vec)

### [thin-slice](https://docs.rs/thin-slice)

Note that `thin-slice` stores the fat pointer of the slice into a new heap memory if the slice length is large.
