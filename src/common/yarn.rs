use std::{hash::Hash, marker::PhantomData, mem::{self, MaybeUninit}, num::NonZeroUsize, path::Path, ptr, str::{self, pattern::{Pattern, ReverseSearcher, Searcher}, FromStr}};
use core::slice;

use regex::Regex;

const BORROWED: u8 = 0;
const HEAP: u8 = 1;
const SMALL: u8 = 2;
const STATIC: u8 = 3;
const SSO_LEN: usize = (mem::size_of::<usize>() * 2) - 1;

#[repr(C)]
#[derive(Clone, Copy)]
struct RawYarn {
    ptr: MaybeUninit<*mut u8>,
    len: NonZeroUsize
}

impl RawYarn {

    unsafe fn from_raw_parts(ptr: *mut u8, len: usize, kind: u8) -> Self {
        assert!(len <= usize::MAX / 4, "No Way!");
        assert!(len != 0);

        RawYarn {
            ptr: MaybeUninit::new(ptr),
            len: NonZeroUsize::new_unchecked((kind as usize & 0b11) << usize::BITS - 2 | len)
        }
    }

    unsafe fn from_small(data: *const u8, len: usize) -> Self {
        assert!(len <= SSO_LEN, "Not valid");
        let mut yarn = Self {
            ptr: MaybeUninit::uninit(),
            len: NonZeroUsize::new_unchecked(((SMALL as usize) << 6 | len) << usize::BITS - 8)
        };

        ptr::copy_nonoverlapping(
            data,
            &mut yarn as *mut RawYarn as *mut u8,
            len
        );

        yarn
    }

    fn kind(&self) -> u8 {
        (self.len.clone().get() >> (usize::BITS - 2)) as u8
    }

    fn len(&self) -> usize {
        (self.len.clone().get() << 2) >> (2 + self.adjust())
    }

    unsafe fn as_ptr(&self) -> *const u8 {
        match self.kind() {
            SMALL => self as *const Self as *const u8,
            _ => self.ptr.assume_init()
        }
    }

    unsafe fn as_mut_ptr(&mut self) -> *mut u8  {
        match self.kind() {
            SMALL => self as *mut Self as *mut u8,
            _ => self.ptr.assume_init().as_mut().unwrap()
        }
    }

    fn adjust(&self) -> u32 {
        match self.kind() {
            SMALL => usize::BITS - 8,
            _ => 0
        }
    }

    unsafe fn as_slice(&self) -> &'_ [u8] {
        slice::from_raw_parts(self.as_ptr(), self.len())
    }

    unsafe fn as_mut_slice(&mut self) -> &'_ mut [u8] {
        slice::from_raw_parts_mut(self.as_mut_ptr(), self.len())
    }  

}

impl PartialEq for RawYarn {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            self.as_slice() == other.as_slice()
        }
    }

    fn ne(&self, other: &Self) -> bool {
        unsafe {
            self.as_slice() != other.as_slice()
        }
    }
}

impl Eq for RawYarn {
    fn assert_receiver_is_total_eq(&self) {
        
    }
}

impl Hash for RawYarn {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let me = unsafe {
            self.as_slice()
        };
        state.write(me);
        state.write_usize(self.len());
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Yarn<'a> {
    raw: RawYarn,
    _ph: PhantomData<&'a str>
}

impl<'a> Yarn<'a> {

    pub fn from_char(data: char) -> Self {
        let mut buf = [0u8; 4];
        let data = data.encode_utf8(&mut buf);

        Self {
            raw: unsafe { RawYarn::from_small(buf.as_ptr(), 4) },
            _ph: PhantomData
        }
    }

    pub fn from_static(data: &'static str) -> Self {
        let len = data.len();
        let ptr = data.as_ptr().cast_mut();

        if len <= SSO_LEN {
            unsafe{ return Self {
                raw: RawYarn::from_small(ptr, len),
                _ph: PhantomData
            }};
        }

        Self {
            raw: unsafe { RawYarn::from_raw_parts(ptr, len, STATIC) },
            _ph: PhantomData
        }

    }

