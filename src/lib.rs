use core::borrow::Borrow;
use core::cmp::max;
use core::hash::Hash;
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::slice;

use allocator_api2::alloc::{self, Allocator, Global};

mod tests;

#[derive(Debug)]
pub struct ThinBoxedSlice<T, A: Allocator = Global> {
    p: *mut u8,
    phantom: PhantomData<(T, A)>,
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
    pub fn len(&self) -> usize {
        unsafe { self.p.cast::<usize>().read() }
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.array_ptr(), self.len()) }
    }
}

impl<T, A: Allocator> Drop for ThinBoxedSlice<T, A> {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.p, Self::layout(self.len()));
        }
    }
}

impl<T: Clone, A: Allocator> From<&[T]> for ThinBoxedSlice<T, A> {
    fn from(value: &[T]) -> Self {
        let layout = Self::layout(value.len());
        unsafe {
            let p = alloc::alloc(layout);
            if p.is_null() {
                alloc::handle_alloc_error(layout);
            }
            p.cast::<usize>().write(value.len());
            let ret = Self {
                p,
                phantom: PhantomData::default(),
            };
            let mut v = ret.array_ptr();
            // Useful unstable: maybe_uninit_write_slice
            for i in 0..value.len() {
                v.write(value[i].clone());
                v = v.add(1);
            }
            ret
        }
    }
}

impl<T: Clone, A: Allocator, const N: usize> From<&[T; N]>
    for ThinBoxedSlice<T, A>
{
    fn from(value: &[T; N]) -> Self {
        Self::from(value.as_slice())
    }
}

impl<T, A: Allocator> Borrow<[T]> for ThinBoxedSlice<T, A> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for ThinBoxedSlice<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: PartialEq, A: Allocator> Eq for ThinBoxedSlice<T, A> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T: Hash, A: Allocator> Hash for ThinBoxedSlice<T, A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}
