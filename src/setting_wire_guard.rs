// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::object::Cast;
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
use glib::Value;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use gobject_sys;
use nm_sys;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::mem;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::mem::transmute;
use Setting;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use SettingSecretFlags;
#[cfg(any(feature = "v1_20", feature = "dox"))]
use Ternary;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use WireGuardPeer;

glib_wrapper! {
    pub struct SettingWireGuard(Object<nm_sys::NMSettingWireGuard, nm_sys::NMSettingWireGuardClass, SettingWireGuardClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_wireguard_get_type(),
    }
}

impl SettingWireGuard {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn new() -> SettingWireGuard {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_wireguard_new()).unsafe_cast() }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn append_peer(&self, peer: &WireGuardPeer) {
        unsafe {
            nm_sys::nm_setting_wireguard_append_peer(self.to_glib_none().0, peer.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn clear_peers(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireguard_clear_peers(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_fwmark(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireguard_get_fwmark(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn get_ip4_auto_default_route(&self) -> Ternary {
        unsafe {
            from_glib(nm_sys::nm_setting_wireguard_get_ip4_auto_default_route(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn get_ip6_auto_default_route(&self) -> Ternary {
        unsafe {
            from_glib(nm_sys::nm_setting_wireguard_get_ip6_auto_default_route(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_listen_port(&self) -> u16 {
        unsafe { nm_sys::nm_setting_wireguard_get_listen_port(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_mtu(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireguard_get_mtu(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_peer(&self, idx: u32) -> Option<WireGuardPeer> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireguard_get_peer(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_peer_by_public_key(&self, public_key: &str) -> (WireGuardPeer, u32) {
        unsafe {
            let mut out_idx = mem::MaybeUninit::uninit();
            let ret = from_glib_none(nm_sys::nm_setting_wireguard_get_peer_by_public_key(
                self.to_glib_none().0,
                public_key.to_glib_none().0,
                out_idx.as_mut_ptr(),
            ));
            let out_idx = out_idx.assume_init();
            (ret, out_idx)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_peer_routes(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_wireguard_get_peer_routes(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_peers_len(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireguard_get_peers_len(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_private_key(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireguard_get_private_key(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_private_key_flags(&self) -> SettingSecretFlags {
        unsafe {
            from_glib(nm_sys::nm_setting_wireguard_get_private_key_flags(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn remove_peer(&self, idx: u32) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_wireguard_remove_peer(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_peer(&self, peer: &WireGuardPeer, idx: u32) {
        unsafe {
            nm_sys::nm_setting_wireguard_set_peer(
                self.to_glib_none().0,
                peer.to_glib_none().0,
                idx,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_property_fwmark(&self, fwmark: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"fwmark\0".as_ptr() as *const _,
                Value::from(&fwmark).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn set_property_ip4_auto_default_route(&self, ip4_auto_default_route: Ternary) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"ip4-auto-default-route\0".as_ptr() as *const _,
                Value::from(&ip4_auto_default_route).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn set_property_ip6_auto_default_route(&self, ip6_auto_default_route: Ternary) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"ip6-auto-default-route\0".as_ptr() as *const _,
                Value::from(&ip6_auto_default_route).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_property_listen_port(&self, listen_port: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"listen-port\0".as_ptr() as *const _,
                Value::from(&listen_port).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_property_mtu(&self, mtu: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"mtu\0".as_ptr() as *const _,
                Value::from(&mtu).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_property_peer_routes(&self, peer_routes: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"peer-routes\0".as_ptr() as *const _,
                Value::from(&peer_routes).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_property_private_key(&self, private_key: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"private-key\0".as_ptr() as *const _,
                Value::from(private_key).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_property_private_key_flags(&self, private_key_flags: SettingSecretFlags) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"private-key-flags\0".as_ptr() as *const _,
                Value::from(&private_key_flags).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_fwmark_notify<F: Fn(&SettingWireGuard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_fwmark_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut nm_sys::NMSettingWireGuard,
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
                b"notify::fwmark\0".as_ptr() as *const _,
                Some(transmute(notify_fwmark_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn connect_property_ip4_auto_default_route_notify<F: Fn(&SettingWireGuard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip4_auto_default_route_trampoline<
            F: Fn(&SettingWireGuard) + 'static,
        >(
            this: *mut nm_sys::NMSettingWireGuard,
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
                b"notify::ip4-auto-default-route\0".as_ptr() as *const _,
                Some(transmute(
                    notify_ip4_auto_default_route_trampoline::<F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn connect_property_ip6_auto_default_route_notify<F: Fn(&SettingWireGuard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip6_auto_default_route_trampoline<
            F: Fn(&SettingWireGuard) + 'static,
        >(
            this: *mut nm_sys::NMSettingWireGuard,
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
                b"notify::ip6-auto-default-route\0".as_ptr() as *const _,
                Some(transmute(
                    notify_ip6_auto_default_route_trampoline::<F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_listen_port_notify<F: Fn(&SettingWireGuard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_listen_port_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut nm_sys::NMSettingWireGuard,
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
                b"notify::listen-port\0".as_ptr() as *const _,
                Some(transmute(notify_listen_port_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_mtu_notify<F: Fn(&SettingWireGuard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mtu_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut nm_sys::NMSettingWireGuard,
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
                b"notify::mtu\0".as_ptr() as *const _,
                Some(transmute(notify_mtu_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_peer_routes_notify<F: Fn(&SettingWireGuard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_peer_routes_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut nm_sys::NMSettingWireGuard,
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
                b"notify::peer-routes\0".as_ptr() as *const _,
                Some(transmute(notify_peer_routes_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_private_key_notify<F: Fn(&SettingWireGuard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_private_key_trampoline<F: Fn(&SettingWireGuard) + 'static>(
            this: *mut nm_sys::NMSettingWireGuard,
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
                b"notify::private-key\0".as_ptr() as *const _,
                Some(transmute(notify_private_key_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_private_key_flags_notify<F: Fn(&SettingWireGuard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_private_key_flags_trampoline<
            F: Fn(&SettingWireGuard) + 'static,
        >(
            this: *mut nm_sys::NMSettingWireGuard,
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
                b"notify::private-key-flags\0".as_ptr() as *const _,
                Some(transmute(notify_private_key_flags_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
impl Default for SettingWireGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingWireGuard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingWireGuard")
    }
}
