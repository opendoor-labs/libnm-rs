// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::object::Cast;
use glib::translate::*;
#[cfg(any(feature = "v1_20", feature = "dox"))]
use glib::GString;
use nm_sys;
use std::fmt;
#[cfg(any(feature = "v1_20", feature = "dox"))]
use std::mem;
use Setting;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use Ternary;

glib_wrapper! {
    pub struct SettingEthtool(Object<nm_sys::NMSettingEthtool, nm_sys::NMSettingEthtoolClass, SettingEthtoolClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_ethtool_get_type(),
    }
}

impl SettingEthtool {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new() -> SettingEthtool {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_ethtool_new()).unsafe_cast() }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn clear_features(&self) {
        unsafe {
            nm_sys::nm_setting_ethtool_clear_features(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_feature(&self, optname: &str) -> Ternary {
        unsafe {
            from_glib(nm_sys::nm_setting_ethtool_get_feature(
                self.to_glib_none().0,
                optname.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn get_optnames(&self) -> (Vec<GString>, u32) {
        unsafe {
            let mut out_length = mem::MaybeUninit::uninit();
            let ret =
                FromGlibPtrContainer::from_glib_container(nm_sys::nm_setting_ethtool_get_optnames(
                    self.to_glib_none().0,
                    out_length.as_mut_ptr(),
                ));
            let out_length = out_length.assume_init();
            (ret, out_length)
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn set_feature(&self, optname: &str, value: Ternary) {
        unsafe {
            nm_sys::nm_setting_ethtool_set_feature(
                self.to_glib_none().0,
                optname.to_glib_none().0,
                value.to_glib(),
            );
        }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
impl Default for SettingEthtool {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingEthtool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingEthtool")
    }
}
