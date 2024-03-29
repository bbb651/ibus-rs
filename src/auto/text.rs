// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AttrList,Object,Serializable};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "IBusText")]
    pub struct Text(Object<ffi::IBusText, ffi::IBusTextClass>) @extends Serializable, Object;

    match fn {
        type_ => || ffi::ibus_text_get_type(),
    }
}

impl Text {
        pub const NONE: Option<&'static Text> = None;
    

    //#[doc(alias = "ibus_text_new_from_printf")]
    //#[doc(alias = "new_from_printf")]
    //pub fn from_printf(fmt: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Text {
    //    unsafe { TODO: call ffi:ibus_text_new_from_printf() }
    //}

    #[doc(alias = "ibus_text_new_from_static_string")]
    #[doc(alias = "new_from_static_string")]
    pub fn from_static_string(str: &str) -> Text {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_text_new_from_static_string(str.to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_text_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(str: &str) -> Text {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_text_new_from_string(str.to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_text_new_from_ucs4")]
    #[doc(alias = "new_from_ucs4")]
    pub fn from_ucs4(str: char) -> Text {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_text_new_from_ucs4(str.into_glib()))
        }
    }

    #[doc(alias = "ibus_text_new_from_unichar")]
    #[doc(alias = "new_from_unichar")]
    pub fn from_unichar(c: char) -> Text {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_text_new_from_unichar(c.into_glib()))
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Text>> Sealed for T {}
}

pub trait TextExt: IsA<Text> + sealed::Sealed + 'static {
    #[doc(alias = "ibus_text_append_attribute")]
    fn append_attribute(&self, type_: u32, value: u32, start_index: u32, end_index: i32) {
        unsafe {
            ffi::ibus_text_append_attribute(self.as_ref().to_glib_none().0, type_, value, start_index, end_index);
        }
    }

    #[doc(alias = "ibus_text_get_attributes")]
    #[doc(alias = "get_attributes")]
    fn attributes(&self) -> Option<AttrList> {
        unsafe {
            from_glib_none(ffi::ibus_text_get_attributes(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_text_get_is_static")]
    #[doc(alias = "get_is_static")]
    fn is_static(&self) -> bool {
        unsafe {
            from_glib(ffi::ibus_text_get_is_static(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_text_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> u32 {
        unsafe {
            ffi::ibus_text_get_length(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "ibus_text_get_text")]
    #[doc(alias = "get_text")]
    fn text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_text_get_text(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_text_set_attributes")]
    fn set_attributes(&self, attrs: &impl IsA<AttrList>) {
        unsafe {
            ffi::ibus_text_set_attributes(self.as_ref().to_glib_none().0, attrs.as_ref().to_glib_none().0);
        }
    }
}

impl<O: IsA<Text>> TextExt for O {}
