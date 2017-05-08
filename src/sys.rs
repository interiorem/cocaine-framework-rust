use std::io::Error;

use tokio_core::net::TcpStream;

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix {
    use std::io::Error;
    use std::os::unix::io::RawFd;

    use libc;

    use nix::sys::socket;

    pub fn sendmsg(fd: RawFd, iov: &[&[u8]]) -> Result<usize, Error> {
        #[cfg(target_os = "linux")]
        let len = iov.len() as usize;
        #[cfg(target_os = "macos")]
        let len = iov.len() as i32;

        let mhdr = libc::msghdr {
            msg_name: 0 as *mut libc::c_void,
            msg_namelen: 0,
            msg_iov: iov.as_ptr() as *mut libc::iovec,
            msg_iovlen: len,
            msg_control: 0 as *mut libc::c_void,
            msg_controllen: 0,
            msg_flags: 0,
        };

        let rc = unsafe {
            libc::sendmsg(fd, &mhdr, socket::MsgFlags::empty().bits())
        };

        if rc < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(rc as usize)
        }
    }
}

pub trait SendAll {
    fn send_all(&mut self, iov: &[&[u8]]) -> Result<usize, Error>;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl SendAll for TcpStream {
    fn send_all(&mut self, iov: &[&[u8]]) -> Result<usize, Error> {
        use std::os::unix::io::AsRawFd;

        unix::sendmsg(self.as_raw_fd(), iov)
    }
}

#[cfg(target_os = "windows")]
impl SendAll for TcpStream {
    fn send_all(&mut self, iov: &[&[u8]]) -> Result<usize, Error> {
        use std::io::Write;

        let mut nsend = 0;
        for buf in iov {
            let len = self.write(buf)?;
            nsend += len;

            if len != buf.len() {
                break;
            }
        }

        Ok(nsend)
    }
}
