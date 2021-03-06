// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;

bitflags! {
    pub struct GLAPI: u32 {
        const NONE = 0;
        const OPENGL = 1;
        const OPENGL3 = 2;
        const GLES1 = 32768;
        const GLES2 = 65536;
        const ANY = 4294967295;
    }
}

#[doc(hidden)]
impl ToGlib for GLAPI {
    type GlibType = ffi::GstGLAPI;

    fn to_glib(&self) -> ffi::GstGLAPI {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLAPI> for GLAPI {
    fn from_glib(value: ffi::GstGLAPI) -> GLAPI {
        skip_assert_initialized!();
        GLAPI::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct GLDisplayType: u32 {
        const NONE = 0;
        const X11 = 1;
        const WAYLAND = 2;
        const COCOA = 4;
        const WIN32 = 8;
        const DISPMANX = 16;
        const EGL = 32;
        const VIV_FB = 64;
        const GBM = 128;
        const ANY = 4294967295;
    }
}

#[doc(hidden)]
impl ToGlib for GLDisplayType {
    type GlibType = ffi::GstGLDisplayType;

    fn to_glib(&self) -> ffi::GstGLDisplayType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLDisplayType> for GLDisplayType {
    fn from_glib(value: ffi::GstGLDisplayType) -> GLDisplayType {
        skip_assert_initialized!();
        GLDisplayType::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct GLPlatform: u32 {
        const NONE = 0;
        const EGL = 1;
        const GLX = 2;
        const WGL = 4;
        const CGL = 8;
        const EAGL = 16;
        const ANY = 4294967295;
    }
}

#[doc(hidden)]
impl ToGlib for GLPlatform {
    type GlibType = ffi::GstGLPlatform;

    fn to_glib(&self) -> ffi::GstGLPlatform {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLPlatform> for GLPlatform {
    fn from_glib(value: ffi::GstGLPlatform) -> GLPlatform {
        skip_assert_initialized!();
        GLPlatform::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct GLSLProfile: u32 {
        const NONE = 0;
        const ES = 1;
        const CORE = 2;
        const COMPATIBILITY = 4;
        const ANY = 4294967295;
    }
}

#[doc(hidden)]
impl ToGlib for GLSLProfile {
    type GlibType = ffi::GstGLSLProfile;

    fn to_glib(&self) -> ffi::GstGLSLProfile {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLSLProfile> for GLSLProfile {
    fn from_glib(value: ffi::GstGLSLProfile) -> GLSLProfile {
        skip_assert_initialized!();
        GLSLProfile::from_bits_truncate(value)
    }
}

