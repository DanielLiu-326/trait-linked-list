use std::ops::{Deref, DerefMut};

pub struct Ptr<Dyn: ?Sized> {
    ptr: *const (),
    meta: <Dyn as std::ptr::Pointee>::Metadata,
}

impl<Dyn:?Sized> Clone for Ptr<Dyn> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Dyn:?Sized> Copy for Ptr<Dyn> {}

impl<Dyn: ?Sized> Ptr<Dyn> {
    pub fn new(ptr: &Dyn) -> Self {
        Self {
            ptr: (ptr as *const Dyn).cast(),
            meta: std::ptr::metadata(ptr as *const Dyn),
        }
    }
    pub fn null() -> Self {
        Self {
            ptr: std::ptr::null(),
            meta: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
        }
    }
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    pub fn thin(&self) -> *const () {
        return self.ptr;
    }
    pub fn metadata(&self) -> <Dyn as std::ptr::Pointee>::Metadata {
        self.meta
    }
}
impl<Dyn:?Sized> Into<*const Dyn> for Ptr<Dyn> {
    fn into(self) -> *const Dyn {
        std::ptr::from_raw_parts(self.ptr,self.meta)
    }
}
impl<Dyn: ?Sized> From<&Dyn> for Ptr<Dyn> {
    fn from(f: &Dyn) -> Self {
        Ptr::new(f)
    }
}
impl<Dyn: ?Sized> Deref for Ptr<Dyn> {
    type Target = Dyn;

    fn deref(&self) -> &Self::Target {
        unsafe{
            &*std::ptr::from_raw_parts(self.ptr, self.meta)
        }
    }
}
///
/// DynPtrMut:
/// 可变胖/瘦指针，相对于primitive类型增加了null值，对null值解引用将出错
///
pub struct PtrMut<Dyn: ?Sized> {
    ptr: *mut (),
    meta: <Dyn as std::ptr::Pointee>::Metadata,
}

impl<Dyn:?Sized> Clone for PtrMut<Dyn> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Dyn:?Sized> Copy for PtrMut<Dyn> {}

impl<Dyn: ?Sized> PtrMut<Dyn> {
    pub fn new(ptr: &mut Dyn) -> Self {
        Self {
            ptr: (ptr as *mut Dyn).cast(),
            meta: std::ptr::metadata(ptr as *mut Dyn),
        }
    }
    #[inline]
    pub fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            meta: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
        }
    }
    #[inline]
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    #[inline]
    pub fn thin(&self) -> *mut () {
        self.ptr
    }
    #[inline]
    pub fn metadata(&self) -> <Dyn as std::ptr::Pointee>::Metadata {
        self.meta
    }
}

impl<Dyn:?Sized> Into<*mut Dyn> for PtrMut<Dyn> {
    fn into(self) -> *mut Dyn {
        std::ptr::from_raw_parts_mut(self.ptr,self.meta)
    }
}
impl<Dyn:?Sized> Into<*const Dyn> for PtrMut<Dyn> {
    fn into(self) -> *const Dyn {
        std::ptr::from_raw_parts(self.ptr,self.meta)
    }
}

impl<Dyn: ?Sized+'static> From<&mut Dyn> for PtrMut<Dyn> {
    fn from(f: &mut Dyn) -> Self {
        PtrMut::new(f)
    }
}
impl<Dyn: ?Sized> Deref for PtrMut<Dyn> {
    type Target = Dyn;

    fn deref(&self) -> &Self::Target {
        unsafe{
            &*std::ptr::from_raw_parts(self.ptr, self.meta)
        }
    }
}

impl<Dyn: ?Sized> DerefMut for PtrMut<Dyn> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *std::ptr::from_raw_parts_mut(self.ptr, self.meta)
        }
    }
}
