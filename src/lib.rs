use std::ffi::{CStr, c_char};

use gix_transport::client::http::{Transport, reqwest::Remote};

#[repr(C)]
pub enum GitTransport {
    Http(Transport<Remote>),
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn git_transport_http_new(uri: *const c_char) -> *mut GitTransport {
    let url = {
        let raw_url = unsafe { CStr::from_ptr(uri) };

        raw_url.to_str().unwrap()
    };

    let transport = Box::new(GitTransport::Http(Transport::new(
        gix_url::parse(url.into()).unwrap(),
        gix_transport::Protocol::V2,
        false,
    )));

    Box::into_raw(transport)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn git_transport_free(transport: *mut GitTransport) {
    let _ = unsafe { Box::from_raw(transport) };
}

#[cfg(test)]
mod tests {
    use crate::git_transport_free;

    use super::git_transport_http_new;

    #[test]
    fn transport() {
        unsafe {
            let transport =
                git_transport_http_new(c"https://github.com/zlib-ng/zlib-ng.git".as_ptr());
            git_transport_free(transport);
        }
    }
}
