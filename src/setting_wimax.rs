// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::Value;
use glib_sys;
use gobject_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Setting;

glib_wrapper! {
    pub struct SettingWimax(Object<nm_sys::NMSettingWimax, nm_sys::NMSettingWimaxClass, SettingWimaxClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_wimax_get_type(),
    }
}

impl SettingWimax {
    #[cfg_attr(feature = "v1_2", deprecated)]
    pub fn new() -> SettingWimax {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_wimax_new()).unsafe_cast() }
    }
}

#[cfg_attr(feature = "v1_2", deprecated)]
impl Default for SettingWimax {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_WIMAX: Option<&SettingWimax> = None;

pub trait SettingWimaxExt: 'static {
    #[cfg_attr(feature = "v1_2", deprecated)]
    fn get_mac_address(&self) -> Option<GString>;

    #[cfg_attr(feature = "v1_2", deprecated)]
    fn get_network_name(&self) -> Option<GString>;

    #[cfg_attr(feature = "v1_2", deprecated)]
    fn set_property_mac_address(&self, mac_address: Option<&str>);

    #[cfg_attr(feature = "v1_2", deprecated)]
    fn set_property_network_name(&self, network_name: Option<&str>);

    #[cfg_attr(feature = "v1_2", deprecated)]
    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v1_2", deprecated)]
    fn connect_property_network_name_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<SettingWimax>> SettingWimaxExt for O {
    fn get_mac_address(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wimax_get_mac_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_network_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wimax_get_network_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_property_mac_address(&self, mac_address: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mac-address\0".as_ptr() as *const _,
                Value::from(mac_address).to_glib_none().0,
            );
        }
    }

    fn set_property_network_name(&self, network_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"network-name\0".as_ptr() as *const _,
                Value::from(network_name).to_glib_none().0,
            );
        }
    }

    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mac_address_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWimax,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWimax>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWimax::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mac-address\0".as_ptr() as *const _,
                Some(transmute(notify_mac_address_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_network_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_network_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWimax,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWimax>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWimax::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::network-name\0".as_ptr() as *const _,
                Some(transmute(
                    notify_network_name_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingWimax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingWimax")
    }
}
