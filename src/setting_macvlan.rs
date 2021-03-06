// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

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
use glib::Value;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use gobject_sys;
use nm_sys;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::mem::transmute;
use Setting;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use SettingMacvlanMode;

glib_wrapper! {
    pub struct SettingMacvlan(Object<nm_sys::NMSettingMacvlan, nm_sys::NMSettingMacvlanClass, SettingMacvlanClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_macvlan_get_type(),
    }
}

impl SettingMacvlan {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn new() -> SettingMacvlan {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_macvlan_new()).unsafe_cast() }
    }
}

#[cfg(any(feature = "v1_2", feature = "dox"))]
impl Default for SettingMacvlan {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_MACVLAN: Option<&SettingMacvlan> = None;

pub trait SettingMacvlanExt: 'static {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_mode(&self) -> SettingMacvlanMode;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_parent(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_promiscuous(&self) -> bool;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_tap(&self) -> bool;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_mode(&self, mode: u32);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_parent(&self, parent: Option<&str>);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_promiscuous(&self, promiscuous: bool);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_tap(&self, tap: bool);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_promiscuous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_tap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingMacvlan>> SettingMacvlanExt for O {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_mode(&self) -> SettingMacvlanMode {
        unsafe {
            from_glib(nm_sys::nm_setting_macvlan_get_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_parent(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_macvlan_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_promiscuous(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_macvlan_get_promiscuous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_tap(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_macvlan_get_tap(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_mode(&self, mode: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mode\0".as_ptr() as *const _,
                Value::from(&mode).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_parent(&self, parent: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"parent\0".as_ptr() as *const _,
                Value::from(parent).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_promiscuous(&self, promiscuous: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"promiscuous\0".as_ptr() as *const _,
                Value::from(&promiscuous).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_tap(&self, tap: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"tap\0".as_ptr() as *const _,
                Value::from(&tap).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingMacvlan,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingMacvlan>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingMacvlan::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute(notify_mode_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingMacvlan,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingMacvlan>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingMacvlan::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute(notify_parent_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_promiscuous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_promiscuous_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingMacvlan,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingMacvlan>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingMacvlan::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::promiscuous\0".as_ptr() as *const _,
                Some(transmute(notify_promiscuous_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_tap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tap_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingMacvlan,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingMacvlan>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingMacvlan::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tap\0".as_ptr() as *const _,
                Some(transmute(notify_tap_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingMacvlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingMacvlan")
    }
}
