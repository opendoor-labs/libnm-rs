// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Device;
use Object;

glib_wrapper! {
    pub struct DeviceEthernet(Object<nm_sys::NMDeviceEthernet, nm_sys::NMDeviceEthernetClass, DeviceEthernetClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_ethernet_get_type(),
    }
}

impl DeviceEthernet {
    pub fn get_carrier(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_device_ethernet_get_carrier(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_permanent_hw_address(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_device_ethernet_get_permanent_hw_address(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_s390_subchannels(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(nm_sys::nm_device_ethernet_get_s390_subchannels(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_speed(&self) -> u32 {
        unsafe { nm_sys::nm_device_ethernet_get_speed(self.to_glib_none().0) }
    }

    pub fn get_property_perm_hw_address(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"perm-hw-address\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `perm-hw-address` getter")
        }
    }

    pub fn connect_property_carrier_notify<F: Fn(&DeviceEthernet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceEthernet) + 'static>(
            this: *mut nm_sys::NMDeviceEthernet,
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
                b"notify::carrier\0".as_ptr() as *const _,
                Some(transmute(notify_carrier_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_hw_address_notify<F: Fn(&DeviceEthernet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_hw_address_trampoline<F: Fn(&DeviceEthernet) + 'static>(
            this: *mut nm_sys::NMDeviceEthernet,
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

    pub fn connect_property_perm_hw_address_notify<F: Fn(&DeviceEthernet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_perm_hw_address_trampoline<F: Fn(&DeviceEthernet) + 'static>(
            this: *mut nm_sys::NMDeviceEthernet,
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
                b"notify::perm-hw-address\0".as_ptr() as *const _,
                Some(transmute(notify_perm_hw_address_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_s390_subchannels_notify<F: Fn(&DeviceEthernet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_s390_subchannels_trampoline<
            F: Fn(&DeviceEthernet) + 'static,
        >(
            this: *mut nm_sys::NMDeviceEthernet,
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
                b"notify::s390-subchannels\0".as_ptr() as *const _,
                Some(transmute(notify_s390_subchannels_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_speed_notify<F: Fn(&DeviceEthernet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_speed_trampoline<F: Fn(&DeviceEthernet) + 'static>(
            this: *mut nm_sys::NMDeviceEthernet,
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
                b"notify::speed\0".as_ptr() as *const _,
                Some(transmute(notify_speed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceEthernet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceEthernet")
    }
}
