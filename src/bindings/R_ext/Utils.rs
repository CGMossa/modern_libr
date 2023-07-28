/* automatically generated by rust-bindgen 0.66.1 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
pub const Rboolean_FALSE: Rboolean = 0;
pub const Rboolean_TRUE: Rboolean = 1;
pub type Rboolean = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union Rcomplex {
    pub __bindgen_anon_1: Rcomplex__bindgen_ty_1,
    pub private_data_c: __BindgenComplex<f64>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rcomplex__bindgen_ty_1 {
    pub r: f64,
    pub i: f64,
}
#[test]
fn bindgen_test_layout_Rcomplex__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<Rcomplex__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Rcomplex__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(Rcomplex__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<Rcomplex__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(Rcomplex__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Rcomplex__bindgen_ty_1),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).i) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Rcomplex__bindgen_ty_1),
            "::",
            stringify!(i)
        )
    );
}
#[test]
fn bindgen_test_layout_Rcomplex() {
    const UNINIT: ::std::mem::MaybeUninit<Rcomplex> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Rcomplex>(),
        16usize,
        concat!("Size of: ", stringify!(Rcomplex))
    );
    assert_eq!(
        ::std::mem::align_of::<Rcomplex>(),
        8usize,
        concat!("Alignment of ", stringify!(Rcomplex))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).private_data_c) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Rcomplex),
            "::",
            stringify!(private_data_c)
        )
    );
}
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
extern "C" {
    pub fn R_isort(arg1: *mut ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn R_rsort(arg1: *mut f64, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn R_csort(arg1: *mut Rcomplex, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rsort_with_index(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Rf_revsort(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Rf_iPsort(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Rf_rPsort(arg1: *mut f64, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Rf_cPsort(arg1: *mut Rcomplex, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int);
}
extern "C" {
    pub fn R_qsort(v: *mut f64, i: usize, j: usize);
}
extern "C" {
    pub fn R_qsort_I(
        v: *mut f64,
        II: *mut ::std::os::raw::c_int,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn R_qsort_int(iv: *mut ::std::os::raw::c_int, i: usize, j: usize);
}
extern "C" {
    pub fn R_qsort_int_I(
        iv: *mut ::std::os::raw::c_int,
        II: *mut ::std::os::raw::c_int,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn R_ExpandFileName(arg1: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Rf_setIVector(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Rf_setRVector(arg1: *mut f64, arg2: ::std::os::raw::c_int, arg3: f64);
}
extern "C" {
    pub fn Rf_StringFalse(arg1: *const ::std::os::raw::c_char) -> Rboolean;
}
extern "C" {
    pub fn Rf_StringTrue(arg1: *const ::std::os::raw::c_char) -> Rboolean;
}
extern "C" {
    pub fn Rf_isBlankString(arg1: *const ::std::os::raw::c_char) -> Rboolean;
}
extern "C" {
    pub fn R_atof(str_: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn R_strtod(c: *const ::std::os::raw::c_char, end: *mut *mut ::std::os::raw::c_char)
        -> f64;
}
extern "C" {
    pub fn R_tmpnam(
        prefix: *const ::std::os::raw::c_char,
        tempdir: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn R_tmpnam2(
        prefix: *const ::std::os::raw::c_char,
        tempdir: *const ::std::os::raw::c_char,
        fileext: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn R_free_tmpnam(name: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn R_CheckUserInterrupt();
}
extern "C" {
    pub fn R_CheckStack();
}
extern "C" {
    pub fn R_CheckStack2(arg1: usize);
}
extern "C" {
    pub fn findInterval(
        xt: *mut f64,
        n: ::std::os::raw::c_int,
        x: f64,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        ilo: ::std::os::raw::c_int,
        mflag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn findInterval2(
        xt: *mut f64,
        n: ::std::os::raw::c_int,
        x: f64,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        left_open: Rboolean,
        ilo: ::std::os::raw::c_int,
        mflag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn find_interv_vec(
        xt: *mut f64,
        n: *mut ::std::os::raw::c_int,
        x: *mut f64,
        nx: *mut ::std::os::raw::c_int,
        rightmost_closed: *mut ::std::os::raw::c_int,
        all_inside: *mut ::std::os::raw::c_int,
        indx: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn R_max_col(
        matrix: *mut f64,
        nr: *mut ::std::os::raw::c_int,
        nc: *mut ::std::os::raw::c_int,
        maxes: *mut ::std::os::raw::c_int,
        ties_meth: *mut ::std::os::raw::c_int,
    );
}
