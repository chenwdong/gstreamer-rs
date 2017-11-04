// This file was generated by gir (12a28ac+) from gir-files (???)
// DO NOT EDIT

use ChildProxy;
use Element;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use ElementFlags;
use Object;
use Pad;
use PadDirection;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
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
    pub struct Bin(Object<ffi::GstBin>): Element, Object, ChildProxy;

    match fn {
        get_type => || ffi::gst_bin_get_type(),
    }
}

impl Bin {
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P) -> Bin {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            Element::from_glib_none(ffi::gst_bin_new(name.0)).downcast_unchecked()
        }
    }
}

unsafe impl Send for Bin {}
unsafe impl Sync for Bin {}

pub trait BinExt {
    fn add<P: IsA<Element>>(&self, element: &P) -> Result<(), glib::error::BoolError>;

    //fn add_many<P: IsA<Element>>(&self, element_1: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn find_unlinked_pad(&self, direction: PadDirection) -> Option<Pad>;

    fn get_by_interface(&self, iface: glib::types::Type) -> Option<Element>;

    fn get_by_name(&self, name: &str) -> Option<Element>;

    fn get_by_name_recurse_up(&self, name: &str) -> Option<Element>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_suppressed_flags(&self) -> ElementFlags;

    //fn iterate_all_by_interface(&self, iface: glib::types::Type) -> /*Ignored*/Option<Iterator>;

    //fn iterate_elements(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_recurse(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_sinks(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_sorted(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_sources(&self) -> /*Ignored*/Option<Iterator>;

    fn recalculate_latency(&self) -> Result<(), glib::error::BoolError>;

    fn remove<P: IsA<Element>>(&self, element: &P) -> Result<(), glib::error::BoolError>;

    //fn remove_many<P: IsA<Element>>(&self, element_1: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_suppressed_flags(&self, flags: ElementFlags);

    fn sync_children_states(&self) -> Result<(), glib::error::BoolError>;

    fn get_property_async_handling(&self) -> bool;

    fn set_property_async_handling(&self, async_handling: bool);

    fn get_property_message_forward(&self) -> bool;

    fn set_property_message_forward(&self, message_forward: bool);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_deep_element_added<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_deep_element_removed<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_do_latency<F: Fn(&Self) -> bool + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_element_added<F: Fn(&Self, &Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_element_removed<F: Fn(&Self, &Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_async_handling_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_message_forward_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Bin> + IsA<glib::object::Object>> BinExt for O {
    fn add<P: IsA<Element>>(&self, element: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_bin_add(self.to_glib_none().0, element.to_glib_none().0), "Failed to add element")
        }
    }

    //fn add_many<P: IsA<Element>>(&self, element_1: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_bin_add_many() }
    //}

    fn find_unlinked_pad(&self, direction: PadDirection) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_bin_find_unlinked_pad(self.to_glib_none().0, direction.to_glib()))
        }
    }

    fn get_by_interface(&self, iface: glib::types::Type) -> Option<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_get_by_interface(self.to_glib_none().0, iface.to_glib()))
        }
    }

    fn get_by_name(&self, name: &str) -> Option<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_get_by_name(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_by_name_recurse_up(&self, name: &str) -> Option<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_get_by_name_recurse_up(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_suppressed_flags(&self) -> ElementFlags {
        unsafe {
            from_glib(ffi::gst_bin_get_suppressed_flags(self.to_glib_none().0))
        }
    }

    //fn iterate_all_by_interface(&self, iface: glib::types::Type) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_bin_iterate_all_by_interface() }
    //}

    //fn iterate_elements(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_bin_iterate_elements() }
    //}

    //fn iterate_recurse(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_bin_iterate_recurse() }
    //}

    //fn iterate_sinks(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_bin_iterate_sinks() }
    //}

    //fn iterate_sorted(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_bin_iterate_sorted() }
    //}

    //fn iterate_sources(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_bin_iterate_sources() }
    //}

    fn recalculate_latency(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_bin_recalculate_latency(self.to_glib_none().0), "Failed to recalculate latency")
        }
    }

    fn remove<P: IsA<Element>>(&self, element: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_bin_remove(self.to_glib_none().0, element.to_glib_none().0), "Failed to remove element")
        }
    }

    //fn remove_many<P: IsA<Element>>(&self, element_1: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_bin_remove_many() }
    //}

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_suppressed_flags(&self, flags: ElementFlags) {
        unsafe {
            ffi::gst_bin_set_suppressed_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn sync_children_states(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_bin_sync_children_states(self.to_glib_none().0), "Failed to sync children states")
        }
    }

    fn get_property_async_handling(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "async-handling".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_async_handling(&self, async_handling: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "async-handling".to_glib_none().0, Value::from(&async_handling).to_glib_none().0);
        }
    }

    fn get_property_message_forward(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "message-forward".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_message_forward(&self, message_forward: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "message-forward".to_glib_none().0, Value::from(&message_forward).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_deep_element_added<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Bin, &Element) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "deep-element-added",
                transmute(deep_element_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_deep_element_removed<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Bin, &Element) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "deep-element-removed",
                transmute(deep_element_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_do_latency<F: Fn(&Self) -> bool + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "do-latency",
                transmute(do_latency_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_element_added<F: Fn(&Self, &Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Element) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "element-added",
                transmute(element_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_element_removed<F: Fn(&Self, &Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Element) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "element-removed",
                transmute(element_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_async_handling_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::async-handling",
                transmute(notify_async_handling_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_message_forward_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::message-forward",
                transmute(notify_message_forward_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn deep_element_added_trampoline<P>(this: *mut ffi::GstBin, sub_bin: *mut ffi::GstBin, element: *mut ffi::GstElement, f: glib_ffi::gpointer)
where P: IsA<Bin> {
    callback_guard!();
    let f: &&(Fn(&P, &Bin, &Element) + Send + Sync + 'static) = transmute(f);
    f(&Bin::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(sub_bin), &from_glib_borrow(element))
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn deep_element_removed_trampoline<P>(this: *mut ffi::GstBin, sub_bin: *mut ffi::GstBin, element: *mut ffi::GstElement, f: glib_ffi::gpointer)
where P: IsA<Bin> {
    callback_guard!();
    let f: &&(Fn(&P, &Bin, &Element) + Send + Sync + 'static) = transmute(f);
    f(&Bin::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(sub_bin), &from_glib_borrow(element))
}

unsafe extern "C" fn do_latency_trampoline<P>(this: *mut ffi::GstBin, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Bin> {
    callback_guard!();
    let f: &&(Fn(&P) -> bool + Send + Sync + 'static) = transmute(f);
    f(&Bin::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn element_added_trampoline<P>(this: *mut ffi::GstBin, element: *mut ffi::GstElement, f: glib_ffi::gpointer)
where P: IsA<Bin> {
    callback_guard!();
    let f: &&(Fn(&P, &Element) + Send + Sync + 'static) = transmute(f);
    f(&Bin::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(element))
}

unsafe extern "C" fn element_removed_trampoline<P>(this: *mut ffi::GstBin, element: *mut ffi::GstElement, f: glib_ffi::gpointer)
where P: IsA<Bin> {
    callback_guard!();
    let f: &&(Fn(&P, &Element) + Send + Sync + 'static) = transmute(f);
    f(&Bin::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(element))
}

unsafe extern "C" fn notify_async_handling_trampoline<P>(this: *mut ffi::GstBin, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Bin> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Bin::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_message_forward_trampoline<P>(this: *mut ffi::GstBin, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Bin> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Bin::from_glib_borrow(this).downcast_unchecked())
}
