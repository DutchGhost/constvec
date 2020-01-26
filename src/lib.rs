#![no_std]
#![feature(const_generics)]
#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(const_if_match)]
#![feature(const_panic)]
#![feature(const_raw_ptr_deref)]
#![feature(slice_from_raw_parts)]
#![feature(const_slice_from_raw_parts)]

use core::{mem::MaybeUninit, ptr};

/// A `ConstVec` is an array with a Vec like API,
/// but usable in constant functions.
///
/// In order to increase or decrease the number
/// of elements held by the ConstVec,
/// the element type must implement Copy. This is
/// to prevent accidental leaks in a non-const
/// function (Copy implies !Drop).
pub struct ConstVec<T, const N: usize> {
    /// This MaybeUninit is const constructable
    data: MaybeUninit<[T; N]>,
    len: usize,
}

impl<T, const N: usize> ConstVec<T, { N }> {
    const unsafe fn as_slice_mut(&mut self) -> &mut [T] {
        let len = self.len();
        let ptr = &mut self.data as *mut _ as *mut T;
        &mut *ptr::slice_from_raw_parts_mut(ptr, len)
    }

    const unsafe fn as_uninit_slice_mut(&mut self) -> &mut [MaybeUninit<T>] {
        let ptr = &mut self.data as *mut _ as *mut MaybeUninit<T>;
        &mut *ptr::slice_from_raw_parts_mut(ptr, N)
    }
}

impl<T, const N: usize> ConstVec<T, { N }> {
    /// Returns a new, empty ConstVec.
    pub const fn new() -> Self {
        Self {
            data: MaybeUninit::uninit(),
            len: 0,
        }
    }

    /// Returns the length of the ConstVec.
    /// This is how many elements it currently contains.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns the capacity of the ConstVec.
    /// This is how many elements it can maximally hold.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns a bool to indicate whether the ConstVec
    /// is empty or not.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a bool to indicate wheter the ConstVec
    /// is not empty
    pub const fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    /// Returns a bool to indicate whether the ConstVec
    /// is full. This means the ConstVec has reached
    /// its capacity, and does not have room for new
    /// elements.
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Returns bool to indicate whether the ConstVec
    /// is not full.
    pub const fn is_not_full(&self) -> bool {
        self.len < N
    }

    pub const unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }
}

/// Growing and shrinkin requires T: Copy,
/// because T: Copy implies T: !Drop.
///
/// We require T to not implement Drop,
/// because only then we can ensure we
/// don't leak accidentally in non-const
/// functions. Otherwise, there is a
/// possability to leak, as ConstVec does NOT
/// implement Drop. It can't implement Drop,
/// since that takes away the usability in
/// const functions.
impl<T: Copy, const N: usize> ConstVec<T, { N }> {
    /// Pushes `data` onto the ConstVec.
    pub const unsafe fn push_unchecked(&mut self, data: T) {
        debug_assert!(self.is_not_full());
        let len = self.len();
        let slice = self.as_uninit_slice_mut();
        slice[len] = MaybeUninit::new(data);

        self.set_len(len + 1);
    }

    /// Attempts to push `data` onto the ConstVec.
    /// Returns a Result to indicate success or failure.
    pub const fn try_push(&mut self, data: T) -> Result<(), T> {
        if self.is_full() {
            Err(data)
        } else {
            unsafe {
                self.push_unchecked(data);
                Ok(())
            }
        }
    }

    /// Pushes `data` onto the ConstVec.
    ///
    /// # Panic
    /// Panic's if the maximum capacity was already reached.
    pub const fn push(&mut self, data: T) {
        match self.try_push(data) {
            Ok(_) => {}
            Err(_) => panic!("ConstBuf::push called trough ConstBuf already at maximum capacity!"),
        }
    }

    /// Pops the last element from the ConstVec.
    pub const unsafe fn pop_unchecked(&mut self) -> T {
        assert!(self.is_not_empty());
        let len = self.len() - 1;
        let slice = unsafe { self.as_slice_mut() };
        let popped = slice[len];

        self.set_len(len);
        popped
    }

    /// Pops the last element from the ConstVec and
    /// returns it, or None if it is empty.
    pub const fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    /// Clears the ConstVec.
    pub const fn clear(&mut self) {
        unsafe { self.set_len(0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn run() {
        let mut b = ConstVec::<_, 10>::new();
        b.push(20i32);

        match b.pop() {
            Some(n) => assert!(n == 20),
            None => panic!("The vec should contain at least one element!"),
        }

        assert!(b.is_empty());

        match b.pop() {
            Some(_) => panic!("The vector should be empty!"),
            None => {}
        }
    }

    #[test]
    fn it_works() {
        const _: () = run();
    }
}
