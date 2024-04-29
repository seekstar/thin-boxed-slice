//! `ThinBoxedSlice` stores the size of the slice before the content of the slice, so that `size_of::<ThinBoxedSlice>` is only the size of a pointer:
//!
//! ```
//! use core::mem::size_of;
//! use thin_boxed_slice::ThinBoxedSlice;
//! assert_eq!(size_of::<ThinBoxedSlice<u8>>(), size_of::<*mut u8>());
//! ```
//!
//! # Examples
//!
//! ```
//! use thin_boxed_slice::ThinBoxedSlice;
//! use core::ops::Deref;
//!
//! let data = &[1, 2, 3];
//! let result = ThinBoxedSlice::<i32>::from(data);
//! assert_eq!(result.len(), 3);
//! assert_eq!(result.deref(), data);
//! ```

use core::borrow::Borrow;
use core::cmp::max;
use core::hash::Hash;
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::ops::Deref;
use core::slice;

use allocator_api2::alloc::{Allocator, Global};

mod tests;

#[derive(Debug)]
pub struct ThinBoxedSlice<T, A: Allocator = Global> {
    // TODO: Tell the compiler that this pointer is non-null
    p: *mut u8,
    allocator: A,
    phantom: PhantomData<T>,
}

impl<T, A: Allocator> ThinBoxedSlice<T, A> {
    const fn array_offset() -> usize {
        let align = align_of::<T>();
        let misalign = size_of::<usize>() % align;
        let padding = if misalign == 0 { 0 } else { align - misalign };
        size_of::<usize>() + padding
    }
    fn layout(n: usize) -> std::alloc::Layout {
        let alloc_len = Self::array_offset() + n * size_of::<T>();
        let align = max(align_of::<usize>(), align_of::<T>());
        std::alloc::Layout::from_size_align(alloc_len, align).unwrap()
    }
    fn array_ptr(&self) -> *mut T {
        unsafe { self.p.add(Self::array_offset()) as *mut T }
    }
    fn len(&self) -> usize {
        unsafe { self.p.cast::<usize>().read() }
    }
}

impl<T: Clone, A: Allocator> ThinBoxedSlice<T, A> {
    pub fn new_in(s: &[T], allocator: A) -> Self {
        let layout = Self::layout(s.len());
        unsafe {
            let p = allocator.allocate(layout).unwrap().as_ptr().cast::<u8>();
            p.cast::<usize>().write(s.len());
            let ret = Self {
                p,
                allocator,
                phantom: PhantomData::default(),
            };
            let mut v = ret.array_ptr();
            // Useful unstable: maybe_uninit_write_slice
            for i in 0..s.len() {
                v.write(s[i].clone());
                v = v.add(1);
            }
            ret
        }
    }
}

impl<T, A: Allocator> Drop for ThinBoxedSlice<T, A> {
    fn drop(&mut self) {
        unsafe {
            self.allocator.deallocate(
                (&*self.p.cast::<u8>()).into(),
                Self::layout(self.len()),
            );
        }
    }
}

impl<T: Clone, A: Allocator + Default> From<&[T]> for ThinBoxedSlice<T, A> {
    fn from(value: &[T]) -> Self {
        Self::new_in(value, A::default())
    }
}

impl<T: Clone, A: Allocator + Default, const N: usize> From<&[T; N]>
    for ThinBoxedSlice<T, A>
{
    fn from(value: &[T; N]) -> Self {
        Self::from(value.as_slice())
    }
}

impl<T, A: Allocator> Deref for ThinBoxedSlice<T, A> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.array_ptr(), self.len()) }
    }
}

impl<T, A: Allocator> Borrow<[T]> for ThinBoxedSlice<T, A> {
    fn borrow(&self) -> &[T] {
        self.deref()
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for ThinBoxedSlice<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T: PartialEq, A: Allocator> Eq for ThinBoxedSlice<T, A> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T: Hash, A: Allocator> Hash for ThinBoxedSlice<T, A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}
