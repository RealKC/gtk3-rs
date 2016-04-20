// This file was generated by gir (26020e8) from gir-files (11e0e6d)
// DO NOT EDIT

#[cfg(feature = "v3_16")]
use Error;
use Widget;
use ffi;
#[cfg(feature = "v3_16")]
use gdk;
use glib::object::Downcast;
use glib::translate::*;
#[cfg(feature = "v3_16")]
use std::mem;

glib_wrapper! {
    pub struct GLArea(Object<ffi::GtkGLArea>): Widget;

    match fn {
        get_type => || ffi::gtk_gl_area_get_type(),
    }
}

impl GLArea {
    #[cfg(feature = "v3_16")]
    pub fn new() -> GLArea {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_full(ffi::gtk_gl_area_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn attach_buffers(&self) {
        unsafe {
            ffi::gtk_gl_area_attach_buffers(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_auto_render(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_auto_render(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_context(&self) -> Option<gdk::GLContext> {
        unsafe {
            from_glib_none(ffi::gtk_gl_area_get_context(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_error(&self) -> Option<Error> {
        unsafe {
            from_glib_none(ffi::gtk_gl_area_get_error(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_has_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_alpha(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_has_depth_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_depth_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_has_stencil_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_stencil_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::uninitialized();
            let mut minor = mem::uninitialized();
            ffi::gtk_gl_area_get_required_version(self.to_glib_none().0, &mut major, &mut minor);
            (major, minor)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn make_current(&self) {
        unsafe {
            ffi::gtk_gl_area_make_current(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn queue_render(&self) {
        unsafe {
            ffi::gtk_gl_area_queue_render(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_auto_render(&self, auto_render: bool) {
        unsafe {
            ffi::gtk_gl_area_set_auto_render(self.to_glib_none().0, auto_render.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_error(&self, error: Option<&Error>) {
        unsafe {
            ffi::gtk_gl_area_set_error(self.to_glib_none().0, error.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_has_alpha(&self, has_alpha: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_alpha(self.to_glib_none().0, has_alpha.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_has_depth_buffer(&self, has_depth_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_depth_buffer(self.to_glib_none().0, has_depth_buffer.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_has_stencil_buffer(&self, has_stencil_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_stencil_buffer(self.to_glib_none().0, has_stencil_buffer.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gtk_gl_area_set_required_version(self.to_glib_none().0, major, minor);
        }
    }
}
