#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::let_and_return))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::new_ret_no_self))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::derive_hash_xor_eq))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::should_implement_trait))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::missing_safety_doc))]

#[macro_use]
extern crate bitflags;

extern crate once_cell;

extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
#[macro_use]
extern crate glib;
extern crate gio;
extern crate libc;

#[cfg(feature = "futures")]
extern crate fragile;

#[cfg(feature = "futures")]
extern crate futures_core;

extern crate nm_sys;

pub use gio::NONE_CANCELLABLE;

pub use glib::prelude::*;

pub use functions::*;
pub use traits::*;

mod access_point;
pub use self::access_point::{AccessPoint, AccessPointClass};

mod active_connection;
pub use self::active_connection::ActiveConnectionExt;
pub use self::active_connection::{
    ActiveConnection, ActiveConnectionClass, NONE_ACTIVE_CONNECTION,
};

#[cfg(any(feature = "v1_12", feature = "dox"))]
mod checkpoint;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::checkpoint::{Checkpoint, CheckpointClass};

mod client;
pub use self::client::{Client, ClientClass};

mod connection;
pub use self::connection::ConnectionExt;
pub use self::connection::{Connection, NONE_CONNECTION};

mod device;
pub use self::device::DeviceExt;
pub use self::device::{Device, DeviceClass, NONE_DEVICE};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod device6_lowpan;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::device6_lowpan::{Device6Lowpan, Device6LowpanClass};

mod device_adsl;
pub use self::device_adsl::{DeviceAdsl, DeviceAdslClass};

mod device_bond;
pub use self::device_bond::{DeviceBond, DeviceBondClass};

mod device_bridge;
pub use self::device_bridge::{DeviceBridge, DeviceBridgeClass};

mod device_bt;
pub use self::device_bt::{DeviceBt, DeviceBtClass};

mod device_dummy;
pub use self::device_dummy::{DeviceDummy, DeviceDummyClass};

mod device_ethernet;
pub use self::device_ethernet::{DeviceEthernet, DeviceEthernetClass};

mod device_generic;
pub use self::device_generic::{DeviceGeneric, DeviceGenericClass};

#[cfg(any(feature = "v1_2", feature = "dox"))]
mod device_ip_tunnel;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::device_ip_tunnel::{DeviceIPTunnel, DeviceIPTunnelClass};

mod device_infiniband;
pub use self::device_infiniband::{DeviceInfiniband, DeviceInfinibandClass};

#[cfg(any(feature = "v1_6", feature = "dox"))]
mod device_macsec;
#[cfg(any(feature = "v1_6", feature = "dox"))]
pub use self::device_macsec::{DeviceMacsec, DeviceMacsecClass};

#[cfg(any(feature = "v1_2", feature = "dox"))]
mod device_macvlan;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::device_macvlan::{DeviceMacvlan, DeviceMacvlanClass};

mod device_modem;
pub use self::device_modem::{DeviceModem, DeviceModemClass};

mod device_olpc_mesh;
pub use self::device_olpc_mesh::{DeviceOlpcMesh, DeviceOlpcMeshClass};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod device_ovs_bridge;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::device_ovs_bridge::{DeviceOvsBridge, DeviceOvsBridgeClass};

mod device_ovs_interface;
pub use self::device_ovs_interface::{DeviceOvsInterface, DeviceOvsInterfaceClass};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod device_ovs_port;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::device_ovs_port::{DeviceOvsPort, DeviceOvsPortClass};

mod device_ppp;
pub use self::device_ppp::{DevicePpp, DevicePppClass};

mod device_team;
pub use self::device_team::{DeviceTeam, DeviceTeamClass};

mod device_tun;
pub use self::device_tun::{DeviceTun, DeviceTunClass};

mod device_vlan;
pub use self::device_vlan::{DeviceVlan, DeviceVlanClass};

#[cfg(any(feature = "v1_2", feature = "dox"))]
mod device_vxlan;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::device_vxlan::{DeviceVxlan, DeviceVxlanClass};

mod device_wifi;
pub use self::device_wifi::{DeviceWifi, DeviceWifiClass};

#[cfg(any(feature = "v1_16", feature = "dox"))]
mod device_wifi_p2p;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use self::device_wifi_p2p::{DeviceWifiP2P, DeviceWifiP2PClass};

mod device_wimax;
pub use self::device_wimax::{DeviceWimax, DeviceWimaxClass};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod device_wire_guard;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::device_wire_guard::{DeviceWireGuard, DeviceWireGuardClass};

mod dhcp_config;
pub use self::dhcp_config::{DhcpConfig, DhcpConfigClass};

mod ip_config;
pub use self::ip_config::{IPConfig, IPConfigClass};

mod object;
pub use self::object::ObjectExt;
pub use self::object::{Object, ObjectClass, NONE_OBJECT};

mod remote_connection;
pub use self::remote_connection::{RemoteConnection, RemoteConnectionClass};

mod secret_agent_old;
pub use self::secret_agent_old::SecretAgentOldExt;
pub use self::secret_agent_old::{SecretAgentOld, SecretAgentOldClass, NONE_SECRET_AGENT_OLD};

mod setting;
pub use self::setting::SettingExt;
pub use self::setting::{Setting, SettingClass, NONE_SETTING};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod setting6_lowpan;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::setting6_lowpan::{Setting6Lowpan, Setting6LowpanClass};

mod setting8021x;
pub use self::setting8021x::Setting8021xExt;
pub use self::setting8021x::{Setting8021x, Setting8021xClass, NONE_SETTING8021X};

mod setting_adsl;
pub use self::setting_adsl::SettingAdslExt;
pub use self::setting_adsl::{SettingAdsl, SettingAdslClass, NONE_SETTING_ADSL};

mod setting_bluetooth;
pub use self::setting_bluetooth::SettingBluetoothExt;
pub use self::setting_bluetooth::{
    SettingBluetooth, SettingBluetoothClass, NONE_SETTING_BLUETOOTH,
};

mod setting_bond;
pub use self::setting_bond::SettingBondExt;
pub use self::setting_bond::{SettingBond, SettingBondClass, NONE_SETTING_BOND};

mod setting_bridge;
pub use self::setting_bridge::SettingBridgeExt;
pub use self::setting_bridge::{SettingBridge, SettingBridgeClass, NONE_SETTING_BRIDGE};

mod setting_bridge_port;
pub use self::setting_bridge_port::SettingBridgePortExt;
pub use self::setting_bridge_port::{
    SettingBridgePort, SettingBridgePortClass, NONE_SETTING_BRIDGE_PORT,
};

mod setting_cdma;
pub use self::setting_cdma::SettingCdmaExt;
pub use self::setting_cdma::{SettingCdma, SettingCdmaClass, NONE_SETTING_CDMA};

mod setting_connection;
pub use self::setting_connection::SettingConnectionExt;
pub use self::setting_connection::{
    SettingConnection, SettingConnectionClass, NONE_SETTING_CONNECTION,
};

mod setting_dcb;
pub use self::setting_dcb::SettingDcbExt;
pub use self::setting_dcb::{SettingDcb, SettingDcbClass, NONE_SETTING_DCB};

#[cfg(any(feature = "v1_8", feature = "dox"))]
mod setting_dummy;
#[cfg(any(feature = "v1_8", feature = "dox"))]
pub use self::setting_dummy::{SettingDummy, SettingDummyClass, NONE_SETTING_DUMMY};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod setting_ethtool;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::setting_ethtool::{SettingEthtool, SettingEthtoolClass};

mod setting_generic;
pub use self::setting_generic::{SettingGeneric, SettingGenericClass, NONE_SETTING_GENERIC};

mod setting_gsm;
pub use self::setting_gsm::SettingGsmExt;
pub use self::setting_gsm::{SettingGsm, SettingGsmClass, NONE_SETTING_GSM};

mod setting_ip4_config;
pub use self::setting_ip4_config::SettingIP4ConfigExt;
pub use self::setting_ip4_config::{
    SettingIP4Config, SettingIP4ConfigClass, NONE_SETTING_IP4_CONFIG,
};

mod setting_ip6_config;
pub use self::setting_ip6_config::SettingIP6ConfigExt;
pub use self::setting_ip6_config::{
    SettingIP6Config, SettingIP6ConfigClass, NONE_SETTING_IP6_CONFIG,
};

mod setting_ip_config;
pub use self::setting_ip_config::SettingIPConfigExt;
pub use self::setting_ip_config::{SettingIPConfig, SettingIPConfigClass, NONE_SETTING_IP_CONFIG};

mod setting_ip_tunnel;
pub use self::setting_ip_tunnel::SettingIPTunnelExt;
pub use self::setting_ip_tunnel::{SettingIPTunnel, SettingIPTunnelClass, NONE_SETTING_IP_TUNNEL};

mod setting_infiniband;
pub use self::setting_infiniband::SettingInfinibandExt;
pub use self::setting_infiniband::{
    SettingInfiniband, SettingInfinibandClass, NONE_SETTING_INFINIBAND,
};

#[cfg(any(feature = "v1_6", feature = "dox"))]
mod setting_macsec;
#[cfg(any(feature = "v1_6", feature = "dox"))]
pub use self::setting_macsec::SettingMacsecExt;
#[cfg(any(feature = "v1_6", feature = "dox"))]
pub use self::setting_macsec::{SettingMacsec, SettingMacsecClass, NONE_SETTING_MACSEC};

#[cfg(any(feature = "v1_2", feature = "dox"))]
mod setting_macvlan;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::setting_macvlan::SettingMacvlanExt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::setting_macvlan::{SettingMacvlan, SettingMacvlanClass, NONE_SETTING_MACVLAN};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod setting_match;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::setting_match::{SettingMatch, SettingMatchClass};

mod setting_olpc_mesh;
pub use self::setting_olpc_mesh::SettingOlpcMeshExt;
pub use self::setting_olpc_mesh::{SettingOlpcMesh, SettingOlpcMeshClass, NONE_SETTING_OLPC_MESH};

#[cfg(any(feature = "v1_10", feature = "dox"))]
mod setting_ovs_bridge;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub use self::setting_ovs_bridge::{SettingOvsBridge, SettingOvsBridgeClass};

#[cfg(any(feature = "v1_20", feature = "dox"))]
mod setting_ovs_dpdk;
#[cfg(any(feature = "v1_20", feature = "dox"))]
pub use self::setting_ovs_dpdk::{SettingOvsDpdk, SettingOvsDpdkClass};

#[cfg(any(feature = "v1_10", feature = "dox"))]
mod setting_ovs_interface;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub use self::setting_ovs_interface::{SettingOvsInterface, SettingOvsInterfaceClass};

#[cfg(any(feature = "v1_10", feature = "dox"))]
mod setting_ovs_patch;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub use self::setting_ovs_patch::{SettingOvsPatch, SettingOvsPatchClass};

#[cfg(any(feature = "v1_10", feature = "dox"))]
mod setting_ovs_port;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub use self::setting_ovs_port::{SettingOvsPort, SettingOvsPortClass};

mod setting_ppp;
pub use self::setting_ppp::SettingPppExt;
pub use self::setting_ppp::{SettingPpp, SettingPppClass, NONE_SETTING_PPP};

mod setting_pppoe;
pub use self::setting_pppoe::SettingPppoeExt;
pub use self::setting_pppoe::{SettingPppoe, SettingPppoeClass, NONE_SETTING_PPPOE};

#[cfg(any(feature = "v1_6", feature = "dox"))]
mod setting_proxy;
#[cfg(any(feature = "v1_6", feature = "dox"))]
pub use self::setting_proxy::SettingProxyExt;
#[cfg(any(feature = "v1_6", feature = "dox"))]
pub use self::setting_proxy::{SettingProxy, SettingProxyClass, NONE_SETTING_PROXY};

