// This code is from version 4.3.4.
use libc::{c_char, c_int, c_long, c_short, c_uchar, c_void, size_t};

pub const ZMQ_HAUSNUMERO: c_int = 156384712;
pub const EFSM: c_int = ZMQ_HAUSNUMERO + 51;
pub const ENOCOMPATPROTO: c_int = ZMQ_HAUSNUMERO + 52;
pub const ETERM: c_int = ZMQ_HAUSNUMERO + 53;
pub const EMTHREAD: c_int = ZMQ_HAUSNUMERO + 54;

// pub const ENOTSUP: c_int = ZMQ_HAUSNUMERO + 1;
// pub const EPROTONOSUPPORT: c_int = ZMQ_HAUSNUMERO + 2;
// pub const ENOBUFS: c_int = ZMQ_HAUSNUMERO + 3;
// pub const ENETDOWN: c_int = ZMQ_HAUSNUMERO + 4;
// pub const EADDRINUSE: c_int = ZMQ_HAUSNUMERO + 5;
// pub const EADDRNOTAVAIL: c_int = ZMQ_HAUSNUMERO + 6;
// pub const ECONNREFUSED: c_int = ZMQ_HAUSNUMERO + 7;
// pub const EINPROGRESS: c_int = ZMQ_HAUSNUMERO + 8;
// pub const ENOTSOCK: c_int = ZMQ_HAUSNUMERO + 9;
// pub const EMSGSIZE: c_int = ZMQ_HAUSNUMERO + 10;
// pub const EAFNOSUPPORT: c_int = ZMQ_HAUSNUMERO + 11;
// pub const ENETUNREACH: c_int = ZMQ_HAUSNUMERO + 12;
// pub const ECONNABORTED: c_int = ZMQ_HAUSNUMERO + 13;
// pub const ECONNRESET: c_int = ZMQ_HAUSNUMERO + 14;
// pub const ENOTCONN: c_int = ZMQ_HAUSNUMERO + 15;
// pub const ETIMEDOUT: c_int = ZMQ_HAUSNUMERO + 16;
// pub const EHOSTUNREACH: c_int = ZMQ_HAUSNUMERO + 17;
// pub const ENETRESET: c_int = ZMQ_HAUSNUMERO + 18;

pub const ENOTSUP: c_int = libc::ENOTSUP;
pub const EPROTONOSUPPORT: c_int = libc::EPROTONOSUPPORT;
pub const ENOBUFS: c_int = libc::ENOBUFS;
pub const ENETDOWN: c_int = libc::ENETDOWN;
pub const EADDRINUSE: c_int = libc::EADDRINUSE;
pub const EADDRNOTAVAIL: c_int = libc::EADDRNOTAVAIL;
pub const ECONNREFUSED: c_int = libc::ECONNREFUSED;
pub const EINPROGRESS: c_int = libc::EINPROGRESS;
pub const ENOTSOCK: c_int = libc::ENOTSOCK;
pub const EMSGSIZE: c_int = libc::EMSGSIZE;
pub const EAFNOSUPPORT: c_int = libc::EAFNOSUPPORT;
pub const ENETUNREACH: c_int = libc::ENETUNREACH;
pub const ECONNABORTED: c_int = libc::ECONNABORTED;
pub const ECONNRESET: c_int = libc::ECONNRESET;
pub const ENOTCONN: c_int = libc::ENOTCONN;
pub const ETIMEDOUT: c_int = libc::ETIMEDOUT;
pub const EHOSTUNREACH: c_int = libc::EHOSTUNREACH;
pub const ENETRESET: c_int = libc::ENETRESET;

pub const ZMQ_PAIR: c_int = 0;
pub const ZMQ_PUB: c_int = 1;
pub const ZMQ_SUB: c_int = 2;
pub const ZMQ_REQ: c_int = 3;
pub const ZMQ_REP: c_int = 4;
pub const ZMQ_DEALER: c_int = 5;
pub const ZMQ_ROUTER: c_int = 6;
pub const ZMQ_PULL: c_int = 7;
pub const ZMQ_PUSH: c_int = 8;
pub const ZMQ_XPUB: c_int = 9;
pub const ZMQ_XSUB: c_int = 10;
pub const ZMQ_STREAM: c_int = 11;

pub const ZMQ_DONTWAIT: c_int = 1;
pub const ZMQ_SNDMORE: c_int = 2;

