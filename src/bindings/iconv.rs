/* automatically generated by rust-bindgen 0.66.1 */

pub type libiconv_t = *mut ::std::os::raw::c_void;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
extern "C" {
    pub fn libiconv_open(
        tocode: *const ::std::os::raw::c_char,
        fromcode: *const ::std::os::raw::c_char,
    ) -> libiconv_t;
}
extern "C" {
    pub fn libiconv(
        cd: libiconv_t,
        inbuf: *mut *const ::std::os::raw::c_char,
        inbytesleft: *mut usize,
        outbuf: *mut *mut ::std::os::raw::c_char,
        outbytesleft: *mut usize,
    ) -> usize;
}
extern "C" {
    pub fn libiconv_close(cd: libiconv_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libiconvlist(
        do_one: ::std::option::Option<
            unsafe extern "C" fn(
                namescount: ::std::os::raw::c_uint,
                names: *const *const ::std::os::raw::c_char,
                data: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        daXta: *mut ::std::os::raw::c_void,
    );
}