mod setting_serial;
pub use self::setting_serial::SettingSerialExt;
pub use self::setting_serial::{SettingSerial, SettingSerialClass, NONE_SETTING_SERIAL};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod setting_sriov;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::setting_sriov::{SettingSriov, SettingSriovClass};

#[cfg(any(feature = "v1_12", feature = "dox"))]
mod setting_tc_config;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::setting_tc_config::{SettingTCConfig, SettingTCConfigClass};

mod setting_team;
pub use self::setting_team::SettingTeamExt;
pub use self::setting_team::{SettingTeam, SettingTeamClass, NONE_SETTING_TEAM};

mod setting_team_port;
pub use self::setting_team_port::SettingTeamPortExt;
pub use self::setting_team_port::{SettingTeamPort, SettingTeamPortClass, NONE_SETTING_TEAM_PORT};

#[cfg(any(feature = "v1_2", feature = "dox"))]
mod setting_tun;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::setting_tun::SettingTunExt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::setting_tun::{SettingTun, SettingTunClass, NONE_SETTING_TUN};

mod setting_user;
pub use self::setting_user::{SettingUser, SettingUserClass};

mod setting_vlan;
pub use self::setting_vlan::SettingVlanExt;
pub use self::setting_vlan::{SettingVlan, SettingVlanClass, NONE_SETTING_VLAN};

mod setting_vpn;
pub use self::setting_vpn::SettingVpnExt;
pub use self::setting_vpn::{SettingVpn, SettingVpnClass, NONE_SETTING_VPN};

#[cfg(any(feature = "v1_2", feature = "dox"))]
mod setting_vxlan;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::setting_vxlan::SettingVxlanExt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::setting_vxlan::{SettingVxlan, SettingVxlanClass, NONE_SETTING_VXLAN};

#[cfg(any(feature = "v1_16", feature = "dox"))]
mod setting_wifi_p2p;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use self::setting_wifi_p2p::{SettingWifiP2P, SettingWifiP2PClass};

mod setting_wimax;
pub use self::setting_wimax::SettingWimaxExt;
pub use self::setting_wimax::{SettingWimax, SettingWimaxClass, NONE_SETTING_WIMAX};

#[cfg(any(feature = "v1_16", feature = "dox"))]
mod setting_wire_guard;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use self::setting_wire_guard::{SettingWireGuard, SettingWireGuardClass};

mod setting_wired;
pub use self::setting_wired::SettingWiredExt;
pub use self::setting_wired::{SettingWired, SettingWiredClass, NONE_SETTING_WIRED};

mod setting_wireless;
pub use self::setting_wireless::SettingWirelessExt;
pub use self::setting_wireless::{SettingWireless, SettingWirelessClass, NONE_SETTING_WIRELESS};

mod setting_wireless_security;
pub use self::setting_wireless_security::SettingWirelessSecurityExt;
pub use self::setting_wireless_security::{
    SettingWirelessSecurity, SettingWirelessSecurityClass, NONE_SETTING_WIRELESS_SECURITY,
};

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod setting_wpan;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::setting_wpan::{SettingWpan, SettingWpanClass};

mod simple_connection;
pub use self::simple_connection::{
    SimpleConnection, SimpleConnectionClass, NONE_SIMPLE_CONNECTION,
};

mod vpn_connection;
pub use self::vpn_connection::{VpnConnection, VpnConnectionClass};

mod vpn_editor;
pub use self::vpn_editor::VpnEditorExt;
pub use self::vpn_editor::{VpnEditor, NONE_VPN_EDITOR};

mod vpn_editor_plugin;
pub use self::vpn_editor_plugin::VpnEditorPluginExt;
pub use self::vpn_editor_plugin::{VpnEditorPlugin, NONE_VPN_EDITOR_PLUGIN};

#[cfg(any(feature = "v1_2", feature = "dox"))]
mod vpn_plugin_info;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::vpn_plugin_info::VpnPluginInfoExt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::vpn_plugin_info::{VpnPluginInfo, VpnPluginInfoClass, NONE_VPN_PLUGIN_INFO};

mod vpn_plugin_old;
pub use self::vpn_plugin_old::VpnPluginOldExt;
pub use self::vpn_plugin_old::{VpnPluginOld, VpnPluginOldClass, NONE_VPN_PLUGIN_OLD};

mod vpn_service_plugin;
pub use self::vpn_service_plugin::VpnServicePluginExt;
pub use self::vpn_service_plugin::{
    VpnServicePlugin, VpnServicePluginClass, NONE_VPN_SERVICE_PLUGIN,
};

#[cfg(any(feature = "v1_16", feature = "dox"))]
mod wifi_p2p_peer;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use self::wifi_p2p_peer::{WifiP2PPeer, WifiP2PPeerClass};

mod wimax_nsp;
pub use self::wimax_nsp::{WimaxNsp, WimaxNspClass};

#[cfg(any(feature = "v1_18", feature = "dox"))]
mod bridge_vlan;
#[cfg(any(feature = "v1_18", feature = "dox"))]
pub use self::bridge_vlan::BridgeVlan;

#[cfg(any(feature = "v1_6", feature = "dox"))]
mod dns_entry;
#[cfg(any(feature = "v1_6", feature = "dox"))]
pub use self::dns_entry::DnsEntry;

mod ip_address;
pub use self::ip_address::IPAddress;

mod ip_route;
pub use self::ip_route::IPRoute;

#[cfg(any(feature = "v1_18", feature = "dox"))]
mod ip_routing_rule;
#[cfg(any(feature = "v1_18", feature = "dox"))]
pub use self::ip_routing_rule::IPRoutingRule;

#[cfg(any(feature = "v1_2", feature = "dox"))]
mod lldp_neighbor;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::lldp_neighbor::LldpNeighbor;

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod sriov_vf;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::sriov_vf::SriovVF;

#[cfg(any(feature = "v1_12", feature = "dox"))]
mod tc_qdisc;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::tc_qdisc::TCQdisc;

#[cfg(any(feature = "v1_12", feature = "dox"))]
mod tc_tfilter;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::tc_tfilter::TCTfilter;

#[cfg(any(feature = "v1_12", feature = "dox"))]
mod team_link_watcher;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::team_link_watcher::TeamLinkWatcher;

#[cfg(any(feature = "v1_16", feature = "dox"))]
mod wire_guard_peer;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use self::wire_guard_peer::WireGuardPeer;

mod enums;
pub use self::enums::ActiveConnectionState;
#[cfg(any(feature = "v1_8", feature = "dox"))]
pub use self::enums::ActiveConnectionStateReason;
pub use self::enums::AgentManagerError;
pub use self::enums::Capability;
pub use self::enums::ClientError;
pub use self::enums::ClientPermission;
pub use self::enums::ClientPermissionResult;
pub use self::enums::ConnectionError;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::enums::ConnectionMultiConnect;
pub use self::enums::ConnectivityState;
pub use self::enums::CryptoError;
pub use self::enums::DeviceError;
pub use self::enums::DeviceState;
pub use self::enums::DeviceStateReason;
pub use self::enums::DeviceType;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::enums::IPTunnelMode;
pub use self::enums::ManagerError;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::enums::Metered;
#[cfg(any(feature = "v1_4", feature = "dox"))]
pub use self::enums::RollbackResult;
pub use self::enums::SecretAgentError;
pub use self::enums::Setting8021xCKFormat;
pub use self::enums::Setting8021xCKScheme;
pub use self::enums::SettingCompareFlags;
pub use self::enums::SettingConnectionAutoconnectSlaves;
pub use self::enums::SettingConnectionLldp;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::enums::SettingConnectionLlmnr;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::enums::SettingConnectionMdns;
pub use self::enums::SettingDiffResult;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::enums::SettingIP6ConfigAddrGenMode;
pub use self::enums::SettingIP6ConfigPrivacy;
pub use self::enums::SettingMacRandomization;
#[cfg(any(feature = "v1_6", feature = "dox"))]
pub use self::enums::SettingMacsecMode;
#[cfg(any(feature = "v1_6", feature = "dox"))]
pub use self::enums::SettingMacsecValidation;
pub use self::enums::SettingMacvlanMode;
#[cfg(any(feature = "v1_6", feature = "dox"))]
pub use self::enums::SettingProxyMethod;
pub use self::enums::SettingSerialParity;
pub use self::enums::SettingTunMode;
pub use self::enums::SettingWirelessPowersave;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::enums::SettingWirelessSecurityFils;
pub use self::enums::SettingWirelessSecurityPmf;
pub use self::enums::SettingsError;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::enums::SriovVFVlanProtocol;
pub use self::enums::State;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use self::enums::Ternary;
pub use self::enums::UtilsSecurityType;
pub use self::enums::VlanPriorityMap;
pub use self::enums::VpnConnectionState;
pub use self::enums::VpnConnectionStateReason;
pub use self::enums::VpnPluginError;
pub use self::enums::VpnPluginFailure;
pub use self::enums::VpnServiceState;
pub use self::enums::WepKeyType;
pub use self::enums::WimaxNspNetworkType;
pub use self::enums::_80211Mode;

mod flags;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub use self::flags::ActivationStateFlags;
pub use self::flags::BluetoothCapabilities;
#[cfg(any(feature = "v1_4", feature = "dox"))]
pub use self::flags::CheckpointCreateFlags;
pub use self::flags::ConnectionSerializationFlags;
pub use self::flags::DeviceCapabilities;
#[cfg(any(feature = "v1_22", feature = "dox"))]
pub use self::flags::DeviceInterfaceFlags;
pub use self::flags::DeviceModemCapabilities;
pub use self::flags::DeviceWifiCapabilities;
#[cfg(any(feature = "v1_22", feature = "dox"))]
pub use self::flags::DhcpHostnameFlags;
#[cfg(any(feature = "v1_22", feature = "dox"))]
pub use self::flags::IPAddressCmpFlags;
#[cfg(any(feature = "v1_18", feature = "dox"))]
pub use self::flags::IPRoutingRuleAsStringFlags;
pub use self::flags::IPTunnelFlags;
#[cfg(any(feature = "v1_22", feature = "dox"))]
pub use self::flags::ManagerReloadFlags;
pub use self::flags::SecretAgentCapabilities;
pub use self::flags::SecretAgentGetSecretsFlags;
#[cfg(any(feature = "v1_8", feature = "dox"))]
pub use self::flags::Setting8021xAuthFlags;
pub use self::flags::SettingDcbFlags;
pub use self::flags::SettingSecretFlags;
#[cfg(any(feature = "v1_2", feature = "dox"))]
pub use self::flags::SettingWiredWakeOnLan;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub use self::flags::SettingWirelessSecurityWpsMethod;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::flags::SettingWirelessWakeOnWLan;
#[cfg(any(feature = "v1_20", feature = "dox"))]
pub use self::flags::SettingsAddConnection2Flags;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::flags::SettingsConnectionFlags;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::flags::SettingsUpdate2Flags;
pub use self::flags::TeamLinkWatcherArpPingFlags;
pub use self::flags::VlanFlags;
pub use self::flags::VpnEditorPluginCapability;
pub use self::flags::_80211ApFlags;
pub use self::flags::_80211ApSecurityFlags;

pub mod functions;

