// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use glib::{IsA, IsClassFor};
use gst;
use gst_ffi;
use AggregatorPad;

use std::ops;

pub trait AggregatorPadExtManual: 'static {
    fn get_segment(&self) -> gst::Segment;
}

impl<O: IsA<AggregatorPad>> AggregatorPadExtManual for O {
    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let stash = self.to_glib_none();
            let ptr: &ffi::GstAggregatorPad = &*stash.0;
            ::utils::MutexGuard::lock(&ptr.parent.object.lock);
            from_glib_none(&ptr.segment as *const gst_ffi::GstSegment)
        }
    }
}

#[repr(C)]
pub struct AggregatorPadClass(ffi::GstAggregatorPadClass);

unsafe impl IsClassFor for AggregatorPadClass {
    type Instance = ::AggregatorPad;
}

unsafe impl Send for AggregatorPadClass {}
unsafe impl Sync for AggregatorPadClass {}

impl ops::Deref for AggregatorPadClass {
    type Target = gst::PadClass;

    fn deref(&self) -> &Self::Target {
        self.upcast_ref()
    }
}

impl ops::DerefMut for AggregatorPadClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.upcast_ref_mut()
    }
}
