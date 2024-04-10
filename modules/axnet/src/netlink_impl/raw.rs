
use alloc::vec;
use smoltcp::storage::RingBuffer;
use axerrno::AxError;

pub struct RawNetlinkSocket<'a>{
    buffer: RingBuffer<'a, u8>,
}

const NETLINK_BUFFER_SIZE: usize = 64 * 1024;

impl<'a> RawNetlinkSocket<'a> {
    pub fn new() -> Self {
        RawNetlinkSocket {
            buffer: RingBuffer::new(vec![0; NETLINK_BUFFER_SIZE]),
        }
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, AxError> {
        let len = data.len();
        if len > NETLINK_BUFFER_SIZE {
            return Err(AxError::InvalidInput);
        }
        let len = self.buffer.enqueue_slice(data);
        Ok(len)
    }

    pub fn recv(&mut self, data: &mut [u8]) -> Result<usize, AxError> {
        let len = self.buffer.dequeue_slice(data);
        Ok(len)
    }

}