// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use BaseSinkClass;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct BaseSink(Object<ffi::GstBaseSink, ffi::GstBaseSinkClass, BaseSinkClass>): [
        gst::Element => gst_ffi::GstElement,
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_base_sink_get_type(),
    }
}

unsafe impl Send for BaseSink {}
unsafe impl Sync for BaseSink {}

pub trait BaseSinkExt: 'static {
    //fn do_preroll(&self, obj: /*Ignored*/&gst::MiniObject) -> gst::FlowReturn;

    fn get_blocksize(&self) -> u32;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_drop_out_of_segment(&self) -> bool;

    fn get_last_sample(&self) -> Option<gst::Sample>;

    fn get_latency(&self) -> gst::ClockTime;

    fn get_max_bitrate(&self) -> u64;

    fn get_max_lateness(&self) -> i64;

    fn get_render_delay(&self) -> gst::ClockTime;

    fn get_sync(&self) -> bool;

    fn get_throttle_time(&self) -> u64;

    fn get_ts_offset(&self) -> gst::ClockTimeDiff;

    fn is_async_enabled(&self) -> bool;

    fn is_last_sample_enabled(&self) -> bool;

    fn is_qos_enabled(&self) -> bool;

    fn query_latency(&self) -> Option<(bool, bool, gst::ClockTime, gst::ClockTime)>;

    fn set_async_enabled(&self, enabled: bool);

    fn set_blocksize(&self, blocksize: u32);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_drop_out_of_segment(&self, drop_out_of_segment: bool);

    fn set_last_sample_enabled(&self, enabled: bool);

    fn set_max_bitrate(&self, max_bitrate: u64);

    fn set_max_lateness(&self, max_lateness: i64);

    fn set_qos_enabled(&self, enabled: bool);

    fn set_render_delay(&self, delay: gst::ClockTime);

    fn set_sync(&self, sync: bool);

    fn set_throttle_time(&self, throttle: u64);

    fn set_ts_offset(&self, offset: gst::ClockTimeDiff);

    fn wait_clock(&self, time: gst::ClockTime) -> (gst::ClockReturn, gst::ClockTimeDiff);

    fn get_property_async(&self) -> bool;

    fn set_property_async(&self, async: bool);

    fn get_property_enable_last_sample(&self) -> bool;

    fn set_property_enable_last_sample(&self, enable_last_sample: bool);

    fn get_property_qos(&self) -> bool;

    fn set_property_qos(&self, qos: bool);

    fn connect_property_async_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enable_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_bitrate_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_lateness_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_render_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sync_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_throttle_time_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ts_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BaseSink>> BaseSinkExt for O {
    //fn do_preroll(&self, obj: /*Ignored*/&gst::MiniObject) -> gst::FlowReturn {
    //    unsafe { TODO: call ffi::gst_base_sink_do_preroll() }
    //}

    fn get_blocksize(&self) -> u32 {
        unsafe {
            ffi::gst_base_sink_get_blocksize(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_drop_out_of_segment(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_sink_get_drop_out_of_segment(self.to_glib_none().0))
        }
    }

    fn get_last_sample(&self) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_base_sink_get_last_sample(self.to_glib_none().0))
        }
    }

    fn get_latency(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_base_sink_get_latency(self.to_glib_none().0))
        }
    }

    fn get_max_bitrate(&self) -> u64 {
        unsafe {
            ffi::gst_base_sink_get_max_bitrate(self.to_glib_none().0)
        }
    }

    fn get_max_lateness(&self) -> i64 {
        unsafe {
            ffi::gst_base_sink_get_max_lateness(self.to_glib_none().0)
        }
    }

    fn get_render_delay(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_base_sink_get_render_delay(self.to_glib_none().0))
        }
    }

    fn get_sync(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_sink_get_sync(self.to_glib_none().0))
        }
    }

    fn get_throttle_time(&self) -> u64 {
        unsafe {
            ffi::gst_base_sink_get_throttle_time(self.to_glib_none().0)
        }
    }

    fn get_ts_offset(&self) -> gst::ClockTimeDiff {
        unsafe {
            ffi::gst_base_sink_get_ts_offset(self.to_glib_none().0)
        }
    }

    fn is_async_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_sink_is_async_enabled(self.to_glib_none().0))
        }
    }

    fn is_last_sample_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_sink_is_last_sample_enabled(self.to_glib_none().0))
        }
    }

    fn is_qos_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_sink_is_qos_enabled(self.to_glib_none().0))
        }
    }

    fn query_latency(&self) -> Option<(bool, bool, gst::ClockTime, gst::ClockTime)> {
        unsafe {
            let mut live = mem::uninitialized();
            let mut upstream_live = mem::uninitialized();
            let mut min_latency = mem::uninitialized();
            let mut max_latency = mem::uninitialized();
            let ret = from_glib(ffi::gst_base_sink_query_latency(self.to_glib_none().0, &mut live, &mut upstream_live, &mut min_latency, &mut max_latency));
            if ret { Some((from_glib(live), from_glib(upstream_live), from_glib(min_latency), from_glib(max_latency))) } else { None }
        }
    }

    fn set_async_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_base_sink_set_async_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_blocksize(&self, blocksize: u32) {
        unsafe {
            ffi::gst_base_sink_set_blocksize(self.to_glib_none().0, blocksize);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_drop_out_of_segment(&self, drop_out_of_segment: bool) {
        unsafe {
            ffi::gst_base_sink_set_drop_out_of_segment(self.to_glib_none().0, drop_out_of_segment.to_glib());
        }
    }

    fn set_last_sample_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_base_sink_set_last_sample_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_max_bitrate(&self, max_bitrate: u64) {
        unsafe {
            ffi::gst_base_sink_set_max_bitrate(self.to_glib_none().0, max_bitrate);
        }
    }

    fn set_max_lateness(&self, max_lateness: i64) {
        unsafe {
            ffi::gst_base_sink_set_max_lateness(self.to_glib_none().0, max_lateness);
        }
    }

    fn set_qos_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_base_sink_set_qos_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_render_delay(&self, delay: gst::ClockTime) {
        unsafe {
            ffi::gst_base_sink_set_render_delay(self.to_glib_none().0, delay.to_glib());
        }
    }

    fn set_sync(&self, sync: bool) {
        unsafe {
            ffi::gst_base_sink_set_sync(self.to_glib_none().0, sync.to_glib());
        }
    }

    fn set_throttle_time(&self, throttle: u64) {
        unsafe {
            ffi::gst_base_sink_set_throttle_time(self.to_glib_none().0, throttle);
        }
    }

    fn set_ts_offset(&self, offset: gst::ClockTimeDiff) {
        unsafe {
            ffi::gst_base_sink_set_ts_offset(self.to_glib_none().0, offset);
        }
    }

    fn wait_clock(&self, time: gst::ClockTime) -> (gst::ClockReturn, gst::ClockTimeDiff) {
        unsafe {
            let mut jitter = mem::uninitialized();
            let ret = from_glib(ffi::gst_base_sink_wait_clock(self.to_glib_none().0, time.to_glib(), &mut jitter));
            (ret, jitter)
        }
    }

    fn get_property_async(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"async\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_async(&self, async: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"async\0".as_ptr() as *const _, Value::from(&async).to_glib_none().0);
        }
    }

    fn get_property_enable_last_sample(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"enable-last-sample\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_enable_last_sample(&self, enable_last_sample: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"enable-last-sample\0".as_ptr() as *const _, Value::from(&enable_last_sample).to_glib_none().0);
        }
    }

    fn get_property_qos(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"qos\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_qos(&self, qos: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"qos\0".as_ptr() as *const _, Value::from(&qos).to_glib_none().0);
        }
    }

    fn connect_property_async_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::async\0".as_ptr() as *const _,
                transmute(notify_async_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::blocksize\0".as_ptr() as *const _,
                transmute(notify_blocksize_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_enable_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::enable-last-sample\0".as_ptr() as *const _,
                transmute(notify_enable_last_sample_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::last-sample\0".as_ptr() as *const _,
                transmute(notify_last_sample_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_max_bitrate_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::max-bitrate\0".as_ptr() as *const _,
                transmute(notify_max_bitrate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_max_lateness_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::max-lateness\0".as_ptr() as *const _,
                transmute(notify_max_lateness_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::qos\0".as_ptr() as *const _,
                transmute(notify_qos_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_render_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::render-delay\0".as_ptr() as *const _,
                transmute(notify_render_delay_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_sync_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::sync\0".as_ptr() as *const _,
                transmute(notify_sync_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_throttle_time_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::throttle-time\0".as_ptr() as *const _,
                transmute(notify_throttle_time_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ts_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::ts-offset\0".as_ptr() as *const _,
                transmute(notify_ts_offset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_async_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_blocksize_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_enable_last_sample_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_last_sample_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_max_bitrate_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_max_lateness_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_qos_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_render_delay_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_sync_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_throttle_time_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ts_offset_trampoline<P>(this: *mut ffi::GstBaseSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSink> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSink::from_glib_borrow(this).downcast_unchecked())
}
