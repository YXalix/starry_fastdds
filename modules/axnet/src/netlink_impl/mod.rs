mod netlink;

use alloc::vec::Vec;
use alloc::vec;
use lazy_init::LazyInit;
use smoltcp::storage::{PacketBuffer, PacketMetadata};
use axsync::Mutex;


use self::netlink::NetlinkMessageHeader;
pub use self::netlink::NetlinkSocket;

const RTM_GETLINK: u16 = 18;

const RTM_GETADDR: u16 = 22;

const NETLINK_BUFFER_SIZE: usize = 64 * 1024;

static NETLINK_SOCKET_SET: LazyInit<NetlinkSockSetWrapper> = LazyInit::new();
struct NetlinkSockSetWrapper<'a>(Mutex<Vec<RawNetlinkSocket<'a>>>);

pub struct RawNetlinkSocket<'a>{
    rx_buffer: PacketBuffer<'a, PacketMetadata<NetlinkMessageHeader>>,
    tx_buffer: PacketBuffer<'a, PacketMetadata<NetlinkMessageHeader>>,
}


impl<'a> RawNetlinkSocket<'a> {
    fn new() -> Self {
        RawNetlinkSocket {
            rx_buffer: PacketBuffer::new(
                vec![PacketMetadata::EMPTY; 64],
                vec![0; NETLINK_BUFFER_SIZE],
            ),
            tx_buffer: PacketBuffer::new(
                vec![PacketMetadata::EMPTY; 64],
                vec![0; NETLINK_BUFFER_SIZE],
            ),
        }
    }

    fn send(&mut self, buf: &[u8], pid: u64) -> usize {
        todo!("RawNetlinkSocket::send")
    }

}

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