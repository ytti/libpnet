extern crate libc;

use std::io;
use std::time::Duration;

pub type CSocket = libc::SOCKET;
pub type BufLen = i32;


fn errno() -> i32 {
    io::Error::last_os_error().raw_os_error().unwrap()
}

pub unsafe fn close(sock: CSocket) {
    let _ = libc::closesocket(sock);
}

#[inline]
pub fn retry<F>(f: &mut F) -> libc::c_int
    where F: FnMut() -> libc::c_int
{
    loop {
        let minus1 = -1;
        let ret = f();
        if ret != minus1 || errno() as isize != libc::WSAEINTR as isize {
            return ret;
        }
    }
}

pub fn duration_to_timeval(dur: Duration) -> libc::timeval {
    libc::timeval {
        tv_sec: dur.as_secs() as libc::c_long,
        tv_usec: (dur.subsec_nanos() / 1_000) as libc::c_long
    }
}
