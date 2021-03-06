// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib_sys;
use nm_sys;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::ptr;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use VpnEditorPlugin;

glib_wrapper! {
    pub struct VpnPluginInfo(Object<nm_sys::NMVpnPluginInfo, nm_sys::NMVpnPluginInfoClass, VpnPluginInfoClass>);

    match fn {
        get_type => || nm_sys::nm_vpn_plugin_info_get_type(),
    }
}

impl VpnPluginInfo {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn new_from_file(filename: &str) -> Result<VpnPluginInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                nm_sys::nm_vpn_plugin_info_new_from_file(filename.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn new_search_file(name: Option<&str>, service: Option<&str>) -> VpnPluginInfo {
        unsafe {
            from_glib_full(nm_sys::nm_vpn_plugin_info_new_search_file(
                name.to_glib_none().0,
                service.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn new_with_data(
        filename: &str,
        keyfile: &glib::KeyFile,
    ) -> Result<VpnPluginInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_vpn_plugin_info_new_with_data(
                filename.to_glib_none().0,
                keyfile.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn validate_filename(filename: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_vpn_plugin_info_validate_filename(
                filename.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_VPN_PLUGIN_INFO: Option<&VpnPluginInfo> = None;

pub trait VpnPluginInfoExt: 'static {
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_aliases(&self) -> Vec<GString>;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_auth_dialog(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_editor_plugin(&self) -> Option<VpnEditorPlugin>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_filename(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_name(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_plugin(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_program(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_service(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn load_editor_plugin(&self) -> Result<VpnEditorPlugin, glib::Error>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn lookup_property(&self, group: &str, key: &str) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_editor_plugin<P: IsA<VpnEditorPlugin>>(&self, plugin: Option<&P>);

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn supports_hints(&self) -> bool;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn supports_multiple(&self) -> bool;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<VpnPluginInfo>> VpnPluginInfoExt for O {
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_aliases(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(nm_sys::nm_vpn_plugin_info_get_aliases(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_auth_dialog(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_auth_dialog(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_editor_plugin(&self) -> Option<VpnEditorPlugin> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_editor_plugin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_filename(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_filename(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_plugin(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_plugin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_program(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_program(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_service(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_get_service(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn load_editor_plugin(&self) -> Result<VpnEditorPlugin, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_vpn_plugin_info_load_editor_plugin(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn lookup_property(&self, group: &str, key: &str) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_plugin_info_lookup_property(
                self.as_ref().to_glib_none().0,
                group.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_editor_plugin<P: IsA<VpnEditorPlugin>>(&self, plugin: Option<&P>) {
        unsafe {
            nm_sys::nm_vpn_plugin_info_set_editor_plugin(
                self.as_ref().to_glib_none().0,
                plugin.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn supports_hints(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_vpn_plugin_info_supports_hints(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn supports_multiple(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_vpn_plugin_info_supports_multiple(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMVpnPluginInfo,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnPluginInfo>,
        {
            let f: &F = &*(f as *const F);
            f(&VpnPluginInfo::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute(notify_name_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for VpnPluginInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VpnPluginInfo")
    }
}