mod constants;
pub use self::constants::ACCESS_POINT_BSSID;
pub use self::constants::ACCESS_POINT_FLAGS;
pub use self::constants::ACCESS_POINT_FREQUENCY;
pub use self::constants::ACCESS_POINT_HW_ADDRESS;
pub use self::constants::ACCESS_POINT_LAST_SEEN;
pub use self::constants::ACCESS_POINT_MAX_BITRATE;
pub use self::constants::ACCESS_POINT_MODE;
pub use self::constants::ACCESS_POINT_RSN_FLAGS;
pub use self::constants::ACCESS_POINT_SSID;
pub use self::constants::ACCESS_POINT_STRENGTH;
pub use self::constants::ACCESS_POINT_WPA_FLAGS;
pub use self::constants::ACTIVE_CONNECTION_CONNECTION;
pub use self::constants::ACTIVE_CONNECTION_DEFAULT;
pub use self::constants::ACTIVE_CONNECTION_DEFAULT6;
pub use self::constants::ACTIVE_CONNECTION_DEVICES;
pub use self::constants::ACTIVE_CONNECTION_DHCP4_CONFIG;
pub use self::constants::ACTIVE_CONNECTION_DHCP6_CONFIG;
pub use self::constants::ACTIVE_CONNECTION_ID;
pub use self::constants::ACTIVE_CONNECTION_IP4_CONFIG;
pub use self::constants::ACTIVE_CONNECTION_IP6_CONFIG;
pub use self::constants::ACTIVE_CONNECTION_MASTER;
pub use self::constants::ACTIVE_CONNECTION_SPECIFIC_OBJECT_PATH;
pub use self::constants::ACTIVE_CONNECTION_STATE;
pub use self::constants::ACTIVE_CONNECTION_STATE_FLAGS;
pub use self::constants::ACTIVE_CONNECTION_TYPE;
pub use self::constants::ACTIVE_CONNECTION_UUID;
pub use self::constants::ACTIVE_CONNECTION_VPN;
pub use self::constants::CHECKPOINT_CREATED;
pub use self::constants::CHECKPOINT_DEVICES;
pub use self::constants::CHECKPOINT_ROLLBACK_TIMEOUT;
pub use self::constants::CLIENT_ACTIVATING_CONNECTION;
pub use self::constants::CLIENT_ACTIVE_CONNECTIONS;
pub use self::constants::CLIENT_ACTIVE_CONNECTION_ADDED;
pub use self::constants::CLIENT_ACTIVE_CONNECTION_REMOVED;
pub use self::constants::CLIENT_ALL_DEVICES;
pub use self::constants::CLIENT_ANY_DEVICE_ADDED;
pub use self::constants::CLIENT_ANY_DEVICE_REMOVED;
pub use self::constants::CLIENT_CAN_MODIFY;
pub use self::constants::CLIENT_CAPABILITIES;
pub use self::constants::CLIENT_CHECKPOINTS;
pub use self::constants::CLIENT_CONNECTIONS;
pub use self::constants::CLIENT_CONNECTION_ADDED;
pub use self::constants::CLIENT_CONNECTION_REMOVED;
pub use self::constants::CLIENT_CONNECTIVITY;
pub use self::constants::CLIENT_CONNECTIVITY_CHECK_AVAILABLE;
pub use self::constants::CLIENT_CONNECTIVITY_CHECK_ENABLED;
pub use self::constants::CLIENT_CONNECTIVITY_CHECK_URI;
pub use self::constants::CLIENT_DBUS_CONNECTION;
pub use self::constants::CLIENT_DBUS_NAME_OWNER;
pub use self::constants::CLIENT_DEVICES;
pub use self::constants::CLIENT_DEVICE_ADDED;
pub use self::constants::CLIENT_DEVICE_REMOVED;
pub use self::constants::CLIENT_DNS_CONFIGURATION;
pub use self::constants::CLIENT_DNS_MODE;
pub use self::constants::CLIENT_DNS_RC_MANAGER;
pub use self::constants::CLIENT_HOSTNAME;
pub use self::constants::CLIENT_METERED;
pub use self::constants::CLIENT_NETWORKING_ENABLED;
pub use self::constants::CLIENT_NM_RUNNING;
pub use self::constants::CLIENT_PERMISSION_CHANGED;
pub use self::constants::CLIENT_PRIMARY_CONNECTION;
pub use self::constants::CLIENT_STARTUP;
pub use self::constants::CLIENT_STATE;
pub use self::constants::CLIENT_VERSION;
pub use self::constants::CLIENT_WIMAX_ENABLED;
pub use self::constants::CLIENT_WIMAX_HARDWARE_ENABLED;
pub use self::constants::CLIENT_WIRELESS_ENABLED;
pub use self::constants::CLIENT_WIRELESS_HARDWARE_ENABLED;
pub use self::constants::CLIENT_WWAN_ENABLED;
pub use self::constants::CLIENT_WWAN_HARDWARE_ENABLED;
pub use self::constants::CONNECTION_CHANGED;
pub use self::constants::CONNECTION_NORMALIZE_PARAM_IP6_CONFIG_METHOD;
pub use self::constants::CONNECTION_SECRETS_CLEARED;
pub use self::constants::CONNECTION_SECRETS_UPDATED;
pub use self::constants::DBUS_INTERFACE;
pub use self::constants::DBUS_INTERFACE_DNS_MANAGER;
pub use self::constants::DBUS_INTERFACE_SETTINGS;
pub use self::constants::DBUS_INTERFACE_SETTINGS_CONNECTION;
pub use self::constants::DBUS_INTERFACE_SETTINGS_CONNECTION_SECRETS;
pub use self::constants::DBUS_INTERFACE_VPN;
pub use self::constants::DBUS_INTERFACE_VPN_CONNECTION;
pub use self::constants::DBUS_INVALID_VPN_CONNECTION;
pub use self::constants::DBUS_NO_ACTIVE_VPN_CONNECTION;
pub use self::constants::DBUS_NO_VPN_CONNECTIONS;
pub use self::constants::DBUS_PATH;
pub use self::constants::DBUS_PATH_AGENT_MANAGER;
pub use self::constants::DBUS_PATH_DNS_MANAGER;
pub use self::constants::DBUS_PATH_SECRET_AGENT;
pub use self::constants::DBUS_PATH_SETTINGS;
pub use self::constants::DBUS_PATH_SETTINGS_CONNECTION;
pub use self::constants::DBUS_PATH_VPN;
pub use self::constants::DBUS_PATH_VPN_CONNECTION;
pub use self::constants::DBUS_SERVICE;
pub use self::constants::DBUS_VPN_ALREADY_STARTED;
pub use self::constants::DBUS_VPN_ALREADY_STOPPED;
pub use self::constants::DBUS_VPN_BAD_ARGUMENTS;
pub use self::constants::DBUS_VPN_ERROR_PREFIX;
pub use self::constants::DBUS_VPN_INTERACTIVE_NOT_SUPPORTED;
pub use self::constants::DBUS_VPN_SIGNAL_CONNECT_FAILED;
pub use self::constants::DBUS_VPN_SIGNAL_IP4_CONFIG;
pub use self::constants::DBUS_VPN_SIGNAL_IP_CONFIG_BAD;
pub use self::constants::DBUS_VPN_SIGNAL_LAUNCH_FAILED;
pub use self::constants::DBUS_VPN_SIGNAL_LOGIN_BANNER;
pub use self::constants::DBUS_VPN_SIGNAL_LOGIN_FAILED;
pub use self::constants::DBUS_VPN_SIGNAL_STATE_CHANGE;
pub use self::constants::DBUS_VPN_SIGNAL_VPN_CONFIG_BAD;
pub use self::constants::DBUS_VPN_STARTING_IN_PROGRESS;
pub use self::constants::DBUS_VPN_STOPPING_IN_PROGRESS;
pub use self::constants::DBUS_VPN_WRONG_STATE;
pub use self::constants::DEVICE_6LOWPAN_HW_ADDRESS;
pub use self::constants::DEVICE_6LOWPAN_PARENT;
pub use self::constants::DEVICE_ACTIVE_CONNECTION;
pub use self::constants::DEVICE_ADSL_CARRIER;
pub use self::constants::DEVICE_AUTOCONNECT;
pub use self::constants::DEVICE_AVAILABLE_CONNECTIONS;
pub use self::constants::DEVICE_BOND_CARRIER;
pub use self::constants::DEVICE_BOND_HW_ADDRESS;
pub use self::constants::DEVICE_BOND_SLAVES;
pub use self::constants::DEVICE_BRIDGE_CARRIER;
pub use self::constants::DEVICE_BRIDGE_HW_ADDRESS;
pub use self::constants::DEVICE_BRIDGE_SLAVES;
pub use self::constants::DEVICE_BT_CAPABILITIES;
pub use self::constants::DEVICE_BT_HW_ADDRESS;
pub use self::constants::DEVICE_BT_NAME;
pub use self::constants::DEVICE_CAPABILITIES;
pub use self::constants::DEVICE_DEVICE_TYPE;
pub use self::constants::DEVICE_DHCP4_CONFIG;
pub use self::constants::DEVICE_DHCP6_CONFIG;
pub use self::constants::DEVICE_DRIVER;
pub use self::constants::DEVICE_DRIVER_VERSION;
pub use self::constants::DEVICE_DUMMY_HW_ADDRESS;
pub use self::constants::DEVICE_ETHERNET_CARRIER;
pub use self::constants::DEVICE_ETHERNET_HW_ADDRESS;
pub use self::constants::DEVICE_ETHERNET_PERMANENT_HW_ADDRESS;
pub use self::constants::DEVICE_ETHERNET_S390_SUBCHANNELS;
pub use self::constants::DEVICE_ETHERNET_SPEED;
pub use self::constants::DEVICE_FIRMWARE_MISSING;
pub use self::constants::DEVICE_FIRMWARE_VERSION;
pub use self::constants::DEVICE_GENERIC_HW_ADDRESS;
pub use self::constants::DEVICE_GENERIC_TYPE_DESCRIPTION;
pub use self::constants::DEVICE_INFINIBAND_CARRIER;
pub use self::constants::DEVICE_INFINIBAND_HW_ADDRESS;
pub use self::constants::DEVICE_INTERFACE;
pub use self::constants::DEVICE_INTERFACE_FLAGS;
pub use self::constants::DEVICE_IP4_CONFIG;
pub use self::constants::DEVICE_IP4_CONNECTIVITY;
pub use self::constants::DEVICE_IP6_CONFIG;
pub use self::constants::DEVICE_IP6_CONNECTIVITY;
pub use self::constants::DEVICE_IP_INTERFACE;
pub use self::constants::DEVICE_IP_TUNNEL_ENCAPSULATION_LIMIT;
pub use self::constants::DEVICE_IP_TUNNEL_FLAGS;
pub use self::constants::DEVICE_IP_TUNNEL_FLOW_LABEL;
pub use self::constants::DEVICE_IP_TUNNEL_INPUT_KEY;
pub use self::constants::DEVICE_IP_TUNNEL_LOCAL;
pub use self::constants::DEVICE_IP_TUNNEL_MODE;
pub use self::constants::DEVICE_IP_TUNNEL_OUTPUT_KEY;
pub use self::constants::DEVICE_IP_TUNNEL_PARENT;
pub use self::constants::DEVICE_IP_TUNNEL_PATH_MTU_DISCOVERY;
pub use self::constants::DEVICE_IP_TUNNEL_REMOTE;
pub use self::constants::DEVICE_IP_TUNNEL_TOS;
pub use self::constants::DEVICE_IP_TUNNEL_TTL;
pub use self::constants::DEVICE_LLDP_NEIGHBORS;
pub use self::constants::DEVICE_MACSEC_CIPHER_SUITE;
pub use self::constants::DEVICE_MACSEC_ENCODING_SA;
pub use self::constants::DEVICE_MACSEC_ENCRYPT;
pub use self::constants::DEVICE_MACSEC_ES;
pub use self::constants::DEVICE_MACSEC_HW_ADDRESS;
pub use self::constants::DEVICE_MACSEC_ICV_LENGTH;
pub use self::constants::DEVICE_MACSEC_INCLUDE_SCI;
pub use self::constants::DEVICE_MACSEC_PARENT;
pub use self::constants::DEVICE_MACSEC_PROTECT;
pub use self::constants::DEVICE_MACSEC_REPLAY_PROTECT;
pub use self::constants::DEVICE_MACSEC_SCB;
pub use self::constants::DEVICE_MACSEC_SCI;
pub use self::constants::DEVICE_MACSEC_VALIDATION;
pub use self::constants::DEVICE_MACSEC_WINDOW;
pub use self::constants::DEVICE_MACVLAN_HW_ADDRESS;
pub use self::constants::DEVICE_MACVLAN_MODE;
pub use self::constants::DEVICE_MACVLAN_NO_PROMISC;
pub use self::constants::DEVICE_MACVLAN_PARENT;
pub use self::constants::DEVICE_MACVLAN_TAP;
pub use self::constants::DEVICE_MANAGED;
pub use self::constants::DEVICE_METERED;
pub use self::constants::DEVICE_MODEM_APN;
pub use self::constants::DEVICE_MODEM_CURRENT_CAPABILITIES;
pub use self::constants::DEVICE_MODEM_DEVICE_ID;
pub use self::constants::DEVICE_MODEM_MODEM_CAPABILITIES;
pub use self::constants::DEVICE_MODEM_OPERATOR_CODE;
pub use self::constants::DEVICE_MTU;
pub use self::constants::DEVICE_NM_PLUGIN_MISSING;
pub use self::constants::DEVICE_OLPC_MESH_ACTIVE_CHANNEL;
pub use self::constants::DEVICE_OLPC_MESH_COMPANION;
pub use self::constants::DEVICE_OLPC_MESH_HW_ADDRESS;
pub use self::constants::DEVICE_OVS_BRIDGE_SLAVES;
pub use self::constants::DEVICE_OVS_PORT_SLAVES;
pub use self::constants::DEVICE_PHYSICAL_PORT_ID;
pub use self::constants::DEVICE_PRODUCT;
pub use self::constants::DEVICE_REAL;
pub use self::constants::DEVICE_STATE;
pub use self::constants::DEVICE_STATE_REASON;
pub use self::constants::DEVICE_TEAM_CARRIER;
pub use self::constants::DEVICE_TEAM_CONFIG;
pub use self::constants::DEVICE_TEAM_HW_ADDRESS;
pub use self::constants::DEVICE_TEAM_SLAVES;
pub use self::constants::DEVICE_TUN_GROUP;
pub use self::constants::DEVICE_TUN_HW_ADDRESS;
pub use self::constants::DEVICE_TUN_MODE;
pub use self::constants::DEVICE_TUN_MULTI_QUEUE;
pub use self::constants::DEVICE_TUN_NO_PI;
pub use self::constants::DEVICE_TUN_OWNER;
pub use self::constants::DEVICE_TUN_VNET_HDR;
pub use self::constants::DEVICE_UDI;
pub use self::constants::DEVICE_VENDOR;
pub use self::constants::DEVICE_VLAN_CARRIER;
pub use self::constants::DEVICE_VLAN_HW_ADDRESS;
pub use self::constants::DEVICE_VLAN_PARENT;
pub use self::constants::DEVICE_VLAN_VLAN_ID;
pub use self::constants::DEVICE_VXLAN_AGEING;
pub use self::constants::DEVICE_VXLAN_CARRIER;
pub use self::constants::DEVICE_VXLAN_DST_PORT;
pub use self::constants::DEVICE_VXLAN_GROUP;
pub use self::constants::DEVICE_VXLAN_HW_ADDRESS;
pub use self::constants::DEVICE_VXLAN_ID;
pub use self::constants::DEVICE_VXLAN_L2MISS;
pub use self::constants::DEVICE_VXLAN_L3MISS;
pub use self::constants::DEVICE_VXLAN_LEARNING;
pub use self::constants::DEVICE_VXLAN_LIMIT;
pub use self::constants::DEVICE_VXLAN_LOCAL;
pub use self::constants::DEVICE_VXLAN_PARENT;
pub use self::constants::DEVICE_VXLAN_PROXY;
pub use self::constants::DEVICE_VXLAN_RSC;
pub use self::constants::DEVICE_VXLAN_SRC_PORT_MAX;
pub use self::constants::DEVICE_VXLAN_SRC_PORT_MIN;
pub use self::constants::DEVICE_VXLAN_TOS;
pub use self::constants::DEVICE_VXLAN_TTL;
pub use self::constants::DEVICE_WIFI_ACCESS_POINTS;
pub use self::constants::DEVICE_WIFI_ACTIVE_ACCESS_POINT;
pub use self::constants::DEVICE_WIFI_BITRATE;
pub use self::constants::DEVICE_WIFI_CAPABILITIES;
pub use self::constants::DEVICE_WIFI_HW_ADDRESS;
pub use self::constants::DEVICE_WIFI_LAST_SCAN;
pub use self::constants::DEVICE_WIFI_MODE;
pub use self::constants::DEVICE_WIFI_P2P_HW_ADDRESS;
pub use self::constants::DEVICE_WIFI_P2P_PEERS;
pub use self::constants::DEVICE_WIFI_P2P_WFDIES;
pub use self::constants::DEVICE_WIFI_PERMANENT_HW_ADDRESS;
pub use self::constants::DEVICE_WIMAX_ACTIVE_NSP;
pub use self::constants::DEVICE_WIMAX_BSID;
pub use self::constants::DEVICE_WIMAX_CENTER_FREQUENCY;
pub use self::constants::DEVICE_WIMAX_CINR;
pub use self::constants::DEVICE_WIMAX_HW_ADDRESS;
pub use self::constants::DEVICE_WIMAX_NSPS;
pub use self::constants::DEVICE_WIMAX_RSSI;
pub use self::constants::DEVICE_WIMAX_TX_POWER;
pub use self::constants::DEVICE_WIREGUARD_FWMARK;
pub use self::constants::DEVICE_WIREGUARD_LISTEN_PORT;
pub use self::constants::DEVICE_WIREGUARD_PUBLIC_KEY;
pub use self::constants::DEVICE_WPAN_HW_ADDRESS;
pub use self::constants::DHCP_CONFIG_FAMILY;
pub use self::constants::DHCP_CONFIG_OPTIONS;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_ESP_HW_OFFLOAD;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_ESP_TX_CSUM_HW_OFFLOAD;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_FCOE_MTU;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_GRO;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_GSO;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_HIGHDMA;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_HW_TC_OFFLOAD;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_L2_FWD_OFFLOAD;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_LOOPBACK;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_LRO;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_NTUPLE;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RX;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RXHASH;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RXVLAN;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RX_ALL;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RX_FCS;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RX_GRO_HW;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RX_UDP_TUNNEL_PORT_OFFLOAD;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RX_VLAN_FILTER;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RX_VLAN_STAG_FILTER;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_RX_VLAN_STAG_HW_PARSE;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_SG;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TLS_HW_RECORD;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TLS_HW_TX_OFFLOAD;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TSO;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TXVLAN;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_CHECKSUM_FCOE_CRC;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_CHECKSUM_IPV4;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_CHECKSUM_IPV6;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_CHECKSUM_IP_GENERIC;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_CHECKSUM_SCTP;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_ESP_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_FCOE_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_GRE_CSUM_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_GRE_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_GSO_PARTIAL;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_GSO_ROBUST;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_IPXIP4_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_IPXIP6_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_NOCACHE_COPY;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_SCATTER_GATHER;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_SCATTER_GATHER_FRAGLIST;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_SCTP_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_TCP6_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_TCP_ECN_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_TCP_MANGLEID_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_TCP_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_UDP_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_UDP_TNL_CSUM_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_UDP_TNL_SEGMENTATION;
pub use self::constants::ETHTOOL_OPTNAME_FEATURE_TX_VLAN_STAG_HW_INSERT;
pub use self::constants::IP_ADDRESS_ATTRIBUTE_LABEL;
pub use self::constants::IP_CONFIG_ADDRESSES;
pub use self::constants::IP_CONFIG_DOMAINS;
pub use self::constants::IP_CONFIG_FAMILY;
pub use self::constants::IP_CONFIG_GATEWAY;
pub use self::constants::IP_CONFIG_NAMESERVERS;
pub use self::constants::IP_CONFIG_ROUTES;
pub use self::constants::IP_CONFIG_SEARCHES;
pub use self::constants::IP_CONFIG_WINS_SERVERS;
pub use self::constants::IP_ROUTE_ATTRIBUTE_CWND;
pub use self::constants::IP_ROUTE_ATTRIBUTE_FROM;
pub use self::constants::IP_ROUTE_ATTRIBUTE_INITCWND;
pub use self::constants::IP_ROUTE_ATTRIBUTE_INITRWND;
pub use self::constants::IP_ROUTE_ATTRIBUTE_LOCK_CWND;
pub use self::constants::IP_ROUTE_ATTRIBUTE_LOCK_INITCWND;
pub use self::constants::IP_ROUTE_ATTRIBUTE_LOCK_INITRWND;
pub use self::constants::IP_ROUTE_ATTRIBUTE_LOCK_MTU;
pub use self::constants::IP_ROUTE_ATTRIBUTE_LOCK_WINDOW;
pub use self::constants::IP_ROUTE_ATTRIBUTE_MTU;
pub use self::constants::IP_ROUTE_ATTRIBUTE_ONLINK;
pub use self::constants::IP_ROUTE_ATTRIBUTE_SCOPE;
pub use self::constants::IP_ROUTE_ATTRIBUTE_SRC;
pub use self::constants::IP_ROUTE_ATTRIBUTE_TABLE;
pub use self::constants::IP_ROUTE_ATTRIBUTE_TOS;
pub use self::constants::IP_ROUTE_ATTRIBUTE_WINDOW;
pub use self::constants::LLDP_ATTR_CHASSIS_ID;
pub use self::constants::LLDP_ATTR_CHASSIS_ID_TYPE;
pub use self::constants::LLDP_ATTR_DESTINATION;
pub use self::constants::LLDP_ATTR_IEEE_802_1_PPVID;
pub use self::constants::LLDP_ATTR_IEEE_802_1_PPVIDS;
pub use self::constants::LLDP_ATTR_IEEE_802_1_PPVID_FLAGS;
pub use self::constants::LLDP_ATTR_IEEE_802_1_PVID;
pub use self::constants::LLDP_ATTR_IEEE_802_1_VID;
pub use self::constants::LLDP_ATTR_IEEE_802_1_VLANS;
pub use self::constants::LLDP_ATTR_IEEE_802_1_VLAN_NAME;
pub use self::constants::LLDP_ATTR_IEEE_802_3_MAC_PHY_CONF;
pub use self::constants::LLDP_ATTR_IEEE_802_3_MAX_FRAME_SIZE;
pub use self::constants::LLDP_ATTR_IEEE_802_3_POWER_VIA_MDI;
pub use self::constants::LLDP_ATTR_MANAGEMENT_ADDRESSES;
pub use self::constants::LLDP_ATTR_PORT_DESCRIPTION;
pub use self::constants::LLDP_ATTR_PORT_ID;
pub use self::constants::LLDP_ATTR_PORT_ID_TYPE;
pub use self::constants::LLDP_ATTR_SYSTEM_CAPABILITIES;
pub use self::constants::LLDP_ATTR_SYSTEM_DESCRIPTION;
pub use self::constants::LLDP_ATTR_SYSTEM_NAME;
pub use self::constants::LLDP_DEST_NEAREST_BRIDGE;
pub use self::constants::LLDP_DEST_NEAREST_CUSTOMER_BRIDGE;
pub use self::constants::LLDP_DEST_NEAREST_NON_TPMR_BRIDGE;
pub use self::constants::OBJECT_PATH;
pub use self::constants::REMOTE_CONNECTION_DBUS_CONNECTION;
pub use self::constants::REMOTE_CONNECTION_FILENAME;
pub use self::constants::REMOTE_CONNECTION_FLAGS;
pub use self::constants::REMOTE_CONNECTION_PATH;
pub use self::constants::REMOTE_CONNECTION_UNSAVED;
pub use self::constants::REMOTE_CONNECTION_VISIBLE;
pub use self::constants::SECRET_AGENT_OLD_AUTO_REGISTER;
pub use self::constants::SECRET_AGENT_OLD_CAPABILITIES;
pub use self::constants::SECRET_AGENT_OLD_IDENTIFIER;
pub use self::constants::SECRET_AGENT_OLD_REGISTERED;
pub use self::constants::SETTING_6LOWPAN_PARENT;
pub use self::constants::SETTING_6LOWPAN_SETTING_NAME;
pub use self::constants::SETTING_802_1X_ALTSUBJECT_MATCHES;
pub use self::constants::SETTING_802_1X_ANONYMOUS_IDENTITY;
pub use self::constants::SETTING_802_1X_AUTH_TIMEOUT;
pub use self::constants::SETTING_802_1X_CA_CERT;
pub use self::constants::SETTING_802_1X_CA_CERT_PASSWORD;
pub use self::constants::SETTING_802_1X_CA_CERT_PASSWORD_FLAGS;
pub use self::constants::SETTING_802_1X_CA_PATH;
pub use self::constants::SETTING_802_1X_CERT_SCHEME_PREFIX_PATH;
pub use self::constants::SETTING_802_1X_CERT_SCHEME_PREFIX_PKCS11;
pub use self::constants::SETTING_802_1X_CLIENT_CERT;
pub use self::constants::SETTING_802_1X_CLIENT_CERT_PASSWORD;
pub use self::constants::SETTING_802_1X_CLIENT_CERT_PASSWORD_FLAGS;
pub use self::constants::SETTING_802_1X_DOMAIN_SUFFIX_MATCH;
pub use self::constants::SETTING_802_1X_EAP;
pub use self::constants::SETTING_802_1X_IDENTITY;
pub use self::constants::SETTING_802_1X_OPTIONAL;
pub use self::constants::SETTING_802_1X_PAC_FILE;
pub use self::constants::SETTING_802_1X_PASSWORD;
pub use self::constants::SETTING_802_1X_PASSWORD_FLAGS;
pub use self::constants::SETTING_802_1X_PASSWORD_RAW;
pub use self::constants::SETTING_802_1X_PASSWORD_RAW_FLAGS;
pub use self::constants::SETTING_802_1X_PHASE1_AUTH_FLAGS;
pub use self::constants::SETTING_802_1X_PHASE1_FAST_PROVISIONING;
pub use self::constants::SETTING_802_1X_PHASE1_PEAPLABEL;
pub use self::constants::SETTING_802_1X_PHASE1_PEAPVER;
pub use self::constants::SETTING_802_1X_PHASE2_ALTSUBJECT_MATCHES;
pub use self::constants::SETTING_802_1X_PHASE2_AUTH;
pub use self::constants::SETTING_802_1X_PHASE2_AUTHEAP;
pub use self::constants::SETTING_802_1X_PHASE2_CA_CERT;
pub use self::constants::SETTING_802_1X_PHASE2_CA_CERT_PASSWORD;
pub use self::constants::SETTING_802_1X_PHASE2_CA_CERT_PASSWORD_FLAGS;
pub use self::constants::SETTING_802_1X_PHASE2_CA_PATH;
pub use self::constants::SETTING_802_1X_PHASE2_CLIENT_CERT;
pub use self::constants::SETTING_802_1X_PHASE2_CLIENT_CERT_PASSWORD;
pub use self::constants::SETTING_802_1X_PHASE2_CLIENT_CERT_PASSWORD_FLAGS;
pub use self::constants::SETTING_802_1X_PHASE2_DOMAIN_SUFFIX_MATCH;
pub use self::constants::SETTING_802_1X_PHASE2_PRIVATE_KEY;
pub use self::constants::SETTING_802_1X_PHASE2_PRIVATE_KEY_PASSWORD;
pub use self::constants::SETTING_802_1X_PHASE2_PRIVATE_KEY_PASSWORD_FLAGS;
pub use self::constants::SETTING_802_1X_PHASE2_SUBJECT_MATCH;
pub use self::constants::SETTING_802_1X_PIN;
pub use self::constants::SETTING_802_1X_PIN_FLAGS;
pub use self::constants::SETTING_802_1X_PRIVATE_KEY;
pub use self::constants::SETTING_802_1X_PRIVATE_KEY_PASSWORD;
pub use self::constants::SETTING_802_1X_PRIVATE_KEY_PASSWORD_FLAGS;
pub use self::constants::SETTING_802_1X_SETTING_NAME;
pub use self::constants::SETTING_802_1X_SUBJECT_MATCH;
pub use self::constants::SETTING_802_1X_SYSTEM_CA_CERTS;
pub use self::constants::SETTING_ADSL_ENCAPSULATION;
pub use self::constants::SETTING_ADSL_ENCAPSULATION_LLC;
pub use self::constants::SETTING_ADSL_ENCAPSULATION_VCMUX;
pub use self::constants::SETTING_ADSL_PASSWORD;
pub use self::constants::SETTING_ADSL_PASSWORD_FLAGS;
pub use self::constants::SETTING_ADSL_PROTOCOL;
pub use self::constants::SETTING_ADSL_PROTOCOL_IPOATM;
pub use self::constants::SETTING_ADSL_PROTOCOL_PPPOA;
pub use self::constants::SETTING_ADSL_PROTOCOL_PPPOE;
pub use self::constants::SETTING_ADSL_SETTING_NAME;
pub use self::constants::SETTING_ADSL_USERNAME;
pub use self::constants::SETTING_ADSL_VCI;
pub use self::constants::SETTING_ADSL_VPI;
pub use self::constants::SETTING_BLUETOOTH_BDADDR;
pub use self::constants::SETTING_BLUETOOTH_SETTING_NAME;
pub use self::constants::SETTING_BLUETOOTH_TYPE;
pub use self::constants::SETTING_BLUETOOTH_TYPE_DUN;
pub use self::constants::SETTING_BLUETOOTH_TYPE_NAP;
pub use self::constants::SETTING_BLUETOOTH_TYPE_PANU;
pub use self::constants::SETTING_BOND_OPTIONS;
pub use self::constants::SETTING_BOND_OPTION_ACTIVE_SLAVE;
pub use self::constants::SETTING_BOND_OPTION_AD_ACTOR_SYSTEM;
pub use self::constants::SETTING_BOND_OPTION_AD_ACTOR_SYS_PRIO;
pub use self::constants::SETTING_BOND_OPTION_AD_SELECT;
pub use self::constants::SETTING_BOND_OPTION_AD_USER_PORT_KEY;
pub use self::constants::SETTING_BOND_OPTION_ALL_SLAVES_ACTIVE;
pub use self::constants::SETTING_BOND_OPTION_ARP_ALL_TARGETS;
pub use self::constants::SETTING_BOND_OPTION_ARP_INTERVAL;
pub use self::constants::SETTING_BOND_OPTION_ARP_IP_TARGET;
pub use self::constants::SETTING_BOND_OPTION_ARP_VALIDATE;
pub use self::constants::SETTING_BOND_OPTION_DOWNDELAY;
pub use self::constants::SETTING_BOND_OPTION_FAIL_OVER_MAC;
pub use self::constants::SETTING_BOND_OPTION_LACP_RATE;
pub use self::constants::SETTING_BOND_OPTION_LP_INTERVAL;
pub use self::constants::SETTING_BOND_OPTION_MIIMON;
pub use self::constants::SETTING_BOND_OPTION_MIN_LINKS;
pub use self::constants::SETTING_BOND_OPTION_MODE;
pub use self::constants::SETTING_BOND_OPTION_NUM_GRAT_ARP;
pub use self::constants::SETTING_BOND_OPTION_NUM_UNSOL_NA;
pub use self::constants::SETTING_BOND_OPTION_PACKETS_PER_SLAVE;
pub use self::constants::SETTING_BOND_OPTION_PRIMARY;
pub use self::constants::SETTING_BOND_OPTION_PRIMARY_RESELECT;
pub use self::constants::SETTING_BOND_OPTION_RESEND_IGMP;
pub use self::constants::SETTING_BOND_OPTION_TLB_DYNAMIC_LB;
pub use self::constants::SETTING_BOND_OPTION_UPDELAY;
pub use self::constants::SETTING_BOND_OPTION_USE_CARRIER;
pub use self::constants::SETTING_BOND_OPTION_XMIT_HASH_POLICY;
pub use self::constants::SETTING_BOND_SETTING_NAME;
pub use self::constants::SETTING_BRIDGE_AGEING_TIME;
pub use self::constants::SETTING_BRIDGE_FORWARD_DELAY;
pub use self::constants::SETTING_BRIDGE_GROUP_FORWARD_MASK;
pub use self::constants::SETTING_BRIDGE_HELLO_TIME;
pub use self::constants::SETTING_BRIDGE_MAC_ADDRESS;
pub use self::constants::SETTING_BRIDGE_MAX_AGE;
pub use self::constants::SETTING_BRIDGE_MULTICAST_SNOOPING;
pub use self::constants::SETTING_BRIDGE_PORT_HAIRPIN_MODE;
pub use self::constants::SETTING_BRIDGE_PORT_PATH_COST;
pub use self::constants::SETTING_BRIDGE_PORT_PRIORITY;
pub use self::constants::SETTING_BRIDGE_PORT_SETTING_NAME;
pub use self::constants::SETTING_BRIDGE_PORT_VLANS;
pub use self::constants::SETTING_BRIDGE_PRIORITY;
pub use self::constants::SETTING_BRIDGE_SETTING_NAME;
pub use self::constants::SETTING_BRIDGE_STP;
pub use self::constants::SETTING_BRIDGE_VLANS;
pub use self::constants::SETTING_BRIDGE_VLAN_DEFAULT_PVID;
pub use self::constants::SETTING_BRIDGE_VLAN_FILTERING;
pub use self::constants::SETTING_CDMA_MTU;
pub use self::constants::SETTING_CDMA_NUMBER;
pub use self::constants::SETTING_CDMA_PASSWORD;
pub use self::constants::SETTING_CDMA_PASSWORD_FLAGS;
pub use self::constants::SETTING_CDMA_SETTING_NAME;
pub use self::constants::SETTING_CDMA_USERNAME;
pub use self::constants::SETTING_CONNECTION_AUTH_RETRIES;
pub use self::constants::SETTING_CONNECTION_AUTOCONNECT;
pub use self::constants::SETTING_CONNECTION_AUTOCONNECT_PRIORITY;
pub use self::constants::SETTING_CONNECTION_AUTOCONNECT_RETRIES;
pub use self::constants::SETTING_CONNECTION_AUTOCONNECT_SLAVES;
pub use self::constants::SETTING_CONNECTION_GATEWAY_PING_TIMEOUT;
pub use self::constants::SETTING_CONNECTION_ID;
pub use self::constants::SETTING_CONNECTION_INTERFACE_NAME;
pub use self::constants::SETTING_CONNECTION_LLDP;
pub use self::constants::SETTING_CONNECTION_LLMNR;
pub use self::constants::SETTING_CONNECTION_MASTER;
pub use self::constants::SETTING_CONNECTION_MDNS;
pub use self::constants::SETTING_CONNECTION_METERED;
pub use self::constants::SETTING_CONNECTION_MULTI_CONNECT;
pub use self::constants::SETTING_CONNECTION_PERMISSIONS;
pub use self::constants::SETTING_CONNECTION_READ_ONLY;
pub use self::constants::SETTING_CONNECTION_SECONDARIES;
pub use self::constants::SETTING_CONNECTION_SETTING_NAME;
pub use self::constants::SETTING_CONNECTION_SLAVE_TYPE;
pub use self::constants::SETTING_CONNECTION_STABLE_ID;
pub use self::constants::SETTING_CONNECTION_TIMESTAMP;
pub use self::constants::SETTING_CONNECTION_TYPE;
pub use self::constants::SETTING_CONNECTION_UUID;
pub use self::constants::SETTING_CONNECTION_WAIT_DEVICE_TIMEOUT;
pub use self::constants::SETTING_CONNECTION_ZONE;
pub use self::constants::SETTING_DCB_APP_FCOE_FLAGS;
pub use self::constants::SETTING_DCB_APP_FCOE_MODE;
pub use self::constants::SETTING_DCB_APP_FCOE_PRIORITY;
pub use self::constants::SETTING_DCB_APP_FIP_FLAGS;
pub use self::constants::SETTING_DCB_APP_FIP_PRIORITY;
pub use self::constants::SETTING_DCB_APP_ISCSI_FLAGS;
pub use self::constants::SETTING_DCB_APP_ISCSI_PRIORITY;
pub use self::constants::SETTING_DCB_FCOE_MODE_FABRIC;
pub use self::constants::SETTING_DCB_FCOE_MODE_VN2VN;
pub use self::constants::SETTING_DCB_PRIORITY_BANDWIDTH;
pub use self::constants::SETTING_DCB_PRIORITY_FLOW_CONTROL;
pub use self::constants::SETTING_DCB_PRIORITY_FLOW_CONTROL_FLAGS;
pub use self::constants::SETTING_DCB_PRIORITY_GROUP_BANDWIDTH;
pub use self::constants::SETTING_DCB_PRIORITY_GROUP_FLAGS;
pub use self::constants::SETTING_DCB_PRIORITY_GROUP_ID;
pub use self::constants::SETTING_DCB_PRIORITY_STRICT_BANDWIDTH;
pub use self::constants::SETTING_DCB_PRIORITY_TRAFFIC_CLASS;
pub use self::constants::SETTING_DCB_SETTING_NAME;
pub use self::constants::SETTING_DNS_OPTION_ATTEMPTS;
pub use self::constants::SETTING_DNS_OPTION_DEBUG;
pub use self::constants::SETTING_DNS_OPTION_EDNS0;
pub use self::constants::SETTING_DNS_OPTION_INET6;
pub use self::constants::SETTING_DNS_OPTION_IP6_BYTESTRING;
pub use self::constants::SETTING_DNS_OPTION_IP6_DOTINT;
pub use self::constants::SETTING_DNS_OPTION_NDOTS;
pub use self::constants::SETTING_DNS_OPTION_NO_CHECK_NAMES;
pub use self::constants::SETTING_DNS_OPTION_NO_IP6_DOTINT;
pub use self::constants::SETTING_DNS_OPTION_NO_TLD_QUERY;
pub use self::constants::SETTING_DNS_OPTION_ROTATE;
pub use self::constants::SETTING_DNS_OPTION_SINGLE_REQUEST;
pub use self::constants::SETTING_DNS_OPTION_SINGLE_REQUEST_REOPEN;
pub use self::constants::SETTING_DNS_OPTION_TIMEOUT;
pub use self::constants::SETTING_DNS_OPTION_USE_VC;
pub use self::constants::SETTING_DUMMY_SETTING_NAME;
pub use self::constants::SETTING_ETHTOOL_SETTING_NAME;
pub use self::constants::SETTING_GENERIC_SETTING_NAME;
pub use self::constants::SETTING_GSM_APN;
pub use self::constants::SETTING_GSM_AUTO_CONFIG;
pub use self::constants::SETTING_GSM_DEVICE_ID;
pub use self::constants::SETTING_GSM_HOME_ONLY;
pub use self::constants::SETTING_GSM_MTU;
pub use self::constants::SETTING_GSM_NETWORK_ID;
pub use self::constants::SETTING_GSM_NUMBER;
pub use self::constants::SETTING_GSM_PASSWORD;
pub use self::constants::SETTING_GSM_PASSWORD_FLAGS;
pub use self::constants::SETTING_GSM_PIN;
pub use self::constants::SETTING_GSM_PIN_FLAGS;
pub use self::constants::SETTING_GSM_SETTING_NAME;
pub use self::constants::SETTING_GSM_SIM_ID;
pub use self::constants::SETTING_GSM_SIM_OPERATOR_ID;
pub use self::constants::SETTING_GSM_USERNAME;
pub use self::constants::SETTING_INFINIBAND_MAC_ADDRESS;
pub use self::constants::SETTING_INFINIBAND_MTU;
pub use self::constants::SETTING_INFINIBAND_PARENT;
pub use self::constants::SETTING_INFINIBAND_P_KEY;
pub use self::constants::SETTING_INFINIBAND_SETTING_NAME;
pub use self::constants::SETTING_INFINIBAND_TRANSPORT_MODE;
pub use self::constants::SETTING_IP4_CONFIG_DHCP_CLIENT_ID;
pub use self::constants::SETTING_IP4_CONFIG_DHCP_FQDN;
pub use self::constants::SETTING_IP4_CONFIG_METHOD_AUTO;
pub use self::constants::SETTING_IP4_CONFIG_METHOD_DISABLED;
pub use self::constants::SETTING_IP4_CONFIG_METHOD_LINK_LOCAL;
pub use self::constants::SETTING_IP4_CONFIG_METHOD_MANUAL;
pub use self::constants::SETTING_IP4_CONFIG_METHOD_SHARED;
pub use self::constants::SETTING_IP4_CONFIG_SETTING_NAME;
pub use self::constants::SETTING_IP6_CONFIG_ADDR_GEN_MODE;
pub use self::constants::SETTING_IP6_CONFIG_DHCP_DUID;
pub use self::constants::SETTING_IP6_CONFIG_IP6_PRIVACY;
pub use self::constants::SETTING_IP6_CONFIG_METHOD_AUTO;
pub use self::constants::SETTING_IP6_CONFIG_METHOD_DHCP;
#[cfg(any(feature = "v1_20", feature = "dox"))]
pub use self::constants::SETTING_IP6_CONFIG_METHOD_DISABLED;
pub use self::constants::SETTING_IP6_CONFIG_METHOD_IGNORE;
pub use self::constants::SETTING_IP6_CONFIG_METHOD_LINK_LOCAL;
pub use self::constants::SETTING_IP6_CONFIG_METHOD_MANUAL;
pub use self::constants::SETTING_IP6_CONFIG_METHOD_SHARED;
pub use self::constants::SETTING_IP6_CONFIG_SETTING_NAME;
pub use self::constants::SETTING_IP6_CONFIG_TOKEN;
pub use self::constants::SETTING_IP_CONFIG_ADDRESSES;
pub use self::constants::SETTING_IP_CONFIG_DAD_TIMEOUT;
pub use self::constants::SETTING_IP_CONFIG_DHCP_HOSTNAME;
pub use self::constants::SETTING_IP_CONFIG_DHCP_HOSTNAME_FLAGS;
pub use self::constants::SETTING_IP_CONFIG_DHCP_IAID;
pub use self::constants::SETTING_IP_CONFIG_DHCP_SEND_HOSTNAME;
pub use self::constants::SETTING_IP_CONFIG_DHCP_TIMEOUT;
pub use self::constants::SETTING_IP_CONFIG_DNS;
pub use self::constants::SETTING_IP_CONFIG_DNS_OPTIONS;
pub use self::constants::SETTING_IP_CONFIG_DNS_PRIORITY;
pub use self::constants::SETTING_IP_CONFIG_DNS_SEARCH;
pub use self::constants::SETTING_IP_CONFIG_GATEWAY;
pub use self::constants::SETTING_IP_CONFIG_IGNORE_AUTO_DNS;
pub use self::constants::SETTING_IP_CONFIG_IGNORE_AUTO_ROUTES;
pub use self::constants::SETTING_IP_CONFIG_MAY_FAIL;
pub use self::constants::SETTING_IP_CONFIG_METHOD;
pub use self::constants::SETTING_IP_CONFIG_NEVER_DEFAULT;
pub use self::constants::SETTING_IP_CONFIG_ROUTES;
pub use self::constants::SETTING_IP_CONFIG_ROUTE_METRIC;
pub use self::constants::SETTING_IP_CONFIG_ROUTE_TABLE;
pub use self::constants::SETTING_IP_CONFIG_ROUTING_RULES;
pub use self::constants::SETTING_IP_TUNNEL_ENCAPSULATION_LIMIT;
pub use self::constants::SETTING_IP_TUNNEL_FLAGS;
pub use self::constants::SETTING_IP_TUNNEL_FLOW_LABEL;
pub use self::constants::SETTING_IP_TUNNEL_INPUT_KEY;
pub use self::constants::SETTING_IP_TUNNEL_LOCAL;
pub use self::constants::SETTING_IP_TUNNEL_MODE;
pub use self::constants::SETTING_IP_TUNNEL_MTU;
pub use self::constants::SETTING_IP_TUNNEL_OUTPUT_KEY;
pub use self::constants::SETTING_IP_TUNNEL_PARENT;
pub use self::constants::SETTING_IP_TUNNEL_PATH_MTU_DISCOVERY;
pub use self::constants::SETTING_IP_TUNNEL_REMOTE;
pub use self::constants::SETTING_IP_TUNNEL_SETTING_NAME;
pub use self::constants::SETTING_IP_TUNNEL_TOS;
pub use self::constants::SETTING_IP_TUNNEL_TTL;
pub use self::constants::SETTING_MACSEC_ENCRYPT;
pub use self::constants::SETTING_MACSEC_MKA_CAK;
pub use self::constants::SETTING_MACSEC_MKA_CAK_FLAGS;
pub use self::constants::SETTING_MACSEC_MKA_CKN;
pub use self::constants::SETTING_MACSEC_MODE;
pub use self::constants::SETTING_MACSEC_PARENT;
pub use self::constants::SETTING_MACSEC_PORT;
pub use self::constants::SETTING_MACSEC_SEND_SCI;
pub use self::constants::SETTING_MACSEC_SETTING_NAME;
pub use self::constants::SETTING_MACSEC_VALIDATION;
pub use self::constants::SETTING_MACVLAN_MODE;
pub use self::constants::SETTING_MACVLAN_PARENT;
pub use self::constants::SETTING_MACVLAN_PROMISCUOUS;
pub use self::constants::SETTING_MACVLAN_SETTING_NAME;
pub use self::constants::SETTING_MACVLAN_TAP;
pub use self::constants::SETTING_MATCH_INTERFACE_NAME;
pub use self::constants::SETTING_MATCH_SETTING_NAME;
pub use self::constants::SETTING_NAME;
pub use self::constants::SETTING_OLPC_MESH_CHANNEL;
pub use self::constants::SETTING_OLPC_MESH_DHCP_ANYCAST_ADDRESS;
pub use self::constants::SETTING_OLPC_MESH_SETTING_NAME;
pub use self::constants::SETTING_OLPC_MESH_SSID;
pub use self::constants::SETTING_OVS_BRIDGE_DATAPATH_TYPE;
pub use self::constants::SETTING_OVS_BRIDGE_FAIL_MODE;
pub use self::constants::SETTING_OVS_BRIDGE_MCAST_SNOOPING_ENABLE;
pub use self::constants::SETTING_OVS_BRIDGE_RSTP_ENABLE;
pub use self::constants::SETTING_OVS_BRIDGE_SETTING_NAME;
pub use self::constants::SETTING_OVS_BRIDGE_STP_ENABLE;
pub use self::constants::SETTING_OVS_DPDK_DEVARGS;
pub use self::constants::SETTING_OVS_DPDK_SETTING_NAME;
pub use self::constants::SETTING_OVS_INTERFACE_SETTING_NAME;
pub use self::constants::SETTING_OVS_INTERFACE_TYPE;
pub use self::constants::SETTING_OVS_PATCH_PEER;
pub use self::constants::SETTING_OVS_PATCH_SETTING_NAME;
pub use self::constants::SETTING_OVS_PORT_BOND_DOWNDELAY;
pub use self::constants::SETTING_OVS_PORT_BOND_MODE;
pub use self::constants::SETTING_OVS_PORT_BOND_UPDELAY;
pub use self::constants::SETTING_OVS_PORT_LACP;
pub use self::constants::SETTING_OVS_PORT_SETTING_NAME;
pub use self::constants::SETTING_OVS_PORT_TAG;
pub use self::constants::SETTING_OVS_PORT_VLAN_MODE;
pub use self::constants::SETTING_PPPOE_PARENT;
pub use self::constants::SETTING_PPPOE_PASSWORD;
pub use self::constants::SETTING_PPPOE_PASSWORD_FLAGS;
pub use self::constants::SETTING_PPPOE_SERVICE;
pub use self::constants::SETTING_PPPOE_SETTING_NAME;
pub use self::constants::SETTING_PPPOE_USERNAME;
pub use self::constants::SETTING_PPP_BAUD;
pub use self::constants::SETTING_PPP_CRTSCTS;
pub use self::constants::SETTING_PPP_LCP_ECHO_FAILURE;
pub use self::constants::SETTING_PPP_LCP_ECHO_INTERVAL;
pub use self::constants::SETTING_PPP_MPPE_STATEFUL;
pub use self::constants::SETTING_PPP_MRU;
pub use self::constants::SETTING_PPP_MTU;
pub use self::constants::SETTING_PPP_NOAUTH;
pub use self::constants::SETTING_PPP_NOBSDCOMP;
pub use self::constants::SETTING_PPP_NODEFLATE;
pub use self::constants::SETTING_PPP_NO_VJ_COMP;
pub use self::constants::SETTING_PPP_REFUSE_CHAP;
pub use self::constants::SETTING_PPP_REFUSE_EAP;
pub use self::constants::SETTING_PPP_REFUSE_MSCHAP;
pub use self::constants::SETTING_PPP_REFUSE_MSCHAPV2;
pub use self::constants::SETTING_PPP_REFUSE_PAP;
pub use self::constants::SETTING_PPP_REQUIRE_MPPE;
pub use self::constants::SETTING_PPP_REQUIRE_MPPE_128;
pub use self::constants::SETTING_PPP_SETTING_NAME;
pub use self::constants::SETTING_PROXY_BROWSER_ONLY;
pub use self::constants::SETTING_PROXY_METHOD;
pub use self::constants::SETTING_PROXY_PAC_SCRIPT;
pub use self::constants::SETTING_PROXY_PAC_URL;
pub use self::constants::SETTING_PROXY_SETTING_NAME;
pub use self::constants::SETTING_SERIAL_BAUD;
pub use self::constants::SETTING_SERIAL_BITS;
pub use self::constants::SETTING_SERIAL_PARITY;
pub use self::constants::SETTING_SERIAL_SEND_DELAY;
pub use self::constants::SETTING_SERIAL_SETTING_NAME;
pub use self::constants::SETTING_SERIAL_STOPBITS;
pub use self::constants::SETTING_SRIOV_AUTOPROBE_DRIVERS;
pub use self::constants::SETTING_SRIOV_SETTING_NAME;
pub use self::constants::SETTING_SRIOV_TOTAL_VFS;
pub use self::constants::SETTING_SRIOV_VFS;
pub use self::constants::SETTING_TC_CONFIG_QDISCS;
pub use self::constants::SETTING_TC_CONFIG_SETTING_NAME;
pub use self::constants::SETTING_TC_CONFIG_TFILTERS;
pub use self::constants::SETTING_TEAM_CONFIG;
pub use self::constants::SETTING_TEAM_LINK_WATCHERS;
pub use self::constants::SETTING_TEAM_MCAST_REJOIN_COUNT;
pub use self::constants::SETTING_TEAM_MCAST_REJOIN_INTERVAL;
pub use self::constants::SETTING_TEAM_NOTIFY_PEERS_COUNT;
pub use self::constants::SETTING_TEAM_NOTIFY_PEERS_INTERVAL;
pub use self::constants::SETTING_TEAM_PORT_CONFIG;
pub use self::constants::SETTING_TEAM_PORT_LACP_KEY;
pub use self::constants::SETTING_TEAM_PORT_LACP_PRIO;
pub use self::constants::SETTING_TEAM_PORT_LINK_WATCHERS;
pub use self::constants::SETTING_TEAM_PORT_PRIO;
pub use self::constants::SETTING_TEAM_PORT_QUEUE_ID;
pub use self::constants::SETTING_TEAM_PORT_SETTING_NAME;
pub use self::constants::SETTING_TEAM_PORT_STICKY;
pub use self::constants::SETTING_TEAM_RUNNER;
pub use self::constants::SETTING_TEAM_RUNNER_ACTIVE;
pub use self::constants::SETTING_TEAM_RUNNER_ACTIVEBACKUP;
pub use self::constants::SETTING_TEAM_RUNNER_AGG_SELECT_POLICY;
pub use self::constants::SETTING_TEAM_RUNNER_AGG_SELECT_POLICY_BANDWIDTH;
pub use self::constants::SETTING_TEAM_RUNNER_AGG_SELECT_POLICY_COUNT;
pub use self::constants::SETTING_TEAM_RUNNER_AGG_SELECT_POLICY_LACP_PRIO;
pub use self::constants::SETTING_TEAM_RUNNER_AGG_SELECT_POLICY_LACP_PRIO_STABLE;
pub use self::constants::SETTING_TEAM_RUNNER_AGG_SELECT_POLICY_PORT_CONFIG;
pub use self::constants::SETTING_TEAM_RUNNER_BROADCAST;
pub use self::constants::SETTING_TEAM_RUNNER_FAST_RATE;
pub use self::constants::SETTING_TEAM_RUNNER_HWADDR_POLICY;
pub use self::constants::SETTING_TEAM_RUNNER_HWADDR_POLICY_BY_ACTIVE;
pub use self::constants::SETTING_TEAM_RUNNER_HWADDR_POLICY_ONLY_ACTIVE;
pub use self::constants::SETTING_TEAM_RUNNER_HWADDR_POLICY_SAME_ALL;
pub use self::constants::SETTING_TEAM_RUNNER_LACP;
pub use self::constants::SETTING_TEAM_RUNNER_LOADBALANCE;
pub use self::constants::SETTING_TEAM_RUNNER_MIN_PORTS;
pub use self::constants::SETTING_TEAM_RUNNER_RANDOM;
pub use self::constants::SETTING_TEAM_RUNNER_ROUNDROBIN;
pub use self::constants::SETTING_TEAM_RUNNER_SYS_PRIO;
pub use self::constants::SETTING_TEAM_RUNNER_TX_BALANCER;
pub use self::constants::SETTING_TEAM_RUNNER_TX_BALANCER_INTERVAL;
pub use self::constants::SETTING_TEAM_RUNNER_TX_HASH;
pub use self::constants::SETTING_TEAM_SETTING_NAME;
pub use self::constants::SETTING_TUN_GROUP;
pub use self::constants::SETTING_TUN_MODE;
pub use self::constants::SETTING_TUN_MULTI_QUEUE;
pub use self::constants::SETTING_TUN_OWNER;
pub use self::constants::SETTING_TUN_PI;
pub use self::constants::SETTING_TUN_SETTING_NAME;
pub use self::constants::SETTING_TUN_VNET_HDR;
pub use self::constants::SETTING_USER_DATA;
pub use self::constants::SETTING_USER_SETTING_NAME;
pub use self::constants::SETTING_VLAN_EGRESS_PRIORITY_MAP;
pub use self::constants::SETTING_VLAN_FLAGS;
pub use self::constants::SETTING_VLAN_ID;
pub use self::constants::SETTING_VLAN_INGRESS_PRIORITY_MAP;
pub use self::constants::SETTING_VLAN_PARENT;
pub use self::constants::SETTING_VLAN_SETTING_NAME;
pub use self::constants::SETTING_VPN_DATA;
pub use self::constants::SETTING_VPN_PERSISTENT;
pub use self::constants::SETTING_VPN_SECRETS;
pub use self::constants::SETTING_VPN_SERVICE_TYPE;
pub use self::constants::SETTING_VPN_SETTING_NAME;
pub use self::constants::SETTING_VPN_TIMEOUT;
pub use self::constants::SETTING_VPN_USER_NAME;
pub use self::constants::SETTING_VXLAN_AGEING;
pub use self::constants::SETTING_VXLAN_DESTINATION_PORT;
pub use self::constants::SETTING_VXLAN_ID;
pub use self::constants::SETTING_VXLAN_L2_MISS;
pub use self::constants::SETTING_VXLAN_L3_MISS;
pub use self::constants::SETTING_VXLAN_LEARNING;
pub use self::constants::SETTING_VXLAN_LIMIT;
pub use self::constants::SETTING_VXLAN_LOCAL;
pub use self::constants::SETTING_VXLAN_PARENT;
pub use self::constants::SETTING_VXLAN_PROXY;
pub use self::constants::SETTING_VXLAN_REMOTE;
pub use self::constants::SETTING_VXLAN_RSC;
pub use self::constants::SETTING_VXLAN_SETTING_NAME;
pub use self::constants::SETTING_VXLAN_SOURCE_PORT_MAX;
pub use self::constants::SETTING_VXLAN_SOURCE_PORT_MIN;
pub use self::constants::SETTING_VXLAN_TOS;
pub use self::constants::SETTING_VXLAN_TTL;
pub use self::constants::SETTING_WIFI_P2P_PEER;
pub use self::constants::SETTING_WIFI_P2P_SETTING_NAME;
pub use self::constants::SETTING_WIFI_P2P_WFD_IES;
pub use self::constants::SETTING_WIFI_P2P_WPS_METHOD;
pub use self::constants::SETTING_WIMAX_MAC_ADDRESS;
pub use self::constants::SETTING_WIMAX_NETWORK_NAME;
pub use self::constants::SETTING_WIMAX_SETTING_NAME;
pub use self::constants::SETTING_WIRED_AUTO_NEGOTIATE;
pub use self::constants::SETTING_WIRED_CLONED_MAC_ADDRESS;
pub use self::constants::SETTING_WIRED_DUPLEX;
pub use self::constants::SETTING_WIRED_GENERATE_MAC_ADDRESS_MASK;
pub use self::constants::SETTING_WIRED_MAC_ADDRESS;
pub use self::constants::SETTING_WIRED_MAC_ADDRESS_BLACKLIST;
pub use self::constants::SETTING_WIRED_MTU;
pub use self::constants::SETTING_WIRED_PORT;
pub use self::constants::SETTING_WIRED_S390_NETTYPE;
pub use self::constants::SETTING_WIRED_S390_OPTIONS;
pub use self::constants::SETTING_WIRED_S390_SUBCHANNELS;
pub use self::constants::SETTING_WIRED_SETTING_NAME;
pub use self::constants::SETTING_WIRED_SPEED;
pub use self::constants::SETTING_WIRED_WAKE_ON_LAN;
pub use self::constants::SETTING_WIRED_WAKE_ON_LAN_PASSWORD;
pub use self::constants::SETTING_WIREGUARD_FWMARK;
pub use self::constants::SETTING_WIREGUARD_IP4_AUTO_DEFAULT_ROUTE;
pub use self::constants::SETTING_WIREGUARD_IP6_AUTO_DEFAULT_ROUTE;
pub use self::constants::SETTING_WIREGUARD_LISTEN_PORT;
pub use self::constants::SETTING_WIREGUARD_MTU;
pub use self::constants::SETTING_WIREGUARD_PEERS;
pub use self::constants::SETTING_WIREGUARD_PEER_ROUTES;
pub use self::constants::SETTING_WIREGUARD_PRIVATE_KEY;
pub use self::constants::SETTING_WIREGUARD_PRIVATE_KEY_FLAGS;
pub use self::constants::SETTING_WIREGUARD_SETTING_NAME;
pub use self::constants::SETTING_WIRELESS_BAND;
pub use self::constants::SETTING_WIRELESS_BSSID;
pub use self::constants::SETTING_WIRELESS_CHANNEL;
pub use self::constants::SETTING_WIRELESS_CLONED_MAC_ADDRESS;
pub use self::constants::SETTING_WIRELESS_GENERATE_MAC_ADDRESS_MASK;
pub use self::constants::SETTING_WIRELESS_HIDDEN;
pub use self::constants::SETTING_WIRELESS_MAC_ADDRESS;
pub use self::constants::SETTING_WIRELESS_MAC_ADDRESS_BLACKLIST;
pub use self::constants::SETTING_WIRELESS_MAC_ADDRESS_RANDOMIZATION;
pub use self::constants::SETTING_WIRELESS_MODE;
pub use self::constants::SETTING_WIRELESS_MODE_ADHOC;
pub use self::constants::SETTING_WIRELESS_MODE_AP;
pub use self::constants::SETTING_WIRELESS_MODE_INFRA;
#[cfg(any(feature = "v1_20", feature = "dox"))]
pub use self::constants::SETTING_WIRELESS_MODE_MESH;
pub use self::constants::SETTING_WIRELESS_MTU;
pub use self::constants::SETTING_WIRELESS_POWERSAVE;
pub use self::constants::SETTING_WIRELESS_RATE;
pub use self::constants::SETTING_WIRELESS_SECURITY_AUTH_ALG;
pub use self::constants::SETTING_WIRELESS_SECURITY_FILS;
pub use self::constants::SETTING_WIRELESS_SECURITY_GROUP;
pub use self::constants::SETTING_WIRELESS_SECURITY_KEY_MGMT;
pub use self::constants::SETTING_WIRELESS_SECURITY_LEAP_PASSWORD;
pub use self::constants::SETTING_WIRELESS_SECURITY_LEAP_PASSWORD_FLAGS;
pub use self::constants::SETTING_WIRELESS_SECURITY_LEAP_USERNAME;
pub use self::constants::SETTING_WIRELESS_SECURITY_PAIRWISE;
pub use self::constants::SETTING_WIRELESS_SECURITY_PMF;
pub use self::constants::SETTING_WIRELESS_SECURITY_PROTO;
pub use self::constants::SETTING_WIRELESS_SECURITY_PSK;
pub use self::constants::SETTING_WIRELESS_SECURITY_PSK_FLAGS;
pub use self::constants::SETTING_WIRELESS_SECURITY_SETTING_NAME;
pub use self::constants::SETTING_WIRELESS_SECURITY_WEP_KEY0;
pub use self::constants::SETTING_WIRELESS_SECURITY_WEP_KEY1;
pub use self::constants::SETTING_WIRELESS_SECURITY_WEP_KEY2;
pub use self::constants::SETTING_WIRELESS_SECURITY_WEP_KEY3;
pub use self::constants::SETTING_WIRELESS_SECURITY_WEP_KEY_FLAGS;
pub use self::constants::SETTING_WIRELESS_SECURITY_WEP_KEY_TYPE;
pub use self::constants::SETTING_WIRELESS_SECURITY_WEP_TX_KEYIDX;
pub use self::constants::SETTING_WIRELESS_SECURITY_WPS_METHOD;
pub use self::constants::SETTING_WIRELESS_SEEN_BSSIDS;
pub use self::constants::SETTING_WIRELESS_SETTING_NAME;
pub use self::constants::SETTING_WIRELESS_SSID;
pub use self::constants::SETTING_WIRELESS_TX_POWER;
pub use self::constants::SETTING_WIRELESS_WAKE_ON_WLAN;
pub use self::constants::SETTING_WPAN_CHANNEL;
pub use self::constants::SETTING_WPAN_MAC_ADDRESS;
pub use self::constants::SETTING_WPAN_PAGE;
pub use self::constants::SETTING_WPAN_PAN_ID;
pub use self::constants::SETTING_WPAN_SETTING_NAME;
pub use self::constants::SETTING_WPAN_SHORT_ADDRESS;
pub use self::constants::SRIOV_VF_ATTRIBUTE_MAC;
pub use self::constants::SRIOV_VF_ATTRIBUTE_MAX_TX_RATE;
pub use self::constants::SRIOV_VF_ATTRIBUTE_MIN_TX_RATE;
pub use self::constants::SRIOV_VF_ATTRIBUTE_SPOOF_CHECK;
pub use self::constants::SRIOV_VF_ATTRIBUTE_TRUST;
pub use self::constants::TEAM_LINK_WATCHER_ARP_PING;
pub use self::constants::TEAM_LINK_WATCHER_ETHTOOL;
pub use self::constants::TEAM_LINK_WATCHER_NSNA_PING;
pub use self::constants::VPN_CONNECTION_BANNER;
pub use self::constants::VPN_CONNECTION_VPN_STATE;
pub use self::constants::VPN_DBUS_PLUGIN_INTERFACE;
pub use self::constants::VPN_DBUS_PLUGIN_PATH;
pub use self::constants::VPN_EDITOR_PLUGIN_DESCRIPTION;
pub use self::constants::VPN_EDITOR_PLUGIN_NAME;
pub use self::constants::VPN_EDITOR_PLUGIN_SERVICE;
pub use self::constants::VPN_PLUGIN_CAN_PERSIST;
pub use self::constants::VPN_PLUGIN_CONFIG_BANNER;
pub use self::constants::VPN_PLUGIN_CONFIG_EXT_GATEWAY;
pub use self::constants::VPN_PLUGIN_CONFIG_HAS_IP4;
pub use self::constants::VPN_PLUGIN_CONFIG_HAS_IP6;
pub use self::constants::VPN_PLUGIN_CONFIG_MTU;
pub use self::constants::VPN_PLUGIN_CONFIG_PROXY_PAC;
pub use self::constants::VPN_PLUGIN_CONFIG_TUNDEV;
pub use self::constants::VPN_PLUGIN_INFO_FILENAME;
pub use self::constants::VPN_PLUGIN_INFO_KEYFILE;
pub use self::constants::VPN_PLUGIN_INFO_KF_GROUP_CONNECTION;
pub use self::constants::VPN_PLUGIN_INFO_KF_GROUP_GNOME;
pub use self::constants::VPN_PLUGIN_INFO_KF_GROUP_LIBNM;
pub use self::constants::VPN_PLUGIN_INFO_NAME;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_ADDRESS;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_DNS;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_DOMAIN;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_DOMAINS;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_INT_GATEWAY;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_MSS;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_NBNS;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_NEVER_DEFAULT;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_PREFIX;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_PRESERVE_ROUTES;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_PTP;
pub use self::constants::VPN_PLUGIN_IP4_CONFIG_ROUTES;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_ADDRESS;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_DNS;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_DOMAIN;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_DOMAINS;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_INT_GATEWAY;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_MSS;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_NEVER_DEFAULT;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_PREFIX;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_PRESERVE_ROUTES;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_PTP;
pub use self::constants::VPN_PLUGIN_IP6_CONFIG_ROUTES;
pub use self::constants::VPN_PLUGIN_OLD_DBUS_SERVICE_NAME;
pub use self::constants::VPN_PLUGIN_OLD_STATE;
pub use self::constants::VPN_SERVICE_PLUGIN_DBUS_SERVICE_NAME;
pub use self::constants::VPN_SERVICE_PLUGIN_DBUS_WATCH_PEER;
pub use self::constants::VPN_SERVICE_PLUGIN_STATE;
pub use self::constants::WIFI_P2P_PEER_FLAGS;
pub use self::constants::WIFI_P2P_PEER_HW_ADDRESS;
pub use self::constants::WIFI_P2P_PEER_LAST_SEEN;
pub use self::constants::WIFI_P2P_PEER_MANUFACTURER;
pub use self::constants::WIFI_P2P_PEER_MODEL;
pub use self::constants::WIFI_P2P_PEER_MODEL_NUMBER;
pub use self::constants::WIFI_P2P_PEER_NAME;
pub use self::constants::WIFI_P2P_PEER_SERIAL;
pub use self::constants::WIFI_P2P_PEER_STRENGTH;
pub use self::constants::WIFI_P2P_PEER_WFD_IES;
pub use self::constants::WIMAX_NSP_NAME;
pub use self::constants::WIMAX_NSP_NETWORK_TYPE;
pub use self::constants::WIMAX_NSP_SIGNAL_QUALITY;
pub use self::constants::WIREGUARD_PEER_ATTR_ALLOWED_IPS;
pub use self::constants::WIREGUARD_PEER_ATTR_ENDPOINT;
pub use self::constants::WIREGUARD_PEER_ATTR_PERSISTENT_KEEPALIVE;
pub use self::constants::WIREGUARD_PEER_ATTR_PRESHARED_KEY;
pub use self::constants::WIREGUARD_PEER_ATTR_PRESHARED_KEY_FLAGS;
pub use self::constants::WIREGUARD_PEER_ATTR_PUBLIC_KEY;

