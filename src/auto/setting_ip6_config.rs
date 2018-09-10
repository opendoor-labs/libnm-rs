// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
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
use SettingIP6ConfigAddrGenMode;
use SettingIP6ConfigPrivacy;
use SettingIPConfig;

glib_wrapper! {
    pub struct SettingIP6Config(Object<ffi::NMSettingIP6Config, ffi::NMSettingIP6ConfigClass>): SettingIPConfig, Setting;

    match fn {
        get_type => || ffi::nm_setting_ip6_config_get_type(),
    }
}

impl SettingIP6Config {
    pub fn new() -> SettingIP6Config {
        unsafe { Setting::from_glib_full(ffi::nm_setting_ip6_config_new()).downcast_unchecked() }
    }
}

impl Default for SettingIP6Config {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingIP6ConfigExt {
    fn get_addr_gen_mode(&self) -> SettingIP6ConfigAddrGenMode;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_dhcp_duid(&self) -> Option<String>;

    fn get_ip6_privacy(&self) -> SettingIP6ConfigPrivacy;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_token(&self) -> Option<String>;

    fn set_property_addr_gen_mode(&self, addr_gen_mode: i32);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_dhcp_duid(&self, dhcp_duid: Option<&str>);

    fn set_property_ip6_privacy(&self, ip6_privacy: SettingIP6ConfigPrivacy);

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_property_token(&self, token: Option<&str>);

    fn connect_property_addr_gen_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_dhcp_duid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ip6_privacy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn connect_property_token_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingIP6Config> + IsA<glib::object::Object>> SettingIP6ConfigExt for O {
    fn get_addr_gen_mode(&self) -> SettingIP6ConfigAddrGenMode {
        unsafe {
            from_glib(ffi::nm_setting_ip6_config_get_addr_gen_mode(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_dhcp_duid(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_ip6_config_get_dhcp_duid(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_ip6_privacy(&self) -> SettingIP6ConfigPrivacy {
        unsafe {
            from_glib(ffi::nm_setting_ip6_config_get_ip6_privacy(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_token(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_ip6_config_get_token(self.to_glib_none().0)) }
    }

    fn set_property_addr_gen_mode(&self, addr_gen_mode: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "addr-gen-mode".to_glib_none().0,
                Value::from(&addr_gen_mode).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_dhcp_duid(&self, dhcp_duid: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "dhcp-duid".to_glib_none().0,
                Value::from(dhcp_duid).to_glib_none().0,
            );
        }
    }

    fn set_property_ip6_privacy(&self, ip6_privacy: SettingIP6ConfigPrivacy) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "ip6-privacy".to_glib_none().0,
                Value::from(&ip6_privacy).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_property_token(&self, token: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "token".to_glib_none().0,
                Value::from(token).to_glib_none().0,
            );
        }
    }

    fn connect_property_addr_gen_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::addr-gen-mode",
                transmute(notify_addr_gen_mode_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_dhcp_duid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::dhcp-duid",
                transmute(notify_dhcp_duid_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_ip6_privacy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::ip6-privacy",
                transmute(notify_ip6_privacy_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn connect_property_token_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::token",
                transmute(notify_token_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_addr_gen_mode_trampoline<P>(
    this: *mut ffi::NMSettingIP6Config,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingIP6Config>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingIP6Config::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn notify_dhcp_duid_trampoline<P>(
    this: *mut ffi::NMSettingIP6Config,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingIP6Config>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingIP6Config::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ip6_privacy_trampoline<P>(
    this: *mut ffi::NMSettingIP6Config,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingIP6Config>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingIP6Config::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_4", feature = "dox"))]
unsafe extern "C" fn notify_token_trampoline<P>(
    this: *mut ffi::NMSettingIP6Config,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingIP6Config>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingIP6Config::from_glib_borrow(this).downcast_unchecked())
}