    pub fn borrowed(data: &'a str) -> Self {
        let len = data.len();
        let ptr = data.as_ptr().cast_mut();

        if len <= SSO_LEN {
            return Self {
                raw: unsafe {RawYarn::from_small(ptr, len)},
                _ph: PhantomData
            };
        }

        Self {
            raw: unsafe{ RawYarn::from_raw_parts(ptr, len, BORROWED) },
            _ph: PhantomData
        }
    }

    pub fn owned(data: Box<str>) -> Self {
        if data.len() <= SSO_LEN {
            return Self {
                raw: unsafe{RawYarn::from_small(data.as_ptr(), data.len())},
                _ph: PhantomData
            };            
        }

        let len = data.len();
        let ptr = data.as_ptr().cast_mut();
        mem::forget(data);

        Self {
            raw: unsafe{ RawYarn::from_raw_parts(ptr, len, HEAP) },
            _ph: PhantomData
        }
    }

    pub fn as_slice(&self) -> &'_ str {
        unsafe {
            str::from_utf8_unchecked(self.raw.as_slice())
        }
    }

    fn as_ptr(&self) -> *const u8 {
        unsafe{ self.raw.as_ptr() }
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        unsafe { self.raw.as_mut_ptr() }
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn spilt<P>(&'a self, pattern: P) -> Vec<Yarn<'_>>
    where P: Pattern<'a> {
        self.as_slice().split(pattern).map(|ch|{
            Self::borrowed(ch)
        }).collect()
    }

    pub fn ends_with<P, S>(&'a self, pattern: P) -> bool 
    where P: Pattern<'a, Searcher = S>,
          S: ReverseSearcher<'a> {
        self.as_slice().ends_with(pattern)
    }

    pub fn last_char(&self) -> Option<char> {
        self.as_slice().chars().last()
    }

    pub fn starts_with<P, S>(&'a self, pattern: P) -> bool 
    where P: Pattern<'a, Searcher = S>,
          S: Searcher<'a> {
        self.as_slice().starts_with(pattern)
    }

    pub fn regex_starts_with(&'a self, regex: Regex) -> bool {
        regex.is_match_at(self.as_slice(), 0)
    }

    pub fn regex_ends_with(&'a self, regex: Regex) -> bool {
        regex.is_match_at(self.as_slice(), self.len() - 1)
    }

    pub fn parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
        self.as_slice().parse::<T>()
    }
}

impl Yarn<'_> {
    pub fn immortalize(mut self) -> Yarn<'static> {
        if self.raw.kind() == BORROWED {
            let copy: Box<str> = self.as_slice().into();
            self = Yarn::owned(copy);
        }

        let raw = self.raw;

        mem::forget(self);
        Yarn::<'static> {
            raw,
            _ph: PhantomData
        }
    }


}

impl<'a> Into<Yarn<'a>> for &'a str {
    fn into(self) -> Yarn<'a> {
        Yarn::borrowed(self)
    }
}

impl Drop for Yarn<'_> {
    fn drop(&mut self) {
        let dropped = unsafe {
            Box::from_raw(self.raw.as_mut_slice())
        };
    }
}

impl Clone for Yarn<'_> {
    fn clone(&self) -> Self {
        let ptr = self.as_ptr().cast_mut();
        let len = self.len();

        Self {
            raw: unsafe{ RawYarn::from_raw_parts(ptr, len, BORROWED) },
            _ph: PhantomData
        }

    }
}

impl AsRef<Path> for Yarn<'_> {
    fn as_ref(&self) -> &Path {
        self.as_slice().as_ref()
    }
}

impl From<String> for Yarn<'_> {
    fn from(value: String) -> Self {
        let ptr = value.as_ptr().cast_mut();
        let len = value.len();

        Self {
            raw: unsafe{ RawYarn::from_raw_parts(ptr, len, BORROWED) },
            _ph: PhantomData
        }
    }
}

impl From<Vec<u8>> for Yarn<'_> {

    fn from(value: Vec<u8>) -> Self {
        let ptr = value.as_ptr().cast_mut();
        let len = value.len();

        Self {
            raw: unsafe { RawYarn::from_raw_parts(ptr, len, HEAP) },
            _ph: PhantomData
        }
    }

}


unsafe impl Send for Yarn<'_> {}
unsafe impl Sync for Yarn<'_> {}

