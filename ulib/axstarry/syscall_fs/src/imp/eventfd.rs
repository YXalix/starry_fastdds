use syscall_utils::{SyscallResult, SyscallError};
use crate::ctype::eventfd::{EventFdFlag, make_eventctx};
use axprocess::current_process;
extern crate alloc;

/// For eventfd2
/// For epoll_create1, If flags is 0, then, other than the fact that the obsolete size argument is dropped, epoll_create1()
///  is the same as epoll_create().
///
/// If flag equals to EPOLL_CLOEXEC, than set the cloexec flag for the fd
pub fn syscall_eventfd2(count: usize, flags: usize) -> SyscallResult {
    let eventflag = EventFdFlag::from_bits_truncate(flags as i32);
    let ctx = make_eventctx(count as u64, eventflag);
    let process = current_process();
    let mut fd_table = process.fd_manager.fd_table.lock();
    if let Ok(num) = process.alloc_fd(&mut fd_table) {
        fd_table[num] = Some(ctx);
        Ok(num as isize)
    } else {
        // ErrorNo::EMFILE as isize
        Err(SyscallError::EMFILE)
    }
}