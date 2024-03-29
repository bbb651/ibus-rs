// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Object,Serializable,XML};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "IBusEngineDesc")]
    pub struct EngineDesc(Object<ffi::IBusEngineDesc, ffi::IBusEngineDescClass>) @extends Serializable, Object;

    match fn {
        type_ => || ffi::ibus_engine_desc_get_type(),
    }
}

impl EngineDesc {
        pub const NONE: Option<&'static EngineDesc> = None;
    

    #[doc(alias = "ibus_engine_desc_new")]
    pub fn new(name: &str, longname: &str, description: &str, language: &str, license: &str, author: &str, icon: &str, layout: &str) -> EngineDesc {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_new(name.to_glib_none().0, longname.to_glib_none().0, description.to_glib_none().0, language.to_glib_none().0, license.to_glib_none().0, author.to_glib_none().0, icon.to_glib_none().0, layout.to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_new_from_xml_node")]
    #[doc(alias = "new_from_xml_node")]
    pub fn from_xml_node(node: &mut XML) -> EngineDesc {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_new_from_xml_node(node.to_glib_none_mut().0))
        }
    }

    //#[doc(alias = "ibus_engine_desc_new_varargs")]
    //pub fn new_varargs(first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> EngineDesc {
    //    unsafe { TODO: call ffi:ibus_engine_desc_new_varargs() }
    //}

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`EngineDesc`] objects.
            ///
            /// This method returns an instance of [`EngineDescBuilder`](crate::builders::EngineDescBuilder) which can be used to create [`EngineDesc`] objects.
            pub fn builder() -> EngineDescBuilder {
                EngineDescBuilder::new()
            }
        
}

impl Default for EngineDesc {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`EngineDesc`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct EngineDescBuilder {
            builder: glib::object::ObjectBuilder<'static, EngineDesc>,
        }

        impl EngineDescBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn author(self, author: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("author", author.into()), }
                        }

                            pub fn description(self, description: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("description", description.into()), }
                        }

                            pub fn hotkeys(self, hotkeys: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("hotkeys", hotkeys.into()), }
                        }

                            pub fn icon(self, icon: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("icon", icon.into()), }
                        }

                            pub fn icon_prop_key(self, icon_prop_key: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("icon-prop-key", icon_prop_key.into()), }
                        }

                            pub fn language(self, language: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("language", language.into()), }
                        }

                            pub fn layout(self, layout: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("layout", layout.into()), }
                        }

                            pub fn layout_option(self, layout_option: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("layout-option", layout_option.into()), }
                        }

                            pub fn layout_variant(self, layout_variant: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("layout-variant", layout_variant.into()), }
                        }

                            pub fn license(self, license: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("license", license.into()), }
                        }

                            pub fn longname(self, longname: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("longname", longname.into()), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn rank(self, rank: u32) -> Self {
                            Self { builder: self.builder.property("rank", rank), }
                        }

                            pub fn setup(self, setup: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("setup", setup.into()), }
                        }

                            pub fn symbol(self, symbol: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("symbol", symbol.into()), }
                        }

                            pub fn textdomain(self, textdomain: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("textdomain", textdomain.into()), }
                        }

                            pub fn version(self, version: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("version", version.into()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`EngineDesc`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> EngineDesc {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::EngineDesc>> Sealed for T {}
}

pub trait EngineDescExt: IsA<EngineDesc> + sealed::Sealed + 'static {
    #[doc(alias = "ibus_engine_desc_get_author")]
    #[doc(alias = "get_author")]
    fn author(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_author(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_description(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_hotkeys")]
    #[doc(alias = "get_hotkeys")]
    fn hotkeys(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_hotkeys(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_icon(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_icon_prop_key")]
    #[doc(alias = "get_icon_prop_key")]
    fn icon_prop_key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_icon_prop_key(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_language")]
    #[doc(alias = "get_language")]
    fn language(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_language(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_layout")]
    #[doc(alias = "get_layout")]
    fn layout(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_layout(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_layout_option")]
    #[doc(alias = "get_layout_option")]
    fn layout_option(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_layout_option(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_layout_variant")]
    #[doc(alias = "get_layout_variant")]
    fn layout_variant(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_layout_variant(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_license")]
    #[doc(alias = "get_license")]
    fn license(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_license(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_longname")]
    #[doc(alias = "get_longname")]
    fn longname(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_longname(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_rank")]
    #[doc(alias = "get_rank")]
    fn rank(&self) -> u32 {
        unsafe {
            ffi::ibus_engine_desc_get_rank(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "ibus_engine_desc_get_setup")]
    #[doc(alias = "get_setup")]
    fn setup(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_setup(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_symbol")]
    #[doc(alias = "get_symbol")]
    fn symbol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_symbol(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_textdomain")]
    #[doc(alias = "get_textdomain")]
    fn textdomain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_textdomain(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "ibus_engine_desc_get_version")]
    #[doc(alias = "get_version")]
    fn version(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_engine_desc_get_version(self.as_ref().to_glib_none().0))
        }
    }

    //#[doc(alias = "ibus_engine_desc_output")]
    //fn output(&self, output: /*Ignored*/&mut glib::String, indent: i32) {
    //    unsafe { TODO: call ffi:ibus_engine_desc_output() }
    //}
}

impl<O: IsA<EngineDesc>> EngineDescExt for O {}
