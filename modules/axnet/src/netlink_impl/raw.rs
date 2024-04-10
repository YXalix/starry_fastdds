
use alloc::vec::Vec;
use alloc::vec;
use smoltcp::storage::RingBuffer;
use axerrno::AxError;

pub struct RawNetlinkSocket<'a>{
    buffer: RingBuffer<'a, u8>,
    queue: Vec<usize>,
}

const NETLINK_BUFFER_SIZE: usize = 64 * 1024;

impl<'a> RawNetlinkSocket<'a> {
    pub fn new() -> Self {
        RawNetlinkSocket {
            buffer: RingBuffer::new(vec![0; NETLINK_BUFFER_SIZE]),
            queue: Vec::new(),
        }
    }

    pub fn send(&mut self, size: usize) -> &mut [u8] {
        self.queue.push(size);
        self.buffer.enqueue_many(size)
    }

    pub fn recv(&mut self, data: &mut [u8]) -> Result<usize, AxError> {
        if let Some(len) = self.queue.pop() {
            let buf = self.buffer.dequeue_many(len);
            data[..len].copy_from_slice(buf);
            return Ok(len);
        } else {
            return Ok(0);
        }
    }

}