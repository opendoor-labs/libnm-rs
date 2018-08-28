// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::Value;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Setting;

glib_wrapper! {
    pub struct SettingIP4Config(Object<ffi::NMSettingIP4Config, ffi::NMSettingIP4ConfigClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_ip4_config_get_type(),
    }
}

impl SettingIP4Config {
    pub fn new() -> SettingIP4Config {
        unsafe { Setting::from_glib_full(ffi::nm_setting_ip4_config_new()).downcast_unchecked() }
    }
}

impl Default for SettingIP4Config {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingIP4ConfigExt {
    fn get_dhcp_client_id(&self) -> Option<String>;

    fn get_dhcp_fqdn(&self) -> Option<String>;

    fn set_property_dhcp_client_id(&self, dhcp_client_id: Option<&str>);

    fn set_property_dhcp_fqdn(&self, dhcp_fqdn: Option<&str>);

    fn connect_property_dhcp_client_id_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_dhcp_fqdn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingIP4Config> + IsA<glib::object::Object>> SettingIP4ConfigExt for O {
    fn get_dhcp_client_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_ip4_config_get_dhcp_client_id(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_dhcp_fqdn(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_ip4_config_get_dhcp_fqdn(
                self.to_glib_none().0,
            ))
        }
    }

    fn set_property_dhcp_client_id(&self, dhcp_client_id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "dhcp-client-id".to_glib_none().0,
                Value::from(dhcp_client_id).to_glib_none().0,
            );
        }
    }

    fn set_property_dhcp_fqdn(&self, dhcp_fqdn: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "dhcp-fqdn".to_glib_none().0,
                Value::from(dhcp_fqdn).to_glib_none().0,
            );
        }
    }

    fn connect_property_dhcp_client_id_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::dhcp-client-id",
                transmute(notify_dhcp_client_id_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_dhcp_fqdn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::dhcp-fqdn",
                transmute(notify_dhcp_fqdn_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_dhcp_client_id_trampoline<P>(
    this: *mut ffi::NMSettingIP4Config,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingIP4Config>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingIP4Config::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_dhcp_fqdn_trampoline<P>(
    this: *mut ffi::NMSettingIP4Config,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingIP4Config>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingIP4Config::from_glib_borrow(this).downcast_unchecked())
}