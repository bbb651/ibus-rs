// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "IBusObject")]
    pub struct Object(Object<ffi::IBusObject, ffi::IBusObjectClass>);

    match fn {
        type_ => || ffi::ibus_object_get_type(),
    }
}

impl Object {
        pub const NONE: Option<&'static Object> = None;
    

    #[doc(alias = "ibus_object_new")]
    pub fn new() -> Object {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_object_new())
        }
    }
}

impl Default for Object {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Object>> Sealed for T {}
}

pub trait ObjectExt: IsA<Object> + sealed::Sealed + 'static {
    #[doc(alias = "ibus_object_destroy")]
    fn destroy(&self) {
        unsafe {
            ffi::ibus_object_destroy(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "destroy")]
    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn destroy_trampoline<P: IsA<Object>, F: Fn(&P) + 'static>(this: *mut ffi::IBusObject, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"destroy\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(destroy_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Object>> ObjectExt for O {}