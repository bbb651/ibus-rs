// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{translate::*};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct XML(Boxed<ffi::IBusXML>);

    match fn {
        copy => |ptr| ffi::ibus_xml_copy(ptr),
        free => |ptr| ffi::ibus_xml_free(ptr),
        type_ => || ffi::ibus_xml_get_type(),
    }
}

impl XML {
    //#[doc(alias = "ibus_xml_output")]
    //pub fn output(&self, output: /*Ignored*/&mut glib::String) {
    //    unsafe { TODO: call ffi:ibus_xml_output() }
    //}

    #[doc(alias = "ibus_xml_parse_buffer")]
    pub fn parse_buffer(buffer: &str) -> Option<XML> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::ibus_xml_parse_buffer(buffer.to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_xml_parse_file")]
    pub fn parse_file(name: &str) -> Option<XML> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::ibus_xml_parse_file(name.to_glib_none().0))
        }
    }
}
