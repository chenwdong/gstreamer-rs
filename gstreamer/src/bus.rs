// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::source::{Continue, Priority, SourceId};
use glib::translate::*;
use glib_ffi;
use glib_ffi::{gboolean, gpointer};
use std::cell::RefCell;
use std::mem::transmute;
use std::ptr;

use Bus;
use BusSyncReply;
use Message;

unsafe extern "C" fn trampoline_watch<F: FnMut(&Bus, &Message) -> Continue + 'static>(
    bus: *mut ffi::GstBus,
    msg: *mut ffi::GstMessage,
    func: gpointer,
) -> gboolean {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (&mut *func.borrow_mut())(&from_glib_borrow(bus), &Message::from_glib_borrow(msg)).to_glib()
}

unsafe extern "C" fn destroy_closure_watch<F: FnMut(&Bus, &Message) -> Continue + 'static>(
    ptr: gpointer,
) {
    Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

fn into_raw_watch<F: FnMut(&Bus, &Message) -> Continue + 'static>(func: F) -> gpointer {
    #[allow(clippy::type_complexity)]
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn trampoline_sync<
    F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
>(
    bus: *mut ffi::GstBus,
    msg: *mut ffi::GstMessage,
    func: gpointer,
) -> ffi::GstBusSyncReply {
    let f: &F = &*(func as *const F);
    let res = f(&from_glib_borrow(bus), &Message::from_glib_borrow(msg)).to_glib();

    if res == ffi::GST_BUS_DROP {
        ffi::gst_mini_object_unref(msg as *mut _);
    }

    res
}

unsafe extern "C" fn destroy_closure_sync<
    F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
>(
    ptr: gpointer,
) {
    Box::<F>::from_raw(ptr as *mut _);
}

fn into_raw_sync<F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static>(
    func: F,
) -> gpointer {
    let func: Box<F> = Box::new(func);
    Box::into_raw(func) as gpointer
}

impl Bus {
    pub fn add_signal_watch_full(&self, priority: Priority) {
        unsafe {
            ffi::gst_bus_add_signal_watch_full(self.to_glib_none().0, priority.to_glib());
        }
    }

    pub fn create_watch<'a, N: Into<Option<&'a str>>, F>(
        &self,
        name: N,
        priority: Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        skip_assert_initialized!();
        unsafe {
            let source = ffi::gst_bus_create_watch(self.to_glib_none().0);
            glib_ffi::g_source_set_callback(
                source,
                Some(transmute(trampoline_watch::<F> as usize)),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );
            glib_ffi::g_source_set_priority(source, priority.to_glib());

            let name = name.into();
            if let Some(name) = name {
                glib_ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    pub fn add_watch<F>(&self, func: F) -> Option<SourceId>
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        unsafe {
            let res = ffi::gst_bus_add_watch_full(
                self.to_glib_none().0,
                glib_ffi::G_PRIORITY_DEFAULT,
                Some(trampoline_watch::<F>),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );

            if res == 0 {
                None
            } else {
                Some(from_glib(res))
            }
        }
    }

    pub fn add_watch_local<F>(&self, func: F) -> Option<SourceId>
    where
        F: FnMut(&Bus, &Message) -> Continue + 'static,
    {
        unsafe {
            assert!(glib::MainContext::ref_thread_default().is_owner());

            let res = ffi::gst_bus_add_watch_full(
                self.to_glib_none().0,
                glib_ffi::G_PRIORITY_DEFAULT,
                Some(trampoline_watch::<F>),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );

            if res == 0 {
                None
            } else {
                Some(from_glib(res))
            }
        }
    }

    pub fn set_sync_handler<F>(&self, func: F)
    where
        F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
    {
        unsafe {
            ffi::gst_bus_set_sync_handler(
                self.to_glib_none().0,
                Some(trampoline_sync::<F>),
                into_raw_sync(func),
                Some(destroy_closure_sync::<F>),
            )
        }
    }

    pub fn unset_sync_handler(&self) {
        unsafe { ffi::gst_bus_set_sync_handler(self.to_glib_none().0, None, ptr::null_mut(), None) }
    }

    pub fn iter(&self) -> Iter {
        self.iter_timed(0.into())
    }

    pub fn iter_timed(&self, timeout: ::ClockTime) -> Iter {
        Iter { bus: self, timeout }
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    bus: &'a Bus,
    timeout: ::ClockTime,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Message;

    fn next(&mut self) -> Option<Message> {
        self.bus.timed_pop(self.timeout)
    }
}

#[cfg(any(feature = "futures", feature = "dox"))]
mod futures {
    use super::*;
    use futures_core::stream::Stream;
    use futures_core::task::{Context, Waker};
    use futures_core::{Async, Poll};
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    pub struct BusStream(Bus, Arc<Mutex<Option<Waker>>>);

    impl BusStream {
        pub fn new(bus: &Bus) -> Self {
            skip_assert_initialized!();
            let waker = Arc::new(Mutex::new(None));
            let waker_clone = Arc::clone(&waker);

            bus.set_sync_handler(move |_, _| {
                let mut waker = waker_clone.lock().unwrap();
                if let Some(waker) = waker.take() {
                    // FIXME: Force type...
                    let waker: Waker = waker;
                    waker.wake();
                }

                BusSyncReply::Pass
            });

            BusStream(bus.clone(), waker)
        }
    }

    impl Drop for BusStream {
        fn drop(&mut self) {
            self.0.unset_sync_handler();
        }
    }

    impl Stream for BusStream {
        type Item = Message;
        type Error = ();

        fn poll_next(&mut self, ctx: &mut Context) -> Poll<Option<Self::Item>, Self::Error> {
            let BusStream(ref bus, ref waker) = *self;

            let msg = bus.pop();
            if let Some(msg) = msg {
                Ok(Async::Ready(Some(msg)))
            } else {
                let mut waker = waker.lock().unwrap();
                *waker = Some(ctx.waker().clone());
                Ok(Async::Pending)
            }
        }
    }
}

#[cfg(any(feature = "futures", feature = "dox"))]
pub use bus::futures::BusStream;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_sync_handler() {
        ::init().unwrap();

        let bus = Bus::new();
        let msgs = Arc::new(Mutex::new(Vec::new()));
        let msgs_clone = msgs.clone();
        bus.set_sync_handler(move |_, msg| {
            msgs_clone.lock().unwrap().push(msg.clone());
            BusSyncReply::Pass
        });

        bus.post(&::Message::new_eos().build()).unwrap();

        let msgs = msgs.lock().unwrap();
        assert_eq!(msgs.len(), 1);
        match msgs[0].view() {
            ::MessageView::Eos(_) => (),
            _ => unreachable!(),
        }
    }
}
