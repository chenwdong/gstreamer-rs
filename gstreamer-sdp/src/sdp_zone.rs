// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use std::mem;

use ffi;
use glib::translate::*;

#[repr(C)]
pub struct SDPZone(pub(crate) ffi::GstSDPZone);

impl SDPZone {
    pub fn new(time: &str, typed_time: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut zone = mem::zeroed();
            ffi::gst_sdp_zone_set(
                &mut zone,
                time.to_glib_none().0,
                typed_time.to_glib_none().0,
            );
            SDPZone(zone)
        }
    }

    pub fn time(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.time).to_str().unwrap() }
    }

    pub fn typed_time(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.typed_time).to_str().unwrap() }
    }
}

impl Clone for SDPZone {
    fn clone(&self) -> Self {
        SDPZone::new(self.time(), self.typed_time())
    }
}

impl Drop for SDPZone {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_sdp_zone_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for SDPZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPZone")
            .field("time", &self.time())
            .field("typed_time", &self.typed_time())
            .finish()
    }
}
