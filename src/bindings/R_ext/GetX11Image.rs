/* automatically generated by rust-bindgen 0.66.1 */

pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
extern "C" {
    #[doc = "used by package tkrplot"]
    pub fn R_GetX11Image(
        d: ::std::os::raw::c_int,
        pximage: *mut ::std::os::raw::c_void,
        pwidth: *mut ::std::os::raw::c_int,
        pheight: *mut ::std::os::raw::c_int,
    ) -> Rboolean;
}