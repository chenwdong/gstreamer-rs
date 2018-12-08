use ffi;
use glib;
use glib::object::IsA;
use glib::source::SourceId;
use glib::translate::*;
use RTSPServer;

pub trait RTSPServerExtManual: 'static {
    fn attach<'a, P: Into<Option<&'a glib::MainContext>>>(&self, context: P) -> SourceId;
}

impl<O: IsA<RTSPServer>> RTSPServerExtManual for O {
    fn attach<'a, P: Into<Option<&'a glib::MainContext>>>(&self, context: P) -> SourceId {
        let context = context.into();
        let context = context.to_glib_none();
        unsafe {
            from_glib(ffi::gst_rtsp_server_attach(
                self.to_glib_none().0,
                context.0,
            ))
        }
    }
}
