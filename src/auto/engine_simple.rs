// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Engine,Object,Service};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "IBusEngineSimple")]
    pub struct EngineSimple(Object<ffi::IBusEngineSimple, ffi::IBusEngineSimpleClass>) @extends Engine, Service, Object;

    match fn {
        type_ => || ffi::ibus_engine_simple_get_type(),
    }
}

impl EngineSimple {
        pub const NONE: Option<&'static EngineSimple> = None;
    

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`EngineSimple`] objects.
            ///
            /// This method returns an instance of [`EngineSimpleBuilder`](crate::builders::EngineSimpleBuilder) which can be used to create [`EngineSimple`] objects.
            pub fn builder() -> EngineSimpleBuilder {
                EngineSimpleBuilder::new()
            }
        
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`EngineSimple`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct EngineSimpleBuilder {
            builder: glib::object::ObjectBuilder<'static, EngineSimple>,
        }

        impl EngineSimpleBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn active_surrounding_text(self, active_surrounding_text: bool) -> Self {
                            Self { builder: self.builder.property("active-surrounding-text", active_surrounding_text), }
                        }

                            pub fn engine_name(self, engine_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("engine-name", engine_name.into()), }
                        }

                            pub fn has_focus_id(self, has_focus_id: bool) -> Self {
                            Self { builder: self.builder.property("has-focus-id", has_focus_id), }
                        }

                            pub fn connection(self, connection: &gio::DBusConnection) -> Self {
                            Self { builder: self.builder.property("connection", connection.clone()), }
                        }

                            pub fn object_path(self, object_path: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("object-path", object_path.into()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`EngineSimple`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> EngineSimple {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::EngineSimple>> Sealed for T {}
}

pub trait EngineSimpleExt: IsA<EngineSimple> + sealed::Sealed + 'static {
    #[doc(alias = "ibus_engine_simple_add_compose_file")]
    fn add_compose_file(&self, file: &str) -> bool {
        unsafe {
            from_glib(ffi::ibus_engine_simple_add_compose_file(self.as_ref().to_glib_none().0, file.to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_simple_add_table")]
    fn add_table(&self, data: &[u16], n_seqs: i32) {
        let max_seq_len = data.len() as _;
        unsafe {
            ffi::ibus_engine_simple_add_table(self.as_ref().to_glib_none().0, data.to_glib_none().0, max_seq_len, n_seqs);
        }
    }

    #[doc(alias = "ibus_engine_simple_add_table_by_locale")]
    fn add_table_by_locale(&self, locale: Option<&str>) -> bool {
        unsafe {
            from_glib(ffi::ibus_engine_simple_add_table_by_locale(self.as_ref().to_glib_none().0, locale.to_glib_none().0))
        }
    }
}

impl<O: IsA<EngineSimple>> EngineSimpleExt for O {}
