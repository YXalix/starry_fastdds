mod netlink;
mod raw;

use alloc::vec::Vec;
use lazy_init::LazyInit;
use axsync::Mutex;

pub use self::raw::RawNetlinkSocket;
pub use self::netlink::NetlinkSocket;

const RTM_GETLINK: u16 = 18;

const RTM_GETADDR: u16 = 22;


static NETLINK_SOCKET_SET: LazyInit<NetlinkSockSetWrapper> = LazyInit::new();
struct NetlinkSockSetWrapper<'a>(Mutex<Vec<RawNetlinkSocket<'a>>>);


impl<'a> NetlinkSockSetWrapper<'a> {
    fn new() -> Self {
        NetlinkSockSetWrapper(Mutex::new(Vec::new()))
    }

    pub fn new_netlink_socket() -> RawNetlinkSocket<'a> {
        RawNetlinkSocket::new()
    }

    pub fn add(&self, socket: RawNetlinkSocket<'a>) -> usize {
        self.0.lock().push(socket);
        let handle = self.0.lock().len() - 1;
        debug!("socket {}: created", handle);
        handle
    }
    
    pub fn with_socket<R, F>(&self, handle: usize, f: F) -> R
    where
        F: FnOnce(&RawNetlinkSocket) -> R,
    {
        let set = self.0.lock();
        let socket = set.get(handle).unwrap();
        f(socket)
    }

    pub fn with_socket_mut<R, F>(&self, handle: usize, f: F) -> R
    where
        F: FnOnce(&mut RawNetlinkSocket<'a>) -> R,
    {
        let mut set = self.0.lock();
        let socket = set.get_mut(handle).unwrap();
        f(socket)
    }

    pub fn remove(&self, handle: usize) {
        let mut set = self.0.lock();
        set.remove(handle);
        debug!("socket {}: removed", handle);
    }


}

pub(crate) fn init() {
    NETLINK_SOCKET_SET.init_by(NetlinkSockSetWrapper::new());
}