// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use libc;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use ActivationStateFlags;
use ActiveConnectionState;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use ActiveConnectionStateReason;
use Device;
use DhcpConfig;
use IPConfig;
use Object;
use RemoteConnection;

glib_wrapper! {
    pub struct ActiveConnection(Object<nm_sys::NMActiveConnection, nm_sys::NMActiveConnectionClass, ActiveConnectionClass>) @extends Object;

    match fn {
        get_type => || nm_sys::nm_active_connection_get_type(),
    }
}

pub const NONE_ACTIVE_CONNECTION: Option<&ActiveConnection> = None;

pub trait ActiveConnectionExt: 'static {
    fn get_connection(&self) -> Option<RemoteConnection>;

    fn get_connection_type(&self) -> Option<GString>;

    fn get_default(&self) -> bool;

    fn get_default6(&self) -> bool;

    fn get_devices(&self) -> Vec<Device>;

    fn get_dhcp4_config(&self) -> Option<DhcpConfig>;

    fn get_dhcp6_config(&self) -> Option<DhcpConfig>;

    fn get_id(&self) -> Option<GString>;

    fn get_ip4_config(&self) -> Option<IPConfig>;

    fn get_ip6_config(&self) -> Option<IPConfig>;

    fn get_master(&self) -> Option<Device>;

    fn get_specific_object_path(&self) -> Option<GString>;

    fn get_state(&self) -> ActiveConnectionState;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_state_flags(&self) -> ActivationStateFlags;

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn get_state_reason(&self) -> ActiveConnectionStateReason;

    fn get_uuid(&self) -> Option<GString>;

    fn get_vpn(&self) -> bool;

    fn get_property_type(&self) -> Option<GString>;

    fn connect_state_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_connection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default6_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_devices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_dhcp4_config_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_dhcp6_config_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ip4_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ip6_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_master_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_specific_object_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_state_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uuid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vpn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ActiveConnection>> ActiveConnectionExt for O {
    fn get_connection(&self) -> Option<RemoteConnection> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_connection_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_default(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_default(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_default6(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_default6(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(nm_sys::nm_active_connection_get_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_dhcp4_config(&self) -> Option<DhcpConfig> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_dhcp4_config(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_dhcp6_config(&self) -> Option<DhcpConfig> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_dhcp6_config(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_ip4_config(&self) -> Option<IPConfig> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_ip4_config(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_ip6_config(&self) -> Option<IPConfig> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_ip6_config(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_master(&self) -> Option<Device> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_master(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_specific_object_path(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_specific_object_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_state(&self) -> ActiveConnectionState {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_state_flags(&self) -> ActivationStateFlags {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_state_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn get_state_reason(&self) -> ActiveConnectionStateReason {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_state_reason(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_uuid(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_active_connection_get_uuid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_vpn(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_active_connection_get_vpn(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
        }
    }

    fn connect_state_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_changed_trampoline<P, F: Fn(&P, u32, u32) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            state: libc::c_uint,
            reason: libc::c_uint,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ActiveConnection::from_glib_borrow(this).unsafe_cast(),
                state,
                reason,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"state-changed\0".as_ptr() as *const _,
                Some(transmute(state_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_connection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_connection_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::connection\0".as_ptr() as *const _,
                Some(transmute(notify_connection_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default\0".as_ptr() as *const _,
                Some(transmute(notify_default_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_default6_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default6_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default6\0".as_ptr() as *const _,
                Some(transmute(notify_default6_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_devices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_devices_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::devices\0".as_ptr() as *const _,
                Some(transmute(notify_devices_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_dhcp4_config_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_dhcp4_config_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dhcp4-config\0".as_ptr() as *const _,
                Some(transmute(
                    notify_dhcp4_config_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_dhcp6_config_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_dhcp6_config_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dhcp6-config\0".as_ptr() as *const _,
                Some(transmute(
                    notify_dhcp6_config_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute(notify_id_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ip4_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip4_config_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ip4-config\0".as_ptr() as *const _,
                Some(transmute(notify_ip4_config_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ip6_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip6_config_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ip6-config\0".as_ptr() as *const _,
                Some(transmute(notify_ip6_config_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_master_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_master_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::master\0".as_ptr() as *const _,
                Some(transmute(notify_master_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_specific_object_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_specific_object_path_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::specific-object-path\0".as_ptr() as *const _,
                Some(transmute(
                    notify_specific_object_path_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state\0".as_ptr() as *const _,
                Some(transmute(notify_state_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_state_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_flags_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state-flags\0".as_ptr() as *const _,
                Some(transmute(notify_state_flags_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute(notify_type_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_uuid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uuid_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::uuid\0".as_ptr() as *const _,
                Some(transmute(notify_uuid_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_vpn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vpn_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMActiveConnection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActiveConnection>,
        {
            let f: &F = &*(f as *const F);
            f(&ActiveConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vpn\0".as_ptr() as *const _,
                Some(transmute(notify_vpn_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ActiveConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActiveConnection")
    }
}
