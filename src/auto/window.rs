// This file was generated by gir (26020e8) from gir-files (11e0e6d)
// DO NOT EDIT

use AccelGroup;
use Application;
use Bin;
use Container;
use Error;
use Rectangle;
use Widget;
use WindowPosition;
use WindowType;
use ffi;
use gdk;
use gdk_pixbuf;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Window(Object<ffi::GtkWindow>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_window_get_type(),
    }
}

impl Window {
    pub fn new(type_: WindowType) -> Window {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_window_new(type_.to_glib())).downcast_unchecked()
        }
    }

    pub fn get_default_icon_list() -> Vec<gdk_pixbuf::Pixbuf> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_get_default_icon_list())
        }
    }

    pub fn get_default_icon_name() -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_window_get_default_icon_name())
        }
    }

    pub fn list_toplevels() -> Vec<Widget> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_list_toplevels())
        }
    }

    pub fn set_auto_startup_notification(setting: bool) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_auto_startup_notification(setting.to_glib());
        }
    }

    pub fn set_default_icon(icon: &gdk_pixbuf::Pixbuf) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_default_icon(icon.to_glib_none().0);
        }
    }

    pub fn set_default_icon_from_file<T: AsRef<std::path::Path>>(filename: T) -> Result<(), Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_window_set_default_icon_from_file(filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_default_icon_list(list: &[gdk_pixbuf::Pixbuf]) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_default_icon_list(list.to_glib_container().0);
        }
    }

    pub fn set_default_icon_name(name: &str) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_default_icon_name(name.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_interactive_debugging(enable: bool) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_window_set_interactive_debugging(enable.to_glib());
        }
    }
}

pub trait WindowExt {
    fn activate_default(&self) -> bool;

    fn activate_focus(&self) -> bool;

    //fn activate_key(&self, event: /*Ignored*/&mut gdk::EventKey) -> bool;

    fn add_accel_group(&self, accel_group: &AccelGroup);

    fn add_mnemonic<T: IsA<Widget>>(&self, keyval: u32, target: &T);

    fn begin_move_drag(&self, button: i32, root_x: i32, root_y: i32, timestamp: u32);

    fn begin_resize_drag(&self, edge: gdk::WindowEdge, button: i32, root_x: i32, root_y: i32, timestamp: u32);

    #[cfg(feature = "v3_10")]
    fn close(&self);

    fn deiconify(&self);

    fn fullscreen(&self);

    fn get_accept_focus(&self) -> bool;

    fn get_application(&self) -> Option<Application>;

    fn get_attached_to(&self) -> Option<Widget>;

    fn get_decorated(&self) -> bool;

    fn get_default_size(&self) -> (i32, i32);

    fn get_default_widget(&self) -> Option<Widget>;

    fn get_deletable(&self) -> bool;

    fn get_destroy_with_parent(&self) -> bool;

    fn get_focus(&self) -> Option<Widget>;

    fn get_focus_on_map(&self) -> bool;

    fn get_focus_visible(&self) -> bool;

    fn get_gravity(&self) -> gdk::Gravity;

    //fn get_group(&self) -> /*Ignored*/Option<WindowGroup>;

    fn get_has_resize_grip(&self) -> bool;

    fn get_hide_titlebar_when_maximized(&self) -> bool;

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_icon_list(&self) -> Vec<gdk_pixbuf::Pixbuf>;

    fn get_icon_name(&self) -> Option<String>;

    fn get_mnemonic_modifier(&self) -> gdk::ModifierType;

    fn get_mnemonics_visible(&self) -> bool;

    fn get_modal(&self) -> bool;

    fn get_opacity(&self) -> f64;

    fn get_position(&self) -> (i32, i32);

    fn get_resizable(&self) -> bool;

    fn get_resize_grip_area(&self) -> Option<Rectangle>;

    fn get_role(&self) -> Option<String>;

    fn get_screen(&self) -> Option<gdk::Screen>;

    fn get_size(&self) -> (i32, i32);

    fn get_skip_pager_hint(&self) -> bool;

    fn get_skip_taskbar_hint(&self) -> bool;

    fn get_title(&self) -> Option<String>;

    #[cfg(feature = "v3_16")]
    fn get_titlebar(&self) -> Option<Widget>;

    fn get_transient_for(&self) -> Option<Window>;

    fn get_type_hint(&self) -> gdk::WindowTypeHint;