pub const ZMQ_DEFINED_STDINT: u32 = 1;
pub const ZMQ_IO_THREADS: u32 = 1;
pub const ZMQ_MAX_SOCKETS: u32 = 2;
pub const ZMQ_SOCKET_LIMIT: u32 = 3;
pub const ZMQ_THREAD_PRIORITY: u32 = 3;
pub const ZMQ_THREAD_SCHED_POLICY: u32 = 4;
pub const ZMQ_MAX_MSGSZ: u32 = 5;
pub const ZMQ_MSG_T_SIZE: u32 = 6;
pub const ZMQ_THREAD_AFFINITY_CPU_ADD: u32 = 7;
pub const ZMQ_THREAD_AFFINITY_CPU_REMOVE: u32 = 8;
pub const ZMQ_THREAD_NAME_PREFIX: u32 = 9;
pub const ZMQ_IO_THREADS_DFLT: u32 = 1;
pub const ZMQ_MAX_SOCKETS_DFLT: u32 = 1023;
pub const ZMQ_THREAD_PRIORITY_DFLT: i32 = -1;
pub const ZMQ_THREAD_SCHED_POLICY_DFLT: i32 = -1;
pub const ZMQ_AFFINITY: u32 = 4;
pub const ZMQ_ROUTING_ID: c_int = 5;
pub const ZMQ_SUBSCRIBE: u32 = 6;
pub const ZMQ_UNSUBSCRIBE: u32 = 7;
pub const ZMQ_RATE: u32 = 8;
pub const ZMQ_RECOVERY_IVL: u32 = 9;
pub const ZMQ_SNDBUF: u32 = 11;
pub const ZMQ_RCVBUF: u32 = 12;
pub const ZMQ_RCVMORE: u32 = 13;
pub const ZMQ_FD: u32 = 14;
pub const ZMQ_EVENTS: u32 = 15;
pub const ZMQ_TYPE: u32 = 16;
pub const ZMQ_LINGER: u32 = 17;
pub const ZMQ_RECONNECT_IVL: u32 = 18;
pub const ZMQ_BACKLOG: u32 = 19;
pub const ZMQ_RECONNECT_IVL_MAX: u32 = 21;
pub const ZMQ_MAXMSGSIZE: u32 = 22;
pub const ZMQ_SNDHWM: u32 = 23;
pub const ZMQ_RCVHWM: u32 = 24;
pub const ZMQ_MULTICAST_HOPS: u32 = 25;
pub const ZMQ_RCVTIMEO: u32 = 27;
pub const ZMQ_SNDTIMEO: u32 = 28;
pub const ZMQ_LAST_ENDPOINT: u32 = 32;
pub const ZMQ_ROUTER_MANDATORY: u32 = 33;
pub const ZMQ_TCP_KEEPALIVE: u32 = 34;
pub const ZMQ_TCP_KEEPALIVE_CNT: u32 = 35;
pub const ZMQ_TCP_KEEPALIVE_IDLE: u32 = 36;
pub const ZMQ_TCP_KEEPALIVE_INTVL: u32 = 37;
pub const ZMQ_IMMEDIATE: u32 = 39;
pub const ZMQ_XPUB_VERBOSE: u32 = 40;
pub const ZMQ_ROUTER_RAW: u32 = 41;
pub const ZMQ_IPV6: u32 = 42;
pub const ZMQ_MECHANISM: u32 = 43;
pub const ZMQ_PLAIN_SERVER: u32 = 44;
pub const ZMQ_PLAIN_USERNAME: u32 = 45;
pub const ZMQ_PLAIN_PASSWORD: u32 = 46;
pub const ZMQ_CURVE_SERVER: u32 = 47;
pub const ZMQ_CURVE_PUBLICKEY: u32 = 48;
pub const ZMQ_CURVE_SECRETKEY: u32 = 49;
pub const ZMQ_CURVE_SERVERKEY: u32 = 50;
pub const ZMQ_PROBE_ROUTER: u32 = 51;
pub const ZMQ_REQ_CORRELATE: u32 = 52;
pub const ZMQ_REQ_RELAXED: u32 = 53;
pub const ZMQ_CONFLATE: u32 = 54;
pub const ZMQ_ZAP_DOMAIN: u32 = 55;
pub const ZMQ_ROUTER_HANDOVER: u32 = 56;
pub const ZMQ_TOS: u32 = 57;
pub const ZMQ_CONNECT_ROUTING_ID: u32 = 61;
pub const ZMQ_GSSAPI_SERVER: u32 = 62;
pub const ZMQ_GSSAPI_PRINCIPAL: u32 = 63;
pub const ZMQ_GSSAPI_SERVICE_PRINCIPAL: u32 = 64;
pub const ZMQ_GSSAPI_PLAINTEXT: u32 = 65;
pub const ZMQ_HANDSHAKE_IVL: u32 = 66;
pub const ZMQ_SOCKS_PROXY: u32 = 68;
pub const ZMQ_XPUB_NODROP: u32 = 69;
pub const ZMQ_BLOCKY: u32 = 70;
pub const ZMQ_XPUB_MANUAL: u32 = 71;
pub const ZMQ_XPUB_WELCOME_MSG: u32 = 72;
pub const ZMQ_STREAM_NOTIFY: u32 = 73;
pub const ZMQ_INVERT_MATCHING: u32 = 74;
pub const ZMQ_HEARTBEAT_IVL: u32 = 75;
pub const ZMQ_HEARTBEAT_TTL: u32 = 76;
pub const ZMQ_HEARTBEAT_TIMEOUT: u32 = 77;
pub const ZMQ_XPUB_VERBOSER: u32 = 78;
pub const ZMQ_CONNECT_TIMEOUT: u32 = 79;
pub const ZMQ_TCP_MAXRT: u32 = 80;
pub const ZMQ_THREAD_SAFE: u32 = 81;
pub const ZMQ_MULTICAST_MAXTPDU: u32 = 84;
pub const ZMQ_VMCI_BUFFER_SIZE: u32 = 85;
pub const ZMQ_VMCI_BUFFER_MIN_SIZE: u32 = 86;
pub const ZMQ_VMCI_BUFFER_MAX_SIZE: u32 = 87;
pub const ZMQ_VMCI_CONNECT_TIMEOUT: u32 = 88;
pub const ZMQ_USE_FD: u32 = 89;
pub const ZMQ_GSSAPI_PRINCIPAL_NAMETYPE: u32 = 90;
pub const ZMQ_GSSAPI_SERVICE_PRINCIPAL_NAMETYPE: u32 = 91;
pub const ZMQ_BINDTODEVICE: u32 = 92;
pub const ZMQ_MORE: u32 = 1;
pub const ZMQ_SHARED: u32 = 3;
pub const ZMQ_NULL: u32 = 0;
pub const ZMQ_PLAIN: u32 = 1;
pub const ZMQ_CURVE: u32 = 2;
pub const ZMQ_GSSAPI: u32 = 3;
pub const ZMQ_GROUP_MAX_LENGTH: u32 = 255;
pub const ZMQ_GSSAPI_NT_HOSTBASED: u32 = 0;
pub const ZMQ_GSSAPI_NT_USER_NAME: u32 = 1;
pub const ZMQ_GSSAPI_NT_KRB5_PRINCIPAL: u32 = 2;
pub const ZMQ_EVENT_CONNECTED: u32 = 1;
pub const ZMQ_EVENT_CONNECT_DELAYED: u32 = 2;
pub const ZMQ_EVENT_CONNECT_RETRIED: u32 = 4;
pub const ZMQ_EVENT_LISTENING: u32 = 8;
pub const ZMQ_EVENT_BIND_FAILED: u32 = 16;
pub const ZMQ_EVENT_ACCEPTED: u32 = 32;
pub const ZMQ_EVENT_ACCEPT_FAILED: u32 = 64;
pub const ZMQ_EVENT_CLOSED: u32 = 128;
pub const ZMQ_EVENT_CLOSE_FAILED: u32 = 256;
pub const ZMQ_EVENT_DISCONNECTED: u32 = 512;
pub const ZMQ_EVENT_MONITOR_STOPPED: u32 = 1024;
pub const ZMQ_EVENT_ALL: u32 = 65535;
pub const ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL: u32 = 2048;
pub const ZMQ_EVENT_HANDSHAKE_SUCCEEDED: u32 = 4096;
pub const ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL: u32 = 8192;
pub const ZMQ_EVENT_HANDSHAKE_FAILED_AUTH: u32 = 16384;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_UNSPECIFIED: u32 = 268435456;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_UNEXPECTED_COMMAND: u32 = 268435457;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_INVALID_SEQUENCE: u32 = 268435458;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_KEY_EXCHANGE: u32 = 268435459;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_UNSPECIFIED: u32 = 268435473;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_MESSAGE: u32 = 268435474;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_HELLO: u32 = 268435475;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_INITIATE: u32 = 268435476;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_ERROR: u32 = 268435477;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_READY: u32 = 268435478;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_MALFORMED_COMMAND_WELCOME: u32 = 268435479;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_INVALID_METADATA: u32 = 268435480;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_CRYPTOGRAPHIC: u32 = 285212673;
pub const ZMQ_PROTOCOL_ERROR_ZMTP_MECHANISM_MISMATCH: u32 = 285212674;
pub const ZMQ_PROTOCOL_ERROR_ZAP_UNSPECIFIED: u32 = 536870912;
pub const ZMQ_PROTOCOL_ERROR_ZAP_MALFORMED_REPLY: u32 = 536870913;
pub const ZMQ_PROTOCOL_ERROR_ZAP_BAD_REQUEST_ID: u32 = 536870914;
pub const ZMQ_PROTOCOL_ERROR_ZAP_BAD_VERSION: u32 = 536870915;
pub const ZMQ_PROTOCOL_ERROR_ZAP_INVALID_STATUS_CODE: u32 = 536870916;
pub const ZMQ_PROTOCOL_ERROR_ZAP_INVALID_METADATA: u32 = 536870917;
pub const ZMQ_PROTOCOL_ERROR_WS_UNSPECIFIED: u32 = 805306368;

