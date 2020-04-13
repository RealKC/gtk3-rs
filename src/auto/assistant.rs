// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_pixbuf;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use Application;
use AssistantPageType;
use Bin;
use Buildable;
use Container;
use ResizeMode;
use Widget;
use Window;
use WindowPosition;
use WindowType;

glib_wrapper! {
    pub struct Assistant(Object<gtk_sys::GtkAssistant, gtk_sys::GtkAssistantClass, AssistantClass>) @extends Window, Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_assistant_get_type(),
    }
}

impl Assistant {
    pub fn new() -> Assistant {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_assistant_new()).unsafe_cast() }
    }
}

impl Default for Assistant {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct AssistantBuilder {
    use_header_bar: Option<i32>,
    accept_focus: Option<bool>,
    application: Option<Application>,
    attached_to: Option<Widget>,
    decorated: Option<bool>,
    default_height: Option<i32>,
    default_width: Option<i32>,
    deletable: Option<bool>,
    destroy_with_parent: Option<bool>,
    focus_on_map: Option<bool>,
    focus_visible: Option<bool>,
    gravity: Option<gdk::Gravity>,
    hide_titlebar_when_maximized: Option<bool>,
    icon: Option<gdk_pixbuf::Pixbuf>,
    icon_name: Option<String>,
    mnemonics_visible: Option<bool>,
    modal: Option<bool>,
    resizable: Option<bool>,
    role: Option<String>,
    screen: Option<gdk::Screen>,
    skip_pager_hint: Option<bool>,
    skip_taskbar_hint: Option<bool>,
    startup_id: Option<String>,
    title: Option<String>,
    transient_for: Option<Window>,
    type_: Option<WindowType>,
    type_hint: Option<gdk::WindowTypeHint>,
    urgency_hint: Option<bool>,
    window_position: Option<WindowPosition>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl AssistantBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Assistant {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref use_header_bar) = self.use_header_bar {
            properties.push(("use-header-bar", use_header_bar));
        }
        if let Some(ref accept_focus) = self.accept_focus {
            properties.push(("accept-focus", accept_focus));
        }
        if let Some(ref application) = self.application {
            properties.push(("application", application));
        }
        if let Some(ref attached_to) = self.attached_to {
            properties.push(("attached-to", attached_to));
        }
        if let Some(ref decorated) = self.decorated {
            properties.push(("decorated", decorated));
        }
        if let Some(ref default_height) = self.default_height {
            properties.push(("default-height", default_height));
        }
        if let Some(ref default_width) = self.default_width {
            properties.push(("default-width", default_width));
        }
        if let Some(ref deletable) = self.deletable {
            properties.push(("deletable", deletable));
        }
        if let Some(ref destroy_with_parent) = self.destroy_with_parent {
            properties.push(("destroy-with-parent", destroy_with_parent));
        }
        if let Some(ref focus_on_map) = self.focus_on_map {
            properties.push(("focus-on-map", focus_on_map));
        }
        if let Some(ref focus_visible) = self.focus_visible {
            properties.push(("focus-visible", focus_visible));
        }
        if let Some(ref gravity) = self.gravity {
            properties.push(("gravity", gravity));
        }
        if let Some(ref hide_titlebar_when_maximized) = self.hide_titlebar_when_maximized {
            properties.push(("hide-titlebar-when-maximized", hide_titlebar_when_maximized));
        }
        if let Some(ref icon) = self.icon {
            properties.push(("icon", icon));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref mnemonics_visible) = self.mnemonics_visible {
            properties.push(("mnemonics-visible", mnemonics_visible));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref role) = self.role {
            properties.push(("role", role));
        }
        if let Some(ref screen) = self.screen {
            properties.push(("screen", screen));
        }
        if let Some(ref skip_pager_hint) = self.skip_pager_hint {
            properties.push(("skip-pager-hint", skip_pager_hint));
        }
        if let Some(ref skip_taskbar_hint) = self.skip_taskbar_hint {
            properties.push(("skip-taskbar-hint", skip_taskbar_hint));
        }
        if let Some(ref startup_id) = self.startup_id {
            properties.push(("startup-id", startup_id));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref transient_for) = self.transient_for {
            properties.push(("transient-for", transient_for));
        }
        if let Some(ref type_) = self.type_ {
            properties.push(("type", type_));
        }
        if let Some(ref type_hint) = self.type_hint {
            properties.push(("type-hint", type_hint));
        }
        if let Some(ref urgency_hint) = self.urgency_hint {
            properties.push(("urgency-hint", urgency_hint));
        }
        if let Some(ref window_position) = self.window_position {
            properties.push(("window-position", window_position));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(Assistant::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn use_header_bar(mut self, use_header_bar: i32) -> Self {
        self.use_header_bar = Some(use_header_bar);
        self
    }

    pub fn accept_focus(mut self, accept_focus: bool) -> Self {
        self.accept_focus = Some(accept_focus);
        self
    }

    pub fn application<P: IsA<Application>>(mut self, application: &P) -> Self {
        self.application = Some(application.clone().upcast());
        self
    }

    pub fn attached_to<P: IsA<Widget>>(mut self, attached_to: &P) -> Self {
        self.attached_to = Some(attached_to.clone().upcast());
        self
    }

    pub fn decorated(mut self, decorated: bool) -> Self {
        self.decorated = Some(decorated);
        self
    }

    pub fn default_height(mut self, default_height: i32) -> Self {
        self.default_height = Some(default_height);
        self
    }

    pub fn default_width(mut self, default_width: i32) -> Self {
        self.default_width = Some(default_width);
        self
    }

    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = Some(deletable);
        self
    }

    pub fn destroy_with_parent(mut self, destroy_with_parent: bool) -> Self {
        self.destroy_with_parent = Some(destroy_with_parent);
        self
    }

    pub fn focus_on_map(mut self, focus_on_map: bool) -> Self {
        self.focus_on_map = Some(focus_on_map);
        self
    }

    pub fn focus_visible(mut self, focus_visible: bool) -> Self {
        self.focus_visible = Some(focus_visible);
        self
    }

    pub fn gravity(mut self, gravity: gdk::Gravity) -> Self {
        self.gravity = Some(gravity);
        self
    }

    pub fn hide_titlebar_when_maximized(mut self, hide_titlebar_when_maximized: bool) -> Self {
        self.hide_titlebar_when_maximized = Some(hide_titlebar_when_maximized);
        self
    }

    pub fn icon(mut self, icon: &gdk_pixbuf::Pixbuf) -> Self {
        self.icon = Some(icon.clone());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn mnemonics_visible(mut self, mnemonics_visible: bool) -> Self {
        self.mnemonics_visible = Some(mnemonics_visible);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn role(mut self, role: &str) -> Self {
        self.role = Some(role.to_string());
        self
    }

    pub fn screen(mut self, screen: &gdk::Screen) -> Self {
        self.screen = Some(screen.clone());
        self
    }

    pub fn skip_pager_hint(mut self, skip_pager_hint: bool) -> Self {
        self.skip_pager_hint = Some(skip_pager_hint);
        self
    }

    pub fn skip_taskbar_hint(mut self, skip_taskbar_hint: bool) -> Self {
        self.skip_taskbar_hint = Some(skip_taskbar_hint);
        self
    }

    pub fn startup_id(mut self, startup_id: &str) -> Self {
        self.startup_id = Some(startup_id.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn transient_for<P: IsA<Window>>(mut self, transient_for: &P) -> Self {
        self.transient_for = Some(transient_for.clone().upcast());
        self
    }

    pub fn type_(mut self, type_: WindowType) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn type_hint(mut self, type_hint: gdk::WindowTypeHint) -> Self {
        self.type_hint = Some(type_hint);
        self
    }

    pub fn urgency_hint(mut self, urgency_hint: bool) -> Self {
        self.urgency_hint = Some(urgency_hint);
        self
    }

    pub fn window_position(mut self, window_position: WindowPosition) -> Self {
        self.window_position = Some(window_position);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_ASSISTANT: Option<&Assistant> = None;

pub trait AssistantExt: 'static {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P);

    fn append_page<P: IsA<Widget>>(&self, page: &P) -> i32;

    fn commit(&self);

    fn get_current_page(&self) -> i32;

    fn get_n_pages(&self) -> i32;

    fn get_nth_page(&self, page_num: i32) -> Option<Widget>;

    fn get_page_complete<P: IsA<Widget>>(&self, page: &P) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_page_has_padding<P: IsA<Widget>>(&self, page: &P) -> bool;

    fn get_page_title<P: IsA<Widget>>(&self, page: &P) -> Option<GString>;

    fn get_page_type<P: IsA<Widget>>(&self, page: &P) -> AssistantPageType;

    fn insert_page<P: IsA<Widget>>(&self, page: &P, position: i32) -> i32;

    fn next_page(&self);

    fn prepend_page<P: IsA<Widget>>(&self, page: &P) -> i32;

    fn previous_page(&self);

    fn remove_action_widget<P: IsA<Widget>>(&self, child: &P);

    fn remove_page(&self, page_num: i32);

    fn set_current_page(&self, page_num: i32);

    fn set_forward_page_func(&self, page_func: Option<Box_<dyn Fn(i32) -> i32 + 'static>>);

    fn set_page_complete<P: IsA<Widget>>(&self, page: &P, complete: bool);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_page_has_padding<P: IsA<Widget>>(&self, page: &P, has_padding: bool);

    fn set_page_title<P: IsA<Widget>>(&self, page: &P, title: &str);

    fn set_page_type<P: IsA<Widget>>(&self, page: &P, type_: AssistantPageType);

    fn update_buttons_state(&self);

    fn get_property_use_header_bar(&self) -> i32;

    fn get_child_complete<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_complete<T: IsA<Widget>>(&self, item: &T, complete: bool);

    fn get_child_has_padding<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_has_padding<T: IsA<Widget>>(&self, item: &T, has_padding: bool);

    fn get_child_page_type<T: IsA<Widget>>(&self, item: &T) -> AssistantPageType;

    fn set_child_page_type<T: IsA<Widget>>(&self, item: &T, page_type: AssistantPageType);

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<GString>;

    fn set_child_title<T: IsA<Widget>>(&self, item: &T, title: Option<&str>);

    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_escape<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_escape(&self);

    fn connect_prepare<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Assistant>> AssistantExt for O {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_assistant_add_action_widget(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn append_page<P: IsA<Widget>>(&self, page: &P) -> i32 {
        unsafe {
            gtk_sys::gtk_assistant_append_page(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            )
        }
    }

    fn commit(&self) {
        unsafe {
            gtk_sys::gtk_assistant_commit(self.as_ref().to_glib_none().0);
        }
    }

    fn get_current_page(&self) -> i32 {
        unsafe { gtk_sys::gtk_assistant_get_current_page(self.as_ref().to_glib_none().0) }
    }

    fn get_n_pages(&self) -> i32 {
        unsafe { gtk_sys::gtk_assistant_get_n_pages(self.as_ref().to_glib_none().0) }
    }

    fn get_nth_page(&self, page_num: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_assistant_get_nth_page(
                self.as_ref().to_glib_none().0,
                page_num,
            ))
        }
    }

    fn get_page_complete<P: IsA<Widget>>(&self, page: &P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_assistant_get_page_complete(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_page_has_padding<P: IsA<Widget>>(&self, page: &P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_assistant_get_page_has_padding(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_page_title<P: IsA<Widget>>(&self, page: &P) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_assistant_get_page_title(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_page_type<P: IsA<Widget>>(&self, page: &P) -> AssistantPageType {
        unsafe {
            from_glib(gtk_sys::gtk_assistant_get_page_type(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert_page<P: IsA<Widget>>(&self, page: &P, position: i32) -> i32 {
        unsafe {
            gtk_sys::gtk_assistant_insert_page(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                position,
            )
        }
    }

    fn next_page(&self) {
        unsafe {
            gtk_sys::gtk_assistant_next_page(self.as_ref().to_glib_none().0);
        }
    }

    fn prepend_page<P: IsA<Widget>>(&self, page: &P) -> i32 {
        unsafe {
            gtk_sys::gtk_assistant_prepend_page(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            )
        }
    }

    fn previous_page(&self) {
        unsafe {
            gtk_sys::gtk_assistant_previous_page(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_action_widget<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_assistant_remove_action_widget(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove_page(&self, page_num: i32) {
        unsafe {
            gtk_sys::gtk_assistant_remove_page(self.as_ref().to_glib_none().0, page_num);
        }
    }

    fn set_current_page(&self, page_num: i32) {
        unsafe {
            gtk_sys::gtk_assistant_set_current_page(self.as_ref().to_glib_none().0, page_num);
        }
    }

    fn set_forward_page_func(&self, page_func: Option<Box_<dyn Fn(i32) -> i32 + 'static>>) {
        let page_func_data: Box_<Option<Box_<dyn Fn(i32) -> i32 + 'static>>> = Box_::new(page_func);
        unsafe extern "C" fn page_func_func(
            current_page: libc::c_int,
            data: glib_sys::gpointer,
        ) -> libc::c_int {
            let callback: &Option<Box_<dyn Fn(i32) -> i32 + 'static>> = &*(data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(current_page)
            } else {
                panic!("cannot get closure...")
            };
            res
        }
        let page_func = if page_func_data.is_some() {
            Some(page_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib_sys::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(i32) -> i32 + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(i32) -> i32 + 'static>>> = page_func_data;
        unsafe {
            gtk_sys::gtk_assistant_set_forward_page_func(
                self.as_ref().to_glib_none().0,
                page_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_page_complete<P: IsA<Widget>>(&self, page: &P, complete: bool) {
        unsafe {
            gtk_sys::gtk_assistant_set_page_complete(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                complete.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_page_has_padding<P: IsA<Widget>>(&self, page: &P, has_padding: bool) {
        unsafe {
            gtk_sys::gtk_assistant_set_page_has_padding(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                has_padding.to_glib(),
            );
        }
    }

    fn set_page_title<P: IsA<Widget>>(&self, page: &P, title: &str) {
        unsafe {
            gtk_sys::gtk_assistant_set_page_title(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    fn set_page_type<P: IsA<Widget>>(&self, page: &P, type_: AssistantPageType) {
        unsafe {
            gtk_sys::gtk_assistant_set_page_type(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                type_.to_glib(),
            );
        }
    }

    fn update_buttons_state(&self) {
        unsafe {
            gtk_sys::gtk_assistant_update_buttons_state(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_use_header_bar(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"use-header-bar\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `use-header-bar` getter")
                .unwrap()
        }
    }

    fn get_child_complete<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"complete\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `complete` getter")
                .unwrap()
        }
    }

    fn set_child_complete<T: IsA<Widget>>(&self, item: &T, complete: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"complete\0".as_ptr() as *const _,
                Value::from(&complete).to_glib_none().0,
            );
        }
    }

    fn get_child_has_padding<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"has-padding\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `has-padding` getter")
                .unwrap()
        }
    }

    fn set_child_has_padding<T: IsA<Widget>>(&self, item: &T, has_padding: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"has-padding\0".as_ptr() as *const _,
                Value::from(&has_padding).to_glib_none().0,
            );
        }
    }

    fn get_child_page_type<T: IsA<Widget>>(&self, item: &T) -> AssistantPageType {
        unsafe {
            let mut value = Value::from_type(<AssistantPageType as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"page-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `page-type` getter")
                .unwrap()
        }
    }

    fn set_child_page_type<T: IsA<Widget>>(&self, item: &T, page_type: AssistantPageType) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"page-type\0".as_ptr() as *const _,
                Value::from(&page_type).to_glib_none().0,
            );
        }
    }

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"title\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `title` getter")
        }
    }

    fn set_child_title<T: IsA<Widget>>(&self, item: &T, title: Option<&str>) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"title\0".as_ptr() as *const _,
                Value::from(title).to_glib_none().0,
            );
        }
    }

    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn apply_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAssistant,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Assistant>,
        {
            let f: &F = &*(f as *const F);
            f(&Assistant::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"apply\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    apply_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAssistant,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Assistant>,
        {
            let f: &F = &*(f as *const F);
            f(&Assistant::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAssistant,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Assistant>,
        {
            let f: &F = &*(f as *const F);
            f(&Assistant::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    close_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_escape<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn escape_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAssistant,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Assistant>,
        {
            let f: &F = &*(f as *const F);
            f(&Assistant::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"escape\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    escape_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_escape(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("escape", &[])
                .unwrap()
        };
    }

    fn connect_prepare<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prepare_trampoline<P, F: Fn(&P, &Widget) + 'static>(
            this: *mut gtk_sys::GtkAssistant,
            page: *mut gtk_sys::GtkWidget,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Assistant>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Assistant::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(page),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prepare\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prepare_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Assistant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assistant")
    }
}