    fn get_urgency_hint(&self) -> bool;

    fn get_window_type(&self) -> WindowType;

    fn has_group(&self) -> bool;

    fn has_toplevel_focus(&self) -> bool;

    fn iconify(&self);

    fn is_active(&self) -> bool;

    #[cfg(feature = "v3_12")]
    fn is_maximized(&self) -> bool;

    fn maximize(&self);

    fn mnemonic_activate(&self, keyval: u32, modifier: gdk::ModifierType) -> bool;

    fn move_(&self, x: i32, y: i32);

    fn parse_geometry(&self, geometry: &str) -> bool;

    fn present(&self);

    fn present_with_time(&self, timestamp: u32);

    //fn propagate_key_event(&self, event: /*Ignored*/&mut gdk::EventKey) -> bool;

    fn remove_accel_group(&self, accel_group: &AccelGroup);

    fn remove_mnemonic<T: IsA<Widget>>(&self, keyval: u32, target: &T);

    fn reshow_with_initial_size(&self);

    fn resize(&self, width: i32, height: i32);

    fn resize_grip_is_visible(&self) -> bool;

    fn resize_to_geometry(&self, width: i32, height: i32);

    fn set_accept_focus(&self, setting: bool);

    fn set_application(&self, application: Option<&Application>);

    fn set_attached_to<T: IsA<Widget>>(&self, attach_widget: Option<&T>);

    fn set_decorated(&self, setting: bool);

    fn set_default<T: IsA<Widget>>(&self, default_widget: Option<&T>);

    fn set_default_geometry(&self, width: i32, height: i32);

    fn set_default_size(&self, width: i32, height: i32);

    fn set_deletable(&self, setting: bool);

    fn set_destroy_with_parent(&self, setting: bool);

    fn set_focus<T: IsA<Widget>>(&self, focus: Option<&T>);

    fn set_focus_on_map(&self, setting: bool);

    fn set_focus_visible(&self, setting: bool);

    //fn set_geometry_hints<T: IsA<Widget>>(&self, geometry_widget: Option<&T>, geometry: /*Ignored*/Option<&mut gdk::Geometry>, geom_mask: /*Ignored*/gdk::WindowHints);

    fn set_gravity(&self, gravity: gdk::Gravity);

    fn set_has_resize_grip(&self, value: bool);

    fn set_has_user_ref_count(&self, setting: bool);

    fn set_hide_titlebar_when_maximized(&self, setting: bool);

    fn set_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>);

    fn set_icon_from_file<T: AsRef<std::path::Path>>(&self, filename: T) -> Result<(), Error>;

    fn set_icon_list(&self, list: &[gdk_pixbuf::Pixbuf]);

    fn set_icon_name(&self, name: Option<&str>);

    fn set_keep_above(&self, setting: bool);

    fn set_keep_below(&self, setting: bool);

    fn set_mnemonic_modifier(&self, modifier: gdk::ModifierType);

    fn set_mnemonics_visible(&self, setting: bool);

    fn set_modal(&self, modal: bool);

    fn set_opacity(&self, opacity: f64);

    fn set_position(&self, position: WindowPosition);

    fn set_resizable(&self, resizable: bool);

    fn set_role(&self, role: &str);

    fn set_screen(&self, screen: &gdk::Screen);

    fn set_skip_pager_hint(&self, setting: bool);

    fn set_skip_taskbar_hint(&self, setting: bool);

    fn set_startup_id(&self, startup_id: &str);

    fn set_title(&self, title: &str);

    #[cfg(feature = "v3_10")]
    fn set_titlebar<T: IsA<Widget>>(&self, titlebar: Option<&T>);

    fn set_transient_for<T: IsA<Window>>(&self, parent: Option<&T>);

    fn set_type_hint(&self, hint: gdk::WindowTypeHint);

    fn set_urgency_hint(&self, setting: bool);

    fn set_wmclass(&self, wmclass_name: &str, wmclass_class: &str);

    fn stick(&self);

    fn unfullscreen(&self);

    fn unmaximize(&self);

    fn unstick(&self);
}

