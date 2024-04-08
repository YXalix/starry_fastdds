use core::fmt::Display;

use super::{NETLINK_SOCKET_SET, NetlinkSockSetWrapper};
use axerrno::{ax_err, ax_err_type, AxError, AxResult};

// use axprocess::current_process;

/// A netlink socket.
pub struct NetlinkSocket {
    handle: usize,
    seq: u32,
    pid: u64,
}

impl Drop for NetlinkSocket {
    fn drop(&mut self) {
        NETLINK_SOCKET_SET.remove(self.handle);
    }
}


impl NetlinkSocket {
    /// Create a new netlink socket.
    pub fn new(pid: u64) -> Self {
        let sock = NetlinkSockSetWrapper::new_netlink_socket();
        let handle = NETLINK_SOCKET_SET.add(sock);
        NetlinkSocket {
            handle,
            seq: 0,
            pid: pid
        }
    }

    /// Transmits data in the given buffer.
    pub fn send(&self, buf: &[u8]) -> AxResult<usize> {
        let header = NetlinkMessageHeader::create_from_buf(buf);
        match &header.ty {
            RTM_GETLINK => {
                self.send_getlink()
            }
            RTM_GETADDR => {
                self.send_getaddr()
            }
            _ => {
                unimplemented!("NetlinkSocket::send: unsupported message type")
            }
        }
    }
    
    fn send_getlink(&self) -> AxResult<usize> {
        
    }
    
    fn send_getaddr(&self) -> AxResult<usize> {
        todo!("NetlinkSocket::send_getaddr")
    } 
}


#[derive(Clone)]
pub struct NetlinkMessageHeader {
    len: u32,
    ty: u16,
    flags: u16,
    seq: u32,
    pid: u32,
}


impl Display for NetlinkMessageHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NetlinkMessageHeader {{ len: {}, ty: {}, flags: {}, seq: {}, pid: {} }}", self.len, self.ty, self.flags, self.seq, self.pid)
    }
}


impl NetlinkMessageHeader {
    pub fn new(len: u32, ty: u16, flags: u16, seq: u32, pid: u32) -> Self {
        Self {
            len,
            ty,
            flags,
            seq,
            pid,
        }
    }

    pub const fn empty() -> Self {
        Self {
            len: 0,
            ty: 0,
            flags: 0,
            seq: 0,
            pid: 0,
        }
    }


    pub fn create_from_buf(buf: &[u8]) -> Self {
        let len = u32::from_ne_bytes(buf[0..4].try_into().unwrap());
        let ty = u16::from_ne_bytes(buf[4..6].try_into().unwrap());
        let flags = u16::from_ne_bytes(buf[6..8].try_into().unwrap());
        let seq = u32::from_ne_bytes(buf[8..12].try_into().unwrap());
        let pid = u32::from_ne_bytes(buf[12..16].try_into().unwrap());
        Self {
            len,
            ty,
            flags,
            seq,
            pid,
        }
    }
}