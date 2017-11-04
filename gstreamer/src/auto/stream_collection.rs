// This file was generated by gir (12a28ac+) from gir-files (???)
// DO NOT EDIT

use Object;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Stream;
use ffi;
use glib::Value;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct StreamCollection(Object<ffi::GstStreamCollection>): Object;

    match fn {
        get_type => || ffi::gst_stream_collection_get_type(),
    }
}

impl StreamCollection {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new<'a, P: Into<Option<&'a str>>>(upstream_id: P) -> StreamCollection {
        assert_initialized_main_thread!();
        let upstream_id = upstream_id.into();
        let upstream_id = upstream_id.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_stream_collection_new(upstream_id.0))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn add_stream(&self, stream: &Stream) -> bool {
        unsafe {
            from_glib(ffi::gst_stream_collection_add_stream(self.to_glib_none().0, stream.to_glib_full()))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_size(&self) -> u32 {
        unsafe {
            ffi::gst_stream_collection_get_size(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream(&self, index: u32) -> Option<Stream> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_stream(self.to_glib_none().0, index))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_upstream_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_upstream_id(self.to_glib_none().0))
        }
    }

    pub fn get_property_upstream_id(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "upstream-id".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_upstream_id(&self, upstream_id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "upstream-id".to_glib_none().0, Value::from(upstream_id).to_glib_none().0);
        }
    }

    //pub fn connect_stream_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored p0: GObject.ParamSpec
    //}

    pub fn connect_property_upstream_id_notify<F: Fn(&StreamCollection) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&StreamCollection) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::upstream-id",
                transmute(notify_upstream_id_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for StreamCollection {}
unsafe impl Sync for StreamCollection {}

unsafe extern "C" fn notify_upstream_id_trampoline(this: *mut ffi::GstStreamCollection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&StreamCollection) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
