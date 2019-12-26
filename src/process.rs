use std::io::{Error};

pub type Pid = libc::pid_t;

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
/// wrapper over libc fork will work only on unix-like incl Mac. Copied from ipc-channel
/// # Safety
/// Undefined behavior in the other process if fork returns -1 in one of branches.
pub unsafe fn fork<F: FnOnce()>(child_func: F) -> Pid {
    match libc::fork() {
        -1 => panic!("Fork failed: {}", Error::last_os_error()),
        0 => {
            child_func();
            libc::exit(0);
        },
        pid => pid,
    }
}

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
pub trait Wait {
    fn wait(self);
}

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
impl Wait for Pid {
    fn wait(self) {
        unsafe {
            libc::waitpid(self, std::ptr::null_mut(), 0);
        }
    }
}


#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
pub trait Kill {
    fn kill(self);
}

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
impl Kill for Pid {
    fn kill(self) {
        unsafe {
            libc::kill(self, libc::SIGHUP);
        }
    }
}
