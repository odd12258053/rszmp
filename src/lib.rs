use libc::{c_char, c_int, c_void};
use std::{mem, ptr, str};

pub mod ffi;

pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl Version {
    pub fn new() -> Self {
        let mut major = 0i32;
        let mut minor = 0i32;
        let mut patch = 0i32;
        unsafe {
            ffi::zmq_version(&mut major, &mut minor, &mut patch);
        }
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

pub enum SocketType {
    PAIR,
    PUB,
    SUB,
    REQ,
    REP,
    DEALER,
    ROUTER,
    PULL,
    PUSH,
    XPUB,
    XSUB,
    STREAM,
}

impl SocketType {
    pub fn code(&self) -> c_int {
        match self {
            SocketType::PAIR => ffi::ZMQ_PAIR,
            SocketType::PUB => ffi::ZMQ_PUB,
            SocketType::SUB => ffi::ZMQ_SUB,
            SocketType::REQ => ffi::ZMQ_REQ,
            SocketType::REP => ffi::ZMQ_REP,
            SocketType::DEALER => ffi::ZMQ_DEALER,
            SocketType::ROUTER => ffi::ZMQ_ROUTER,
            SocketType::PULL => ffi::ZMQ_PULL,
            SocketType::PUSH => ffi::ZMQ_PUSH,
            SocketType::XPUB => ffi::ZMQ_XPUB,
            SocketType::XSUB => ffi::ZMQ_XSUB,
            SocketType::STREAM => ffi::ZMQ_STREAM,
        }
    }
}

pub struct Message(ffi::ZmqMsgT);

impl Message {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
    pub fn close(&mut self) -> i32 {
        unsafe { ffi::zmq_msg_close(&mut self.0) }
    }
    pub fn with_capacity(size: usize) -> Self {
        Message(unsafe {
            let mut msg: ffi::ZmqMsgT = mem::zeroed();
            let rc = ffi::zmq_msg_init_size(&mut msg, size);
            assert_eq!(rc, 0);
            msg
        })
    }
    pub fn as_bytes(&mut self) -> Option<&[u8]> {
        unsafe {
            let data = ffi::zmq_msg_data(&mut self.0);
            let len = ffi::zmq_msg_size(&self.0);
            ptr::slice_from_raw_parts(data as *const u8, len).as_ref()
        }
    }
    pub fn as_str(&mut self) -> Option<&str> {
        self.as_bytes().and_then(|bytes| str::from_utf8(bytes).ok())
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Self(unsafe {
            let mut src = self.0;
            let mut msg: ffi::ZmqMsgT = mem::zeroed();
            ffi::zmq_msg_init(&mut msg);
            ffi::zmq_msg_copy(&mut msg, &mut src);
            msg
        })
    }
}

impl From<&[u8]> for Message {
    fn from(bytes: &[u8]) -> Self {
        let mut msg = Message::with_capacity(bytes.len());
        unsafe {
            ptr::copy_nonoverlapping(
                bytes.as_ptr() as *mut c_void,
                ffi::zmq_msg_data(&mut msg.0),
                bytes.len(),
            );
        }
        msg
    }
}

impl From<&str> for Message {
    fn from(msg: &str) -> Self {
        Self::from(msg.as_bytes())
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        self.close();
    }
}

pub struct SendFlag(i32);

impl SendFlag {
    pub fn new() -> Self {
        Self(0)
    }
    #[allow(non_snake_case)]
    pub fn DONTWAIT() -> Self {
        Self(ffi::ZMQ_DONTWAIT)
    }
    #[allow(non_snake_case)]
    pub fn SNDMORE() -> Self {
        Self(ffi::ZMQ_SNDMORE)
    }
    pub fn dontwait(&self) -> Self {
        Self(self.0 | ffi::ZMQ_DONTWAIT)
    }
    pub fn sndmore(&self) -> Self {
        Self(self.0 | ffi::ZMQ_SNDMORE)
    }
}

impl Default for SendFlag {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RecvFlag(i32);

impl RecvFlag {
    pub fn new() -> Self {
        Self(0)
    }
    #[allow(non_snake_case)]
    pub fn DONTWAIT() -> Self {
        Self(ffi::ZMQ_DONTWAIT)
    }
    pub fn dontwait(&self) -> Self {
        Self(self.0 | ffi::ZMQ_DONTWAIT)
    }
}

impl Default for RecvFlag {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! setsockopt {
    ($name: ident, bool, $option: path) => {
        pub fn $name(&mut self, flag: bool) -> i32 {
            unsafe {
                ffi::zmq_setsockopt(
                    self.0,
                    $option,
                    flag as i32 as *const c_void,
                    mem::size_of::<i32>(),
                )
            }
        }
    };
    ($name: ident, &str, $option: path) => {
        pub fn $name(&mut self, value: &str) -> i32 {
            let bytes = value.as_bytes();
            unsafe {
                ffi::zmq_setsockopt(
                    self.0,
                    $option,
                    bytes.as_ptr() as *const c_void,
                    bytes.len(),
                )
            }
        }
    };
    ($name: ident, &[u8], $option: path) => {
        pub fn $name(&mut self, value: &[u8]) -> i32 {
            unsafe {
                ffi::zmq_setsockopt(
                    self.0,
                    $option,
                    value.as_ptr() as *const c_void,
                    value.len(),
                )
            }
        }
    };
    ($name: ident, $ty: ty, $option: path) => {
        pub fn $name(&mut self, value: $ty) -> i32 {
            unsafe {
                ffi::zmq_setsockopt(
                    self.0,
                    $option,
                    value as *const c_void,
                    mem::size_of::<$ty>(),
                )
            }
        }
    };
}

pub struct Socket(*mut c_void);

impl Socket {
    pub fn bind(&self, addr: &str) -> i32 {
        unsafe { ffi::zmq_bind(self.0, addr.as_bytes().to_vec().as_ptr() as *const c_char) }
    }
    pub fn unbind(&self, addr: &str) -> i32 {
        unsafe { ffi::zmq_unbind(self.0, addr.as_bytes().to_vec().as_ptr() as *const c_char) }
    }
    pub fn connect(&self, addr: &str) -> i32 {
        unsafe { ffi::zmq_connect(self.0, addr.as_bytes().to_vec().as_ptr() as *const c_char) }
    }
    pub fn disconnect(&self, addr: &str) -> i32 {
        unsafe { ffi::zmq_disconnect(self.0, addr.as_bytes().to_vec().as_ptr() as *const c_char) }
    }
    pub fn close(&self) -> i32 {
        unsafe { ffi::zmq_close(self.0) }
    }
    pub fn send(&self, msg: &[u8], flags: SendFlag) -> i32 {
        let len = msg.len();
        unsafe { ffi::zmq_send(self.0, msg.as_ptr() as *const c_void, len, flags.0) }
    }
    pub fn send_msg(&self, msg: &mut Message, flags: SendFlag) -> i32 {
        unsafe { ffi::zmq_msg_send(&mut msg.0, self.0, flags.0) }
    }
    pub fn recv(&self, bytes: &mut [u8], flags: RecvFlag) -> i32 {
        let len = bytes.len();
        unsafe { ffi::zmq_recv(self.0, bytes.as_ptr() as *mut c_void, len, flags.0) }
    }
    pub fn recv_msg(&self, msg: &mut Message, flags: RecvFlag) -> i32 {
        unsafe { ffi::zmq_msg_recv(&mut msg.0, self.0, flags.0) }
    }
    // TODO; handle error.
    pub fn get_routing_id(&self) -> Option<Vec<u8>> {
        let mut size = 255;
        let mut buffer = [0u8; 255];
        unsafe {
            let rc = ffi::zmq_getsockopt(
                self.0,
                ffi::ZMQ_ROUTING_ID,
                buffer.as_mut_ptr() as *mut c_void,
                &mut size,
            );
            assert_eq!(rc, 0);
        }
        if size > 0 {
            Some(buffer[0..size].to_vec())
        } else {
            None
        }
    }
    setsockopt!(set_affinity, u64, ffi::ZMQ_AFFINITY);
    setsockopt!(set_backlog, i32, ffi::ZMQ_BACKLOG);
    setsockopt!(set_bindtodevice, &str, ffi::ZMQ_BINDTODEVICE);
    setsockopt!(set_connect_routing_id, &[u8], ffi::ZMQ_CONNECT_ROUTING_ID);
    setsockopt!(set_conflate, bool, ffi::ZMQ_CONFLATE);
    setsockopt!(set_connect_timeout, i32, ffi::ZMQ_CONNECT_TIMEOUT);
    setsockopt!(set_curve_publickey, &[u8], ffi::ZMQ_CURVE_PUBLICKEY);
    setsockopt!(set_curve_secretkey, &[u8], ffi::ZMQ_CURVE_SECRETKEY);
    setsockopt!(set_curve_server, bool, ffi::ZMQ_CURVE_SERVER);
    setsockopt!(set_curve_serverkey, &[u8], ffi::ZMQ_CURVE_SERVERKEY);
    setsockopt!(set_gssapi_plaintext, bool, ffi::ZMQ_GSSAPI_PLAINTEXT);
    setsockopt!(set_gssapi_principal, &str, ffi::ZMQ_GSSAPI_PRINCIPAL);
    setsockopt!(set_gssapi_server, bool, ffi::ZMQ_GSSAPI_SERVER);
    setsockopt!(
        set_gssapi_service_principal,
        &str,
        ffi::ZMQ_GSSAPI_SERVICE_PRINCIPAL
    );
    setsockopt!(
        set_gssapi_service_principal_nametype,
        i32,
        ffi::ZMQ_GSSAPI_SERVICE_PRINCIPAL_NAMETYPE
    );
    setsockopt!(
        set_gssapi_principal_nametype,
        i32,
        ffi::ZMQ_GSSAPI_PRINCIPAL_NAMETYPE
    );
    setsockopt!(set_handshake_ivl, i32, ffi::ZMQ_HANDSHAKE_IVL);
    setsockopt!(set_heartbeat_ivl, i32, ffi::ZMQ_HEARTBEAT_IVL);
    setsockopt!(set_heartbeat_timeout, i32, ffi::ZMQ_HEARTBEAT_TIMEOUT);
    setsockopt!(set_heartbeat_ttl, i32, ffi::ZMQ_HEARTBEAT_TTL);
    setsockopt!(set_immediate, bool, ffi::ZMQ_IMMEDIATE);
    setsockopt!(set_invert_matching, bool, ffi::ZMQ_INVERT_MATCHING);
    setsockopt!(set_ipv6, bool, ffi::ZMQ_IPV6);
    setsockopt!(set_linger, i32, ffi::ZMQ_LINGER);
    setsockopt!(set_maxmsgsize, i64, ffi::ZMQ_MAXMSGSIZE);
    setsockopt!(set_multicast_hops, i32, ffi::ZMQ_MULTICAST_HOPS);
    setsockopt!(set_multicast_maxtpdu, i32, ffi::ZMQ_MULTICAST_MAXTPDU);
    setsockopt!(set_plain_password, &str, ffi::ZMQ_PLAIN_PASSWORD);
    setsockopt!(set_plain_server, bool, ffi::ZMQ_PLAIN_SERVER);
    setsockopt!(set_plain_username, &str, ffi::ZMQ_PLAIN_USERNAME);
    setsockopt!(set_use_fd, i32, ffi::ZMQ_USE_FD);
    setsockopt!(set_probe_router, bool, ffi::ZMQ_PROBE_ROUTER);
    setsockopt!(set_rate, i32, ffi::ZMQ_RATE);
    setsockopt!(set_rcvbuf, i32, ffi::ZMQ_RCVBUF);
    setsockopt!(set_rcvhwm, i32, ffi::ZMQ_RCVHWM);
    setsockopt!(set_rcvtimeo, i32, ffi::ZMQ_RCVTIMEO);
    setsockopt!(set_reconnect_ivl, i32, ffi::ZMQ_RECONNECT_IVL);
    setsockopt!(set_reconnect_ivl_max, i32, ffi::ZMQ_RECONNECT_IVL_MAX);
    setsockopt!(set_recovery_ivl, i32, ffi::ZMQ_RECOVERY_IVL);
    setsockopt!(set_req_correlate, bool, ffi::ZMQ_REQ_CORRELATE);
    setsockopt!(set_req_relaxed, bool, ffi::ZMQ_REQ_RELAXED);
    setsockopt!(set_router_handover, bool, ffi::ZMQ_ROUTER_HANDOVER);
    setsockopt!(set_router_mandatory, bool, ffi::ZMQ_ROUTER_MANDATORY);
    setsockopt!(set_router_raw, bool, ffi::ZMQ_ROUTER_RAW);
    setsockopt!(set_routing_id, &[u8], ffi::ZMQ_ROUTING_ID);
    setsockopt!(set_sndbuf, i32, ffi::ZMQ_SNDBUF);
    setsockopt!(set_sndhwm, i32, ffi::ZMQ_SNDHWM);
    setsockopt!(set_sndtimeo, i32, ffi::ZMQ_SNDTIMEO);
    setsockopt!(set_socks_proxy, &str, ffi::ZMQ_SOCKS_PROXY);
    setsockopt!(set_stream_notify, bool, ffi::ZMQ_STREAM_NOTIFY);
    setsockopt!(set_subscribe, &[u8], ffi::ZMQ_SUBSCRIBE);
    setsockopt!(set_tcp_keepalive, i32, ffi::ZMQ_TCP_KEEPALIVE);
    setsockopt!(set_tcp_keepalive_cnt, i32, ffi::ZMQ_TCP_KEEPALIVE_CNT);
    setsockopt!(set_tcp_keepalive_idle, i32, ffi::ZMQ_TCP_KEEPALIVE_IDLE);
    setsockopt!(set_tcp_keepalive_intvl, i32, ffi::ZMQ_TCP_KEEPALIVE_INTVL);
    setsockopt!(set_tcp_maxrt, i32, ffi::ZMQ_TCP_MAXRT);
    setsockopt!(set_tos, i32, ffi::ZMQ_TOS);
    setsockopt!(set_unsubscribe, &[u8], ffi::ZMQ_UNSUBSCRIBE);
    setsockopt!(set_xpub_verbose, bool, ffi::ZMQ_XPUB_VERBOSE);
    setsockopt!(set_xpub_verboser, bool, ffi::ZMQ_XPUB_VERBOSER);
    setsockopt!(set_xpub_manual, bool, ffi::ZMQ_XPUB_MANUAL);
    setsockopt!(set_xpub_nodrop, bool, ffi::ZMQ_XPUB_NODROP);
    setsockopt!(set_xpub_welcome_msg, &[u8], ffi::ZMQ_XPUB_WELCOME_MSG);
    setsockopt!(set_zap_domain, &str, ffi::ZMQ_ZAP_DOMAIN);
    setsockopt!(set_vmci_buffer_size, u64, ffi::ZMQ_VMCI_BUFFER_SIZE);
    setsockopt!(set_vmci_buffer_min_size, u64, ffi::ZMQ_VMCI_BUFFER_MIN_SIZE);
    setsockopt!(set_vmci_buffer_max_size, u64, ffi::ZMQ_VMCI_BUFFER_MAX_SIZE);
    setsockopt!(set_vmci_connect_timeout, i32, ffi::ZMQ_VMCI_CONNECT_TIMEOUT);
}

impl Drop for Socket {
    fn drop(&mut self) {
        self.close();
    }
}

pub fn proxy(frontend: &Socket, backend: &Socket) -> i32 {
    unsafe { ffi::zmq_proxy(frontend.0, backend.0, ptr::null_mut()) }
}

pub struct Context(*mut c_void);

impl Context {
    pub fn new() -> Self {
        Context(unsafe { ffi::zmq_ctx_new() })
    }
    pub fn socket(&self, stype: SocketType) -> Socket {
        Socket(unsafe { ffi::zmq_socket(self.0, stype.code()) })
    }
    pub fn terminate(&self) -> i32 {
        unsafe { ffi::zmq_ctx_term(self.0) }
    }
    pub fn shutdown(&self) -> i32 {
        unsafe { ffi::zmq_ctx_shutdown(self.0) }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.terminate();
    }
}

pub struct PollItem(ffi::ZmqPollitemT);

impl PollItem {
    pub fn new() -> Self {
        PollItem(unsafe { mem::zeroed() })
    }
    pub fn from_socket(socket: &mut Socket, events: i16) -> Self {
        let mut instance = Self::new();
        instance.0.socket = socket.0;
        instance.0.events = events;
        instance
    }
    pub fn get_revents(&self) -> i16 {
        self.0.revents
    }
}

impl Default for PollItem {
    fn default() -> Self {
        Self::new()
    }
}

pub fn poll(items: &mut [PollItem], timeout: i64) -> i32 {
    unsafe {
        ffi::zmq_poll(
            items.as_mut_ptr() as *mut ffi::ZmqPollitemT,
            items.len() as c_int,
            timeout,
        )
    }
}

pub enum ErrNo {
    ENOTSUP,
    EPROTONOSUPPORT,
    ENOBUFS,
    ENETDOWN,
    EADDRINUSE,
    EADDRNOTAVAIL,
    ECONNREFUSED,
    EINPROGRESS,
    ENOTSOCK,
    EMSGSIZE,
    EAFNOSUPPORT,
    ENETUNREACH,
    ECONNABORTED,
    ECONNRESET,
    ENOTCONN,
    ETIMEDOUT,
    EHOSTUNREACH,
    ENETRESET,
    EFSM,
    ENOCOMPATPROTO,
    ETERM,
    EMTHREAD,
}

impl From<i32> for ErrNo {
    fn from(errno: i32) -> Self {
        match errno {
            ffi::ENOTSUP => ErrNo::ENOTSUP,
            ffi::EPROTONOSUPPORT => ErrNo::EPROTONOSUPPORT,
            ffi::ENOBUFS => ErrNo::ENOBUFS,
            ffi::ENETDOWN => ErrNo::ENETDOWN,
            ffi::EADDRINUSE => ErrNo::EADDRINUSE,
            ffi::EADDRNOTAVAIL => ErrNo::EADDRNOTAVAIL,
            ffi::ECONNREFUSED => ErrNo::ECONNREFUSED,
            ffi::EINPROGRESS => ErrNo::EINPROGRESS,
            ffi::ENOTSOCK => ErrNo::ENOTSOCK,
            ffi::EMSGSIZE => ErrNo::EMSGSIZE,
            ffi::EAFNOSUPPORT => ErrNo::EAFNOSUPPORT,
            ffi::ENETUNREACH => ErrNo::ENETUNREACH,
            ffi::ECONNABORTED => ErrNo::ECONNABORTED,
            ffi::ECONNRESET => ErrNo::ECONNRESET,
            ffi::ENOTCONN => ErrNo::ENOTCONN,
            ffi::ETIMEDOUT => ErrNo::ETIMEDOUT,
            ffi::EHOSTUNREACH => ErrNo::EHOSTUNREACH,
            ffi::ENETRESET => ErrNo::ENETRESET,
            ffi::EFSM => ErrNo::EFSM,
            ffi::ENOCOMPATPROTO => ErrNo::ENOCOMPATPROTO,
            ffi::ETERM => ErrNo::ETERM,
            ffi::EMTHREAD => ErrNo::EMTHREAD,
            other => {
                panic!("Unknown error: {}", other)
            }
        }
    }
}

impl ErrNo {
    pub fn get_errno() -> ErrNo {
        ErrNo::from(unsafe { ffi::zmq_errno() } as i32)
    }
    pub fn errno(&self) -> i32 {
        match self {
            ErrNo::ENOTSUP => ffi::ENOTSUP,
            ErrNo::EPROTONOSUPPORT => ffi::EPROTONOSUPPORT,
            ErrNo::ENOBUFS => ffi::ENOBUFS,
            ErrNo::ENETDOWN => ffi::ENETDOWN,
            ErrNo::EADDRINUSE => ffi::EADDRINUSE,
            ErrNo::EADDRNOTAVAIL => ffi::EADDRNOTAVAIL,
            ErrNo::ECONNREFUSED => ffi::ECONNREFUSED,
            ErrNo::EINPROGRESS => ffi::EINPROGRESS,
            ErrNo::ENOTSOCK => ffi::ENOTSOCK,
            ErrNo::EMSGSIZE => ffi::EMSGSIZE,
            ErrNo::EAFNOSUPPORT => ffi::EAFNOSUPPORT,
            ErrNo::ENETUNREACH => ffi::ENETUNREACH,
            ErrNo::ECONNABORTED => ffi::ECONNABORTED,
            ErrNo::ECONNRESET => ffi::ECONNRESET,
            ErrNo::ENOTCONN => ffi::ENOTCONN,
            ErrNo::ETIMEDOUT => ffi::ETIMEDOUT,
            ErrNo::EHOSTUNREACH => ffi::EHOSTUNREACH,
            ErrNo::ENETRESET => ffi::ENETRESET,
            ErrNo::EFSM => ffi::EFSM,
            ErrNo::ENOCOMPATPROTO => ffi::ENOCOMPATPROTO,
            ErrNo::ETERM => ffi::ETERM,
            ErrNo::EMTHREAD => ffi::EMTHREAD,
        }
    }
    pub fn err_msg(&self) -> String {
        get_err_msg(self.errno())
    }
}

pub fn get_err_msg(errno: i32) -> String {
    unsafe {
        std::ffi::CStr::from_ptr(ffi::zmq_strerror(errno))
            .to_str()
            .unwrap()
            .to_string()
    }
}