impl<O: IsA<Window>> WindowExt for O {
    fn activate_default(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_activate_default(self.to_glib_none().0))
        }
    }

    fn activate_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_activate_focus(self.to_glib_none().0))
        }
    }

    //fn activate_key(&self, event: /*Ignored*/&mut gdk::EventKey) -> bool {
    //    unsafe { TODO: call ffi::gtk_window_activate_key() }
    //}

    fn add_accel_group(&self, accel_group: &AccelGroup) {
        unsafe {
            ffi::gtk_window_add_accel_group(self.to_glib_none().0, accel_group.to_glib_none().0);
        }
    }

    fn add_mnemonic<T: IsA<Widget>>(&self, keyval: u32, target: &T) {
        unsafe {
            ffi::gtk_window_add_mnemonic(self.to_glib_none().0, keyval, target.to_glib_none().0);
        }
    }

    fn begin_move_drag(&self, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe {
            ffi::gtk_window_begin_move_drag(self.to_glib_none().0, button, root_x, root_y, timestamp);
        }
    }

    fn begin_resize_drag(&self, edge: gdk::WindowEdge, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe {
            ffi::gtk_window_begin_resize_drag(self.to_glib_none().0, edge.to_glib(), button, root_x, root_y, timestamp);
        }
    }

    #[cfg(feature = "v3_10")]
    fn close(&self) {
        unsafe {
            ffi::gtk_window_close(self.to_glib_none().0);
        }
    }

    fn deiconify(&self) {
        unsafe {
            ffi::gtk_window_deiconify(self.to_glib_none().0);
        }
    }

    fn fullscreen(&self) {
        unsafe {
            ffi::gtk_window_fullscreen(self.to_glib_none().0);
        }
    }

    fn get_accept_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_accept_focus(self.to_glib_none().0))
        }
    }

    fn get_application(&self) -> Option<Application> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_application(self.to_glib_none().0))
        }
    }

    fn get_attached_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_attached_to(self.to_glib_none().0))
        }
    }

    fn get_decorated(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_decorated(self.to_glib_none().0))
        }
    }

    fn get_default_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_window_get_default_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn get_default_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_default_widget(self.to_glib_none().0))
        }
    }

    fn get_deletable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_deletable(self.to_glib_none().0))
        }
    }

    fn get_destroy_with_parent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_destroy_with_parent(self.to_glib_none().0))
        }
    }

    fn get_focus(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_focus(self.to_glib_none().0))
        }
    }

    fn get_focus_on_map(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_focus_on_map(self.to_glib_none().0))
        }
    }

    fn get_focus_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_focus_visible(self.to_glib_none().0))
        }
    }

    fn get_gravity(&self) -> gdk::Gravity {
        unsafe {
            from_glib(ffi::gtk_window_get_gravity(self.to_glib_none().0))
        }
    }

    //fn get_group(&self) -> /*Ignored*/Option<WindowGroup> {
    //    unsafe { TODO: call ffi::gtk_window_get_group() }
    //}

    fn get_has_resize_grip(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_has_resize_grip(self.to_glib_none().0))
        }
    }

    fn get_hide_titlebar_when_maximized(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_hide_titlebar_when_maximized(self.to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_icon(self.to_glib_none().0))
        }
    }

    fn get_icon_list(&self) -> Vec<gdk_pixbuf::Pixbuf> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_get_icon_list(self.to_glib_none().0))
        }
    }

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_mnemonic_modifier(&self) -> gdk::ModifierType {
        unsafe {
            from_glib(ffi::gtk_window_get_mnemonic_modifier(self.to_glib_none().0))
        }
    }

    fn get_mnemonics_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_mnemonics_visible(self.to_glib_none().0))
        }
    }

    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_modal(self.to_glib_none().0))
        }
    }

    fn get_opacity(&self) -> f64 {
        unsafe {
            ffi::gtk_window_get_opacity(self.to_glib_none().0)
        }
    }

    fn get_position(&self) -> (i32, i32) {
        unsafe {
            let mut root_x = mem::uninitialized();
            let mut root_y = mem::uninitialized();
            ffi::gtk_window_get_position(self.to_glib_none().0, &mut root_x, &mut root_y);
            (root_x, root_y)
        }
    }

    fn get_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_resizable(self.to_glib_none().0))
        }
    }

    fn get_resize_grip_area(&self) -> Option<Rectangle> {
        unsafe {
            let mut rect = Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_window_get_resize_grip_area(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    fn get_role(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_role(self.to_glib_none().0))
        }
    }

    fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_screen(self.to_glib_none().0))
        }
    }

    fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_window_get_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn get_skip_pager_hint(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_skip_pager_hint(self.to_glib_none().0))
        }
    }

    fn get_skip_taskbar_hint(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_skip_taskbar_hint(self.to_glib_none().0))
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_title(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_titlebar(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_titlebar(self.to_glib_none().0))
        }
    }

    fn get_transient_for(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_transient_for(self.to_glib_none().0))
        }
    }

    fn get_type_hint(&self) -> gdk::WindowTypeHint {
        unsafe {
            from_glib(ffi::gtk_window_get_type_hint(self.to_glib_none().0))
        }
    }

    fn get_urgency_hint(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_get_urgency_hint(self.to_glib_none().0))
        }
    }

    fn get_window_type(&self) -> WindowType {
        unsafe {
            from_glib(ffi::gtk_window_get_window_type(self.to_glib_none().0))
        }
    }

    fn has_group(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_has_group(self.to_glib_none().0))
        }
    }

    fn has_toplevel_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_has_toplevel_focus(self.to_glib_none().0))
        }
    }

    fn iconify(&self) {
        unsafe {
            ffi::gtk_window_iconify(self.to_glib_none().0);
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_is_active(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn is_maximized(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_is_maximized(self.to_glib_none().0))
        }
    }

    fn maximize(&self) {
        unsafe {
            ffi::gtk_window_maximize(self.to_glib_none().0);
        }
    }

    fn mnemonic_activate(&self, keyval: u32, modifier: gdk::ModifierType) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_mnemonic_activate(self.to_glib_none().0, keyval, modifier.to_glib()))
        }
    }

    fn move_(&self, x: i32, y: i32) {
        unsafe {
            ffi::gtk_window_move(self.to_glib_none().0, x, y);
        }
    }

    fn parse_geometry(&self, geometry: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_parse_geometry(self.to_glib_none().0, geometry.to_glib_none().0))
        }
    }

    fn present(&self) {
        unsafe {
            ffi::gtk_window_present(self.to_glib_none().0);
        }
    }

    fn present_with_time(&self, timestamp: u32) {
        unsafe {
            ffi::gtk_window_present_with_time(self.to_glib_none().0, timestamp);
        }
    }

    //fn propagate_key_event(&self, event: /*Ignored*/&mut gdk::EventKey) -> bool {
    //    unsafe { TODO: call ffi::gtk_window_propagate_key_event() }
    //}

    fn remove_accel_group(&self, accel_group: &AccelGroup) {
        unsafe {
            ffi::gtk_window_remove_accel_group(self.to_glib_none().0, accel_group.to_glib_none().0);
        }
    }

    fn remove_mnemonic<T: IsA<Widget>>(&self, keyval: u32, target: &T) {
        unsafe {
            ffi::gtk_window_remove_mnemonic(self.to_glib_none().0, keyval, target.to_glib_none().0);
        }
    }

    fn reshow_with_initial_size(&self) {
        unsafe {
            ffi::gtk_window_reshow_with_initial_size(self.to_glib_none().0);
        }
    }

    fn resize(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_resize(self.to_glib_none().0, width, height);
        }
    }

    fn resize_grip_is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_window_resize_grip_is_visible(self.to_glib_none().0))
        }
    }

    fn resize_to_geometry(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_resize_to_geometry(self.to_glib_none().0, width, height);
        }
    }

    fn set_accept_focus(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_accept_focus(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_application(&self, application: Option<&Application>) {
        unsafe {
            ffi::gtk_window_set_application(self.to_glib_none().0, application.to_glib_none().0);
        }
    }

    fn set_attached_to<T: IsA<Widget>>(&self, attach_widget: Option<&T>) {
        unsafe {
            ffi::gtk_window_set_attached_to(self.to_glib_none().0, attach_widget.to_glib_none().0);
        }
    }

    fn set_decorated(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_decorated(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_default<T: IsA<Widget>>(&self, default_widget: Option<&T>) {
        unsafe {
            ffi::gtk_window_set_default(self.to_glib_none().0, default_widget.to_glib_none().0);
        }
    }

    fn set_default_geometry(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_set_default_geometry(self.to_glib_none().0, width, height);
        }
    }

    fn set_default_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_set_default_size(self.to_glib_none().0, width, height);
        }
    }

    fn set_deletable(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_deletable(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_destroy_with_parent(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_destroy_with_parent(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_focus<T: IsA<Widget>>(&self, focus: Option<&T>) {
        unsafe {
            ffi::gtk_window_set_focus(self.to_glib_none().0, focus.to_glib_none().0);
        }
    }

    fn set_focus_on_map(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_focus_on_map(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_focus_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_focus_visible(self.to_glib_none().0, setting.to_glib());
        }
    }

    //fn set_geometry_hints<T: IsA<Widget>>(&self, geometry_widget: Option<&T>, geometry: /*Ignored*/Option<&mut gdk::Geometry>, geom_mask: /*Ignored*/gdk::WindowHints) {
    //    unsafe { TODO: call ffi::gtk_window_set_geometry_hints() }
    //}

    fn set_gravity(&self, gravity: gdk::Gravity) {
        unsafe {
            ffi::gtk_window_set_gravity(self.to_glib_none().0, gravity.to_glib());
        }
    }

    fn set_has_resize_grip(&self, value: bool) {
        unsafe {
            ffi::gtk_window_set_has_resize_grip(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_has_user_ref_count(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_has_user_ref_count(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_hide_titlebar_when_maximized(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_hide_titlebar_when_maximized(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_window_set_icon(self.to_glib_none().0, icon.to_glib_none().0);
        }
    }

    fn set_icon_from_file<T: AsRef<std::path::Path>>(&self, filename: T) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_window_set_icon_from_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_icon_list(&self, list: &[gdk_pixbuf::Pixbuf]) {
        unsafe {
            ffi::gtk_window_set_icon_list(self.to_glib_none().0, list.to_glib_none().0);
        }
    }

    fn set_icon_name(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_window_set_icon_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_keep_above(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_keep_above(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_keep_below(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_keep_below(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_mnemonic_modifier(&self, modifier: gdk::ModifierType) {
        unsafe {
            ffi::gtk_window_set_mnemonic_modifier(self.to_glib_none().0, modifier.to_glib());
        }
    }

    fn set_mnemonics_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_mnemonics_visible(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_window_set_modal(self.to_glib_none().0, modal.to_glib());
        }
    }

    fn set_opacity(&self, opacity: f64) {
        unsafe {
            ffi::gtk_window_set_opacity(self.to_glib_none().0, opacity);
        }
    }

    fn set_position(&self, position: WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(self.to_glib_none().0, position.to_glib());
        }
    }

    fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gtk_window_set_resizable(self.to_glib_none().0, resizable.to_glib());
        }
    }

    fn set_role(&self, role: &str) {
        unsafe {
            ffi::gtk_window_set_role(self.to_glib_none().0, role.to_glib_none().0);
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_window_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    fn set_skip_pager_hint(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_skip_pager_hint(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_skip_taskbar_hint(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_skip_taskbar_hint(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_startup_id(&self, startup_id: &str) {
        unsafe {
            ffi::gtk_window_set_startup_id(self.to_glib_none().0, startup_id.to_glib_none().0);
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_window_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_titlebar<T: IsA<Widget>>(&self, titlebar: Option<&T>) {
        unsafe {
            ffi::gtk_window_set_titlebar(self.to_glib_none().0, titlebar.to_glib_none().0);
        }
    }

    fn set_transient_for<T: IsA<Window>>(&self, parent: Option<&T>) {
        unsafe {
            ffi::gtk_window_set_transient_for(self.to_glib_none().0, parent.to_glib_none().0);
        }
    }

    fn set_type_hint(&self, hint: gdk::WindowTypeHint) {
        unsafe {
            ffi::gtk_window_set_type_hint(self.to_glib_none().0, hint.to_glib());
        }
    }

    fn set_urgency_hint(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_urgency_hint(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_wmclass(&self, wmclass_name: &str, wmclass_class: &str) {
        unsafe {
            ffi::gtk_window_set_wmclass(self.to_glib_none().0, wmclass_name.to_glib_none().0, wmclass_class.to_glib_none().0);
        }
    }

    fn stick(&self) {
        unsafe {
            ffi::gtk_window_stick(self.to_glib_none().0);
        }
    }

    fn unfullscreen(&self) {
        unsafe {
            ffi::gtk_window_unfullscreen(self.to_glib_none().0);
        }
    }

    fn unmaximize(&self) {
        unsafe {
            ffi::gtk_window_unmaximize(self.to_glib_none().0);
        }
    }

    fn unstick(&self) {
        unsafe {
            ffi::gtk_window_unstick(self.to_glib_none().0);
        }
    }
}
