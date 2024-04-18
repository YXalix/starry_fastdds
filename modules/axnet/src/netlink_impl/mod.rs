mod netlink;
mod raw;

use alloc::vec::{self, Vec};
use lazy_init::LazyInit;
use axsync::Mutex;
use smoltcp::iface::SocketSet;

pub use self::raw::RawNetlinkSocket;
pub use self::netlink::NetlinkSocket;


const RTM_GETLINK: u16 = 18;

const RTM_GETADDR: u16 = 22;


type NetlinkSockSet<'a> = Vec<Option<RawNetlinkSocket<'a>>>;

static NETLINK_SOCKET_SET: LazyInit<NetlinkSockSetWrapper> = LazyInit::new();
struct NetlinkSockSetWrapper<'a>(Mutex<NetlinkSockSet<'a>>);


impl<'a> NetlinkSockSetWrapper<'a> {
    fn new() -> Self {
        NetlinkSockSetWrapper(Mutex::new(Vec::new()))
    }

    pub fn new_netlink_socket() -> RawNetlinkSocket<'a> {
        RawNetlinkSocket::new()
    }

    pub fn add(&self, socket: RawNetlinkSocket<'a>) -> usize {
        let mut set = self.0.lock();
        for (i, slot) in set.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(socket);
                debug!("socket {}: created", i);
                return i;
            }
        }
        set.push(Some(socket));
        debug!("socket {}: created", set.len() - 1);
        set.len() - 1
    }
    
    pub fn with_socket<R, F>(&self, handle: usize, f: F) -> R
    where
        F: FnOnce(&RawNetlinkSocket) -> R,
    {
        let set = self.0.lock();
        let socket = set.get(handle).unwrap();
        let socket = socket.as_ref().unwrap();
        f(socket)
    }

    pub fn with_socket_mut<R, F>(&self, handle: usize, f: F) -> R
    where
        F: FnOnce(&mut RawNetlinkSocket<'a>) -> R,
    {
        let mut set = self.0.lock();
        let socket = set.get_mut(handle).unwrap();
        let socket = socket.as_mut().unwrap();
        f(socket)
    }

    pub fn remove(&self, handle: usize) {
        let mut set = self.0.lock();
        set[handle] = None;
        debug!("socket {}: removed", handle);
    }


}

pub(crate) fn init() {
    NETLINK_SOCKET_SET.init_by(NetlinkSockSetWrapper::new());
}