#[doc(hidden)]
pub mod traits {
    pub use super::ActiveConnectionExt;
    pub use super::ConnectionExt;
    pub use super::DeviceExt;
    pub use super::ObjectExt;
    pub use super::SecretAgentOldExt;
    pub use super::Setting8021xExt;
    pub use super::SettingAdslExt;
    pub use super::SettingBluetoothExt;
    pub use super::SettingBondExt;
    pub use super::SettingBridgeExt;
    pub use super::SettingBridgePortExt;
    pub use super::SettingCdmaExt;
    pub use super::SettingConnectionExt;
    pub use super::SettingDcbExt;
    pub use super::SettingExt;
    pub use super::SettingGsmExt;
    pub use super::SettingIP4ConfigExt;
    pub use super::SettingIP6ConfigExt;
    pub use super::SettingIPConfigExt;
    pub use super::SettingIPTunnelExt;
    pub use super::SettingInfinibandExt;
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    pub use super::SettingMacsecExt;
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub use super::SettingMacvlanExt;
    pub use super::SettingOlpcMeshExt;
    pub use super::SettingPppExt;
    pub use super::SettingPppoeExt;
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    pub use super::SettingProxyExt;
    pub use super::SettingSerialExt;
    pub use super::SettingTeamExt;
    pub use super::SettingTeamPortExt;
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub use super::SettingTunExt;
    pub use super::SettingVlanExt;
    pub use super::SettingVpnExt;
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub use super::SettingVxlanExt;
    pub use super::SettingWimaxExt;
    pub use super::SettingWiredExt;
    pub use super::SettingWirelessExt;
    pub use super::SettingWirelessSecurityExt;
    pub use super::VpnEditorExt;
    pub use super::VpnEditorPluginExt;
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub use super::VpnPluginInfoExt;
    pub use super::VpnPluginOldExt;
    pub use super::VpnServicePluginExt;
}
