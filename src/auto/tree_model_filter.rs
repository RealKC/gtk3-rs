// This file was generated by gir (26020e8) from gir-files (11e0e6d)
// DO NOT EDIT

use TreeIter;
use TreeModel;
use TreePath;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TreeModelFilter(Object<ffi::GtkTreeModelFilter>): TreeModel;

    match fn {
        get_type => || ffi::gtk_tree_model_filter_get_type(),
    }
}

impl TreeModelFilter {
    pub fn clear_cache(&self) {
        unsafe {
            ffi::gtk_tree_model_filter_clear_cache(self.to_glib_none().0);
        }
    }

    pub fn convert_child_iter_to_iter(&self, child_iter: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut filter_iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_filter_convert_child_iter_to_iter(self.to_glib_none().0, filter_iter.to_glib_none_mut().0, mut_override(child_iter.to_glib_none().0)));
            if ret { Some(filter_iter) } else { None }
        }
    }

    pub fn convert_child_path_to_path(&self, child_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_filter_convert_child_path_to_path(self.to_glib_none().0, mut_override(child_path.to_glib_none().0)))
        }
    }

    pub fn convert_iter_to_child_iter(&self, filter_iter: &TreeIter) -> TreeIter {
        unsafe {
            let mut child_iter = TreeIter::uninitialized();
            ffi::gtk_tree_model_filter_convert_iter_to_child_iter(self.to_glib_none().0, child_iter.to_glib_none_mut().0, mut_override(filter_iter.to_glib_none().0));
            child_iter
        }
    }

    pub fn convert_path_to_child_path(&self, filter_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_filter_convert_path_to_child_path(self.to_glib_none().0, mut_override(filter_path.to_glib_none().0)))
        }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_tree_model_filter_get_model(self.to_glib_none().0))
        }
    }

    pub fn refilter(&self) {
        unsafe {
            ffi::gtk_tree_model_filter_refilter(self.to_glib_none().0);
        }
    }

    //pub fn set_modify_func(&self, n_columns: i32, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }, func: /*Unknown conversion*//*Unimplemented*/TreeModelFilterModifyFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_model_filter_set_modify_func() }
    //}

    pub fn set_visible_column(&self, column: i32) {
        unsafe {
            ffi::gtk_tree_model_filter_set_visible_column(self.to_glib_none().0, column);
        }
    }

    //pub fn set_visible_func(&self, func: /*Unknown conversion*//*Unimplemented*/TreeModelFilterVisibleFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_model_filter_set_visible_func() }
    //}
}
