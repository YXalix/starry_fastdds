use axfs::api::FileIO;
use bitflags::bitflags;
use axerrno::{AxResult, AxError};
extern crate alloc;
use alloc::sync::Arc;
use core::sync::atomic::{Ordering, AtomicU64};
use axlog::error;
bitflags! {
    /// 定义eventfd的flag类型
    #[derive(Clone, Copy, Debug)]
    pub struct EventFdFlag: i32 {
        /// 为eventfd设置非阻塞标志
        const EFD_NONBLOCK = 0x800;
        /// 为eventfd设置cloexec标志
        const EFD_CLOEXEC = 0x80000;
        /// 为eventfd设置semaphore标志
        const EFD_SEMAPHORE = 1;
    }
}

pub fn make_eventctx(init_value: u64, flag: EventFdFlag) -> Arc<EventFdCtx> {
    Arc::new(EventFdCtx::new(init_value, flag))
}


/// 定义eventfd
pub struct EventFdCtx {
    /// eventfd的值
    count: AtomicU64,
    /// eventfd的flag
    flag: EventFdFlag,
}

impl EventFdCtx {
    /// 新建一个eventfd
    pub fn new(init_value: u64, flag: EventFdFlag) -> Self {
        Self {
            count: AtomicU64::new(init_value),
            flag,
        }
    }
}

impl FileIO for EventFdCtx {
    /// 读取eventfd的值
    fn read(&self, _buf: &mut [u8]) -> AxResult<usize> {
        if self.count.load(Ordering::Relaxed) == 0{
            return Err(AxError::WouldBlock);
        }
        let cnt = self.count.load(Ordering::Relaxed);
        _buf.copy_from_slice(cnt.to_ne_bytes().as_ref());
        self.count.store(0, Ordering::Relaxed);
        Ok(8)
    }

    /// 写入eventfd的值
    fn write(&self, buf: &[u8]) -> AxResult<usize> {
        let cnt = u64::from_ne_bytes(buf.try_into().unwrap());
        error!("write eventfd input cnt is : {}", cnt);
        self.count.fetch_add(cnt, Ordering::Relaxed);
        Ok(8)
    }
    
    fn readable(&self) -> bool {
        if self.count.load(Ordering::Relaxed) == 0 {
            false
        } else {
            true
        }
    }
    
    fn writable(&self) -> bool {
        true
    }
    
    fn executable(&self) -> bool {
        todo!()
    }
    
    fn get_type(&self) -> axfs::api::FileIOType {
        axfs::api::FileIOType::FileDesc
    }
}
