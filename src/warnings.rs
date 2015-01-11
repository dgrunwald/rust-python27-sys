use libc::{c_char, c_int};
use pyport::Py_ssize_t;
use object::PyObject;

#[link(name = "python2.7")]
extern "C" {
    pub fn PyErr_WarnEx(category: *mut PyObject, msg: *const c_char,
                        stacklevel: Py_ssize_t) -> c_int;
    pub fn PyErr_WarnExplicit(arg1: *mut PyObject,
                              arg2: *const c_char,
                              arg3: *const c_char,
                              arg4: c_int,
                              arg5: *const c_char,
                              arg6: *mut PyObject) -> c_int;
}

#[inline]
pub unsafe fn PyErr_Warn(category: *mut PyObject, msg: *const c_char) -> c_int {
    PyErr_WarnEx(category, msg, 1)
}

