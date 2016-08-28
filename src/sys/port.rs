//! Implements event ports API for Solaris-like operating systems.

mod ffi {
    pub use libc::{c_int, c_uint, c_ushort, uintptr_t, c_void, timespec};
    use super::{PortAlert, EventSource};

    #[repr(C)]
    pub struct PortEvent {
        portev_events: c_int,       // Event data is source specific
        portev_source: EventSource, // Event source
        portev_pad: c_ushort,       // Port internal use
        portev_object: uintptr_t,   // Source specific object
        portev_user: *const c_void, // User cookie
    }

    #[repr(C)]
    pub struct PortNotify {
        portnfy_port: c_int,         // Bind request(s) to port
        portnfy_user: *const c_void, // User defined
    }

    extern {
        pub fn port_create() -> c_int;

        pub fn port_associate(
            port: c_int,
            source: c_int,
            object: uintptr_t,
            events: c_int,
            user: *const c_void) -> c_int;

        pub fn port_dissociate(
            port: c_int,
            source: c_int,
            object: uintptr_t) -> c_int;

        pub fn port_get(
            port: c_int,
            pe: *mut PortEvent,
            timeout: *const timespec) -> c_int;

        pub fn port_getn(
            port: c_int,
            list: *mut PortEvent, // port_event_t list[]
            max: c_uint,
            nget: *const c_uint,
            timeout: *const timespec) -> c_int;

        pub fn port_send(
            port: c_int,
            events: c_int,
            user: *const c_void) -> c_int;

        pub fn port_alert(
            port: c_int,
            flags: PortAlert,
            events: c_int,
            user: *const c_void) -> c_int;
    }
}

// port sources
#[repr(i16)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventSource {
    PORT_SOURCE_AIO   = 1,
    PORT_SOURCE_TIMER = 2,
    PORT_SOURCE_USER  = 3,
    PORT_SOURCE_FD    = 4,
    PORT_SOURCE_ALERT = 5,
    PORT_SOURCE_MQ    = 6,
    PORT_SOURCE_FILE  = 7,
}

bitflags!(
    #[repr(C)]
    flags PortAlert: u16 {
        const PORT_ALERT_SET     = 0x01,
        const PORT_ALERT_UPDATE  = 0x02,
        const PORT_ALERT_INVALID = (PORT_ALERT_SET.bits | PORT_ALERT_UPDATE.bits)
    }
);
