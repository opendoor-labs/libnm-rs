// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_16", feature = "dox"))]
use _80211ApFlags;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::object::IsA;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib_sys;
use nm_sys;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use Connection;
use Object;

glib_wrapper! {
    pub struct WifiP2PPeer(Object<nm_sys::NMWifiP2PPeer, nm_sys::NMWifiP2PPeerClass, WifiP2PPeerClass>) @extends Object;

    match fn {
        get_type => || nm_sys::nm_wifi_p2p_peer_get_type(),
    }
}

impl WifiP2PPeer {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connection_valid<P: IsA<Connection>>(&self, connection: &P) -> bool {
        unsafe {
            from_glib(nm_sys::nm_wifi_p2p_peer_connection_valid(
                self.to_glib_none().0,
                connection.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn filter_connections(&self, connections: &[Connection]) -> Vec<Connection> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(nm_sys::nm_wifi_p2p_peer_filter_connections(
                self.to_glib_none().0,
                connections.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_flags(&self) -> _80211ApFlags {
        unsafe { from_glib(nm_sys::nm_wifi_p2p_peer_get_flags(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_hw_address(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_wifi_p2p_peer_get_hw_address(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_last_seen(&self) -> i32 {
        unsafe { nm_sys::nm_wifi_p2p_peer_get_last_seen(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_manufacturer(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_wifi_p2p_peer_get_manufacturer(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_model(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_wifi_p2p_peer_get_model(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_model_number(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_wifi_p2p_peer_get_model_number(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_name(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_wifi_p2p_peer_get_name(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_serial(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_wifi_p2p_peer_get_serial(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_strength(&self) -> u8 {
        unsafe { nm_sys::nm_wifi_p2p_peer_get_strength(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_wfd_ies(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_none(nm_sys::nm_wifi_p2p_peer_get_wfd_ies(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_flags_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute(notify_flags_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_hw_address_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_hw_address_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hw-address\0".as_ptr() as *const _,
                Some(transmute(notify_hw_address_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_last_seen_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_last_seen_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::last-seen\0".as_ptr() as *const _,
                Some(transmute(notify_last_seen_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_manufacturer_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_manufacturer_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::manufacturer\0".as_ptr() as *const _,
                Some(transmute(notify_manufacturer_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_model_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute(notify_model_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_model_number_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_number_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model-number\0".as_ptr() as *const _,
                Some(transmute(notify_model_number_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_name_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute(notify_name_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_serial_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_serial_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::serial\0".as_ptr() as *const _,
                Some(transmute(notify_serial_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_strength_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_strength_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::strength\0".as_ptr() as *const _,
                Some(transmute(notify_strength_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_wfd_ies_notify<F: Fn(&WifiP2PPeer) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_wfd_ies_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut nm_sys::NMWifiP2PPeer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wfd-ies\0".as_ptr() as *const _,
                Some(transmute(notify_wfd_ies_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for WifiP2PPeer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WifiP2PPeer")
    }
}