pub const ZMQ_POLLIN: c_short = 1;
pub const ZMQ_POLLOUT: c_short = 2;
pub const ZMQ_POLLERR: c_short = 4;
pub const ZMQ_POLLPRI: c_short = 8;

pub const ZMQ_POLLITEMS_DFLT: u32 = 16;
pub const ZMQ_HAS_CAPABILITIES: u32 = 1;

#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct ZmqMsgT {
    pub __: [c_uchar; 64usize],
}

pub type ZmqFreeFn = Option<unsafe extern "C" fn(data_: *mut c_void, hint_: *mut c_void)>;
pub type ZmqTimerFn = Option<unsafe extern "C" fn(timer_id: c_int, arg: *mut c_void)>;
pub type ZmqFdT = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZmqPollitemT {
    pub socket: *mut c_void,
    pub fd: ZmqFdT,
    pub events: c_short,
    pub revents: c_short,
}

#[link(name = "zmq")]
extern "C" {
    pub fn zmq_errno() -> c_int;
    pub fn zmq_strerror(errnum_: c_int) -> *const c_char;
    pub fn zmq_version(major_: *mut c_int, minor_: *mut c_int, patch_: *mut c_int);
    pub fn zmq_ctx_new() -> *mut c_void;
    pub fn zmq_ctx_term(context_: *mut c_void) -> c_int;
    pub fn zmq_ctx_shutdown(context_: *mut c_void) -> c_int;
    pub fn zmq_ctx_set(context_: *mut c_void, option_: c_int, optval_: c_int) -> c_int;
    pub fn zmq_ctx_get(context_: *mut c_void, option_: c_int) -> c_int;
    pub fn zmq_msg_init(msg_: *mut ZmqMsgT) -> c_int;
    pub fn zmq_msg_init_size(msg_: *mut ZmqMsgT, size_: size_t) -> c_int;
    pub fn zmq_msg_init_data(
        msg_: *mut ZmqMsgT,
        data_: *mut c_void,
        size_: size_t,
        ffn_: ZmqFreeFn,
        hint_: *mut c_void,
    ) -> c_int;
    pub fn zmq_msg_send(msg_: *mut ZmqMsgT, s_: *mut c_void, flags_: c_int) -> c_int;
    pub fn zmq_msg_recv(msg_: *mut ZmqMsgT, s_: *mut c_void, flags_: c_int) -> c_int;
    pub fn zmq_msg_close(msg_: *mut ZmqMsgT) -> c_int;
    pub fn zmq_msg_move(dest_: *mut ZmqMsgT, src_: *mut ZmqMsgT) -> c_int;
    pub fn zmq_msg_copy(dest_: *mut ZmqMsgT, src_: *mut ZmqMsgT) -> c_int;
    pub fn zmq_msg_data(msg_: *mut ZmqMsgT) -> *mut c_void;
    pub fn zmq_msg_size(msg_: *const ZmqMsgT) -> size_t;
    pub fn zmq_msg_more(msg_: *const ZmqMsgT) -> c_int;
    pub fn zmq_msg_get(msg_: *const ZmqMsgT, property_: c_int) -> c_int;
    pub fn zmq_msg_set(msg_: *mut ZmqMsgT, property_: c_int, optval_: c_int) -> c_int;
    pub fn zmq_msg_gets(msg_: *const ZmqMsgT, property_: *const c_char) -> *const c_char;
    pub fn zmq_socket(arg1: *mut c_void, type_: c_int) -> *mut c_void;
    pub fn zmq_close(s_: *mut c_void) -> c_int;
    pub fn zmq_setsockopt(
        s_: *mut c_void,
        option_: c_int,
        optval_: *const c_void,
        optvallen_: size_t,
    ) -> c_int;
    pub fn zmq_getsockopt(
        s_: *mut c_void,
        option_: c_int,
        optval_: *mut c_void,
        optvallen_: *mut size_t,
    ) -> c_int;
    pub fn zmq_bind(s_: *mut c_void, addr_: *const c_char) -> c_int;
    pub fn zmq_connect(s_: *mut c_void, addr_: *const c_char) -> c_int;
    pub fn zmq_unbind(s_: *mut c_void, addr_: *const c_char) -> c_int;
    pub fn zmq_disconnect(s_: *mut c_void, addr_: *const c_char) -> c_int;
    pub fn zmq_send(s_: *mut c_void, buf_: *const c_void, len_: size_t, flags_: c_int) -> c_int;
    pub fn zmq_send_const(
        s_: *mut c_void,
        buf_: *const c_void,
        len_: size_t,
        flags_: c_int,
    ) -> c_int;
    pub fn zmq_recv(s_: *mut c_void, buf_: *mut c_void, len_: size_t, flags_: c_int) -> c_int;
    pub fn zmq_socket_monitor(s_: *mut c_void, addr_: *const c_char, events_: c_int) -> c_int;
    pub fn zmq_poll(items_: *mut ZmqPollitemT, nitems_: c_int, timeout_: c_long) -> c_int;
    pub fn zmq_proxy(frontend_: *mut c_void, backend_: *mut c_void, capture_: *mut c_void)
        -> c_int;
    pub fn zmq_proxy_steerable(
        frontend_: *mut c_void,
        backend_: *mut c_void,
        capture_: *mut c_void,
        control_: *mut c_void,
    ) -> c_int;
    pub fn zmq_has(capability_: *const c_char) -> c_int;
    pub fn zmq_z85_encode(dest_: *mut c_char, data_: *const u8, size_: size_t) -> *mut c_char;
    pub fn zmq_z85_decode(dest_: *mut u8, string_: *const c_char) -> *mut u8;
    pub fn zmq_curve_keypair(z85_public_key_: *mut c_char, z85_secret_key_: *mut c_char) -> c_int;
    pub fn zmq_curve_public(z85_public_key_: *mut c_char, z85_secret_key_: *const c_char) -> c_int;
    pub fn zmq_atomic_counter_new() -> *mut c_void;
    pub fn zmq_atomic_counter_set(counter_: *mut c_void, value_: c_int);
    pub fn zmq_atomic_counter_inc(counter_: *mut c_void) -> c_int;
    pub fn zmq_atomic_counter_dec(counter_: *mut c_void) -> c_int;
    pub fn zmq_atomic_counter_value(counter_: *mut c_void) -> c_int;
    pub fn zmq_atomic_counter_destroy(counter_p_: *mut *mut c_void);
    pub fn zmq_timers_new() -> *mut c_void;
    pub fn zmq_timers_destroy(timers_p: *mut *mut c_void) -> c_int;
    pub fn zmq_timers_add(
        timers: *mut c_void,
        interval: size_t,
        handler: ZmqTimerFn,
        arg: *mut c_void,
    ) -> c_int;
    pub fn zmq_timers_cancel(timers: *mut c_void, timer_id: c_int) -> c_int;
    pub fn zmq_timers_set_interval(timers: *mut c_void, timer_id: c_int, interval: size_t)
        -> c_int;
    pub fn zmq_timers_reset(timers: *mut c_void, timer_id: c_int) -> c_int;
    pub fn zmq_timers_timeout(timers: *mut c_void) -> c_long;
    pub fn zmq_timers_execute(timers: *mut c_void) -> c_int;
}
