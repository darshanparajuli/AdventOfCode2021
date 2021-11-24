use std::convert::{AsMut, AsRef};
use std::ptr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RawPtr<T> {
    ptr: *mut T,
}

impl<T> RawPtr<T> {
    pub fn from_boxed(ptr: Box<T>) -> Self {
        Self {
            ptr: Box::into_raw(ptr),
        }
    }

    pub fn null() -> Self {
        Self {
            ptr: ptr::null_mut(),
        }
    }

    pub fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref().unwrap() }
    }

    pub fn set(&mut self, ptr: *mut T) {
        self.ptr = ptr;
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn ptr(&self) -> *mut T {
        self.ptr
    }

    pub fn into_boxed(self) -> Box<T> {
        unsafe { Box::from_raw(self.ptr) }
    }
}

impl<T> AsMut<T> for RawPtr<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl<T> AsRef<T> for RawPtr<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}
