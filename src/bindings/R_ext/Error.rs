/* automatically generated by rust-bindgen 0.66.1 */

pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
extern "C" {
    pub fn Rf_error(arg1: *const ::std::os::raw::c_char, ...) -> !;
}
extern "C" {
    pub fn UNIMPLEMENTED(arg1: *const ::std::os::raw::c_char) -> !;
}
extern "C" {
    pub fn WrongArgCount(arg1: *const ::std::os::raw::c_char) -> !;
}
extern "C" {
    pub fn Rf_warning(arg1: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn R_ShowMessage(s: *const ::std::os::raw::c_char);
}
