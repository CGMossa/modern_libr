/* automatically generated by rust-bindgen 0.66.1 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE: u32 = 1;
pub const _CRT_BUILD_DESKTOP_APP: u32 = 1;
pub const _ARGMAX: u32 = 100;
pub const _CRT_INT_MAX: u32 = 2147483647;
pub const _CRT_FUNCTIONS_REQUIRED: u32 = 1;
pub const _CRT_HAS_CXX17: u32 = 0;
pub const _CRT_HAS_C11: u32 = 1;
pub const _CRT_INTERNAL_NONSTDC_NAMES: u32 = 1;
pub const __STDC_SECURE_LIB__: u32 = 200411;
pub const __GOT_SECURE_LIB__: u32 = 200411;
pub const __STDC_WANT_SECURE_LIB__: u32 = 1;
pub const _SECURECRT_FILL_BUFFER_PATTERN: u32 = 254;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_COUNT: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES: u32 = 1;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_MEMORY: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES_MEMORY: u32 = 0;
pub const _DOMAIN: u32 = 1;
pub const _SING: u32 = 2;
pub const _OVERFLOW: u32 = 3;
pub const _UNDERFLOW: u32 = 4;
pub const _TLOSS: u32 = 5;
pub const _PLOSS: u32 = 6;
pub const _HUGE_ENUF : f64 = 1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0 ;
pub const _DENORM: i32 = -2;
pub const _FINITE: i32 = -1;
pub const _INFCODE: u32 = 1;
pub const _NANCODE: u32 = 2;
pub const FP_INFINITE: u32 = 1;
pub const FP_NAN: u32 = 2;
pub const FP_NORMAL: i32 = -1;
pub const FP_SUBNORMAL: i32 = -2;
pub const FP_ZERO: u32 = 0;
pub const _C2: u32 = 1;
pub const FP_ILOGB0: i32 = -2147483648;
pub const FP_ILOGBNAN: u32 = 2147483647;
pub const MATH_ERRNO: u32 = 1;
pub const MATH_ERREXCEPT: u32 = 2;
pub const math_errhandling: u32 = 3;
pub const _FE_DIVBYZERO: u32 = 4;
pub const _FE_INEXACT: u32 = 32;
pub const _FE_INVALID: u32 = 1;
pub const _FE_OVERFLOW: u32 = 8;
pub const _FE_UNDERFLOW: u32 = 16;
pub const _D0_C: u32 = 3;
pub const _D1_C: u32 = 2;
pub const _D2_C: u32 = 1;
pub const _D3_C: u32 = 0;
pub const _DBIAS: u32 = 1022;
pub const _DOFF: u32 = 4;
pub const _F0_C: u32 = 1;
pub const _F1_C: u32 = 0;
pub const _FBIAS: u32 = 126;
pub const _FOFF: u32 = 7;
pub const _FRND: u32 = 1;
pub const _L0_C: u32 = 3;
pub const _L1_C: u32 = 2;
pub const _L2_C: u32 = 1;
pub const _L3_C: u32 = 0;
pub const _LBIAS: u32 = 1022;
pub const _LOFF: u32 = 4;
pub const _FP_LT: u32 = 1;
pub const _FP_EQ: u32 = 2;
pub const _FP_GT: u32 = 4;
pub const DOMAIN: u32 = 1;
pub const SING: u32 = 2;
pub const OVERFLOW: u32 = 3;
pub const UNDERFLOW: u32 = 4;
pub const TLOSS: u32 = 5;
pub const PLOSS: u32 = 6;
pub type max_align_t = f64;
pub type va_list = *mut ::std::os::raw::c_char;
pub type __vcrt_bool = bool;
pub type __crt_bool = bool;
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___crt_locale_data_public() {
    const UNINIT: ::std::mem::MaybeUninit<__crt_locale_data_public> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_data_public>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_data_public>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._locale_pctype) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_pctype)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._locale_mb_cur_max) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_mb_cur_max)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._locale_lc_codepage) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_lc_codepage)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}
#[test]
fn bindgen_test_layout___crt_locale_pointers() {
    const UNINIT: ::std::mem::MaybeUninit<__crt_locale_pointers> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).locinfo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(locinfo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mbcinfo) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(mbcinfo)
        )
    );
}
pub type _locale_t = *mut __crt_locale_pointers;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout__Mbstatet() {
    const UNINIT: ::std::mem::MaybeUninit<_Mbstatet> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_Mbstatet>(),
        8usize,
        concat!("Size of: ", stringify!(_Mbstatet))
    );
    assert_eq!(
        ::std::mem::align_of::<_Mbstatet>(),
        4usize,
        concat!("Alignment of ", stringify!(_Mbstatet))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Wchar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Wchar)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Byte) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Byte)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._State) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_State)
        )
    );
}
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = usize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _exception {
    pub type_: ::std::os::raw::c_int,
    pub name: *mut ::std::os::raw::c_char,
    pub arg1: f64,
    pub arg2: f64,
    pub retval: f64,
}
#[test]
fn bindgen_test_layout__exception() {
    const UNINIT: ::std::mem::MaybeUninit<_exception> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_exception>(),
        40usize,
        concat!("Size of: ", stringify!(_exception))
    );
    assert_eq!(
        ::std::mem::align_of::<_exception>(),
        8usize,
        concat!("Alignment of ", stringify!(_exception))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_exception),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_exception),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arg1) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_exception),
            "::",
            stringify!(arg1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arg2) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_exception),
            "::",
            stringify!(arg2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).retval) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_exception),
            "::",
            stringify!(retval)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _complex {
    pub x: f64,
    pub y: f64,
}
#[test]
fn bindgen_test_layout__complex() {
    const UNINIT: ::std::mem::MaybeUninit<_complex> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_complex>(),
        16usize,
        concat!("Size of: ", stringify!(_complex))
    );
    assert_eq!(
        ::std::mem::align_of::<_complex>(),
        8usize,
        concat!("Alignment of ", stringify!(_complex))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_complex),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_complex),
            "::",
            stringify!(y)
        )
    );
}
pub type float_t = f32;
pub type double_t = f64;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _double_val {
    pub _Sh: [::std::os::raw::c_ushort; 4usize],
    pub _Val: f64,
}
#[test]
fn bindgen_test_layout__double_val() {
    const UNINIT: ::std::mem::MaybeUninit<_double_val> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_double_val>(),
        8usize,
        concat!("Size of: ", stringify!(_double_val))
    );
    assert_eq!(
        ::std::mem::align_of::<_double_val>(),
        8usize,
        concat!("Alignment of ", stringify!(_double_val))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Sh) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_double_val),
            "::",
            stringify!(_Sh)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_double_val),
            "::",
            stringify!(_Val)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _float_val {
    pub _Sh: [::std::os::raw::c_ushort; 2usize],
    pub _Val: f32,
}
#[test]
fn bindgen_test_layout__float_val() {
    const UNINIT: ::std::mem::MaybeUninit<_float_val> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_float_val>(),
        4usize,
        concat!("Size of: ", stringify!(_float_val))
    );
    assert_eq!(
        ::std::mem::align_of::<_float_val>(),
        4usize,
        concat!("Alignment of ", stringify!(_float_val))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Sh) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_float_val),
            "::",
            stringify!(_Sh)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_float_val),
            "::",
            stringify!(_Val)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _ldouble_val {
    pub _Sh: [::std::os::raw::c_ushort; 4usize],
    pub _Val: f64,
}
#[test]
fn bindgen_test_layout__ldouble_val() {
    const UNINIT: ::std::mem::MaybeUninit<_ldouble_val> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_ldouble_val>(),
        8usize,
        concat!("Size of: ", stringify!(_ldouble_val))
    );
    assert_eq!(
        ::std::mem::align_of::<_ldouble_val>(),
        8usize,
        concat!("Alignment of ", stringify!(_ldouble_val))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Sh) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ldouble_val),
            "::",
            stringify!(_Sh)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ldouble_val),
            "::",
            stringify!(_Val)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _float_const {
    pub _Word: [::std::os::raw::c_ushort; 4usize],
    pub _Float: f32,
    pub _Double: f64,
    pub _Long_double: f64,
}
#[test]
fn bindgen_test_layout__float_const() {
    const UNINIT: ::std::mem::MaybeUninit<_float_const> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_float_const>(),
        8usize,
        concat!("Size of: ", stringify!(_float_const))
    );
    assert_eq!(
        ::std::mem::align_of::<_float_const>(),
        8usize,
        concat!("Alignment of ", stringify!(_float_const))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Word) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_float_const),
            "::",
            stringify!(_Word)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Float) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_float_const),
            "::",
            stringify!(_Float)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Double) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_float_const),
            "::",
            stringify!(_Double)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Long_double) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_float_const),
            "::",
            stringify!(_Long_double)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
    pub fn __security_init_cookie();
    pub fn __security_check_cookie(_StackCookie: usize);
    pub fn __report_gsfailure(_StackCookie: usize) -> !;
    pub static mut __security_cookie: usize;
    pub fn _invalid_parameter_noinfo();
    pub fn _invalid_parameter_noinfo_noreturn() -> !;
    pub fn _invoke_watson(
        _Expression: *const wchar_t,
        _FunctionName: *const wchar_t,
        _FileName: *const wchar_t,
        _LineNo: ::std::os::raw::c_uint,
        _Reserved: usize,
    ) -> !;
    pub static _HUGE: f64;
    pub fn _fperrraise(_Except: ::std::os::raw::c_int);
    pub fn _dclass(_X: f64) -> ::std::os::raw::c_short;
    pub fn _ldclass(_X: f64) -> ::std::os::raw::c_short;
    pub fn _fdclass(_X: f32) -> ::std::os::raw::c_short;
    pub fn _dsign(_X: f64) -> ::std::os::raw::c_int;
    pub fn _ldsign(_X: f64) -> ::std::os::raw::c_int;
    pub fn _fdsign(_X: f32) -> ::std::os::raw::c_int;
    pub fn _dpcomp(_X: f64, _Y: f64) -> ::std::os::raw::c_int;
    pub fn _ldpcomp(_X: f64, _Y: f64) -> ::std::os::raw::c_int;
    pub fn _fdpcomp(_X: f32, _Y: f32) -> ::std::os::raw::c_int;
    pub fn _dtest(_Px: *mut f64) -> ::std::os::raw::c_short;
    pub fn _ldtest(_Px: *mut f64) -> ::std::os::raw::c_short;
    pub fn _fdtest(_Px: *mut f32) -> ::std::os::raw::c_short;
    pub fn _d_int(_Px: *mut f64, _Xexp: ::std::os::raw::c_short) -> ::std::os::raw::c_short;
    pub fn _ld_int(_Px: *mut f64, _Xexp: ::std::os::raw::c_short) -> ::std::os::raw::c_short;
    pub fn _fd_int(_Px: *mut f32, _Xexp: ::std::os::raw::c_short) -> ::std::os::raw::c_short;
    pub fn _dscale(_Px: *mut f64, _Lexp: ::std::os::raw::c_long) -> ::std::os::raw::c_short;
    pub fn _ldscale(_Px: *mut f64, _Lexp: ::std::os::raw::c_long) -> ::std::os::raw::c_short;
    pub fn _fdscale(_Px: *mut f32, _Lexp: ::std::os::raw::c_long) -> ::std::os::raw::c_short;
    pub fn _dunscale(_Pex: *mut ::std::os::raw::c_short, _Px: *mut f64) -> ::std::os::raw::c_short;
    pub fn _ldunscale(_Pex: *mut ::std::os::raw::c_short, _Px: *mut f64)
        -> ::std::os::raw::c_short;
    pub fn _fdunscale(_Pex: *mut ::std::os::raw::c_short, _Px: *mut f32)
        -> ::std::os::raw::c_short;
    pub fn _dexp(_Px: *mut f64, _Y: f64, _Eoff: ::std::os::raw::c_long) -> ::std::os::raw::c_short;
    pub fn _ldexp(_Px: *mut f64, _Y: f64, _Eoff: ::std::os::raw::c_long)
        -> ::std::os::raw::c_short;
    pub fn _fdexp(_Px: *mut f32, _Y: f32, _Eoff: ::std::os::raw::c_long)
        -> ::std::os::raw::c_short;
    pub fn _dnorm(_Ps: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_short;
    pub fn _fdnorm(_Ps: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_short;
    pub fn _dpoly(_X: f64, _Tab: *const f64, _N: ::std::os::raw::c_int) -> f64;
    pub fn _ldpoly(_X: f64, _Tab: *const f64, _N: ::std::os::raw::c_int) -> f64;
    pub fn _fdpoly(_X: f32, _Tab: *const f32, _N: ::std::os::raw::c_int) -> f32;
    pub fn _dlog(_X: f64, _Baseflag: ::std::os::raw::c_int) -> f64;
    pub fn _ldlog(_X: f64, _Baseflag: ::std::os::raw::c_int) -> f64;
    pub fn _fdlog(_X: f32, _Baseflag: ::std::os::raw::c_int) -> f32;
    pub fn _dsin(_X: f64, _Qoff: ::std::os::raw::c_uint) -> f64;
    pub fn _ldsin(_X: f64, _Qoff: ::std::os::raw::c_uint) -> f64;
    pub fn _fdsin(_X: f32, _Qoff: ::std::os::raw::c_uint) -> f32;
    pub static _Denorm_C: _float_const;
    pub static _Inf_C: _float_const;
    pub static _Nan_C: _float_const;
    pub static _Snan_C: _float_const;
    pub static _Hugeval_C: _float_const;
    pub static _FDenorm_C: _float_const;
    pub static _FInf_C: _float_const;
    pub static _FNan_C: _float_const;
    pub static _FSnan_C: _float_const;
    pub static _LDenorm_C: _float_const;
    pub static _LInf_C: _float_const;
    pub static _LNan_C: _float_const;
    pub static _LSnan_C: _float_const;
    pub static _Eps_C: _float_const;
    pub static _Rteps_C: _float_const;
    pub static _FEps_C: _float_const;
    pub static _FRteps_C: _float_const;
    pub static _LEps_C: _float_const;
    pub static _LRteps_C: _float_const;
    pub static _Zero_C: f64;
    pub static _Xbig_C: f64;
    pub static _FZero_C: f32;
    pub static _FXbig_C: f32;
    pub static _LZero_C: f64;
    pub static _LXbig_C: f64;
    pub fn abs(_X: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn labs(_X: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
    pub fn llabs(_X: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
    pub fn acos(_X: f64) -> f64;
    pub fn asin(_X: f64) -> f64;
    pub fn atan(_X: f64) -> f64;
    pub fn atan2(_Y: f64, _X: f64) -> f64;
    pub fn cos(_X: f64) -> f64;
    pub fn cosh(_X: f64) -> f64;
    pub fn exp(_X: f64) -> f64;
    pub fn fabs(_X: f64) -> f64;
    pub fn fmod(_X: f64, _Y: f64) -> f64;
    pub fn log(_X: f64) -> f64;
    pub fn log10(_X: f64) -> f64;
    pub fn pow(_X: f64, _Y: f64) -> f64;
    pub fn sin(_X: f64) -> f64;
    pub fn sinh(_X: f64) -> f64;
    pub fn sqrt(_X: f64) -> f64;
    pub fn tan(_X: f64) -> f64;
    pub fn tanh(_X: f64) -> f64;
    pub fn acosh(_X: f64) -> f64;
    pub fn asinh(_X: f64) -> f64;
    pub fn atanh(_X: f64) -> f64;
    pub fn atof(_String: *const ::std::os::raw::c_char) -> f64;
    pub fn _atof_l(_String: *const ::std::os::raw::c_char, _Locale: _locale_t) -> f64;
    pub fn _cabs(_Complex_value: _complex) -> f64;
    pub fn cbrt(_X: f64) -> f64;
    pub fn ceil(_X: f64) -> f64;
    pub fn _chgsign(_X: f64) -> f64;
    pub fn copysign(_Number: f64, _Sign: f64) -> f64;
    pub fn _copysign(_Number: f64, _Sign: f64) -> f64;
    pub fn erf(_X: f64) -> f64;
    pub fn erfc(_X: f64) -> f64;
    pub fn exp2(_X: f64) -> f64;
    pub fn expm1(_X: f64) -> f64;
    pub fn fdim(_X: f64, _Y: f64) -> f64;
    pub fn floor(_X: f64) -> f64;
    pub fn fma(_X: f64, _Y: f64, _Z: f64) -> f64;
    pub fn fmax(_X: f64, _Y: f64) -> f64;
    pub fn fmin(_X: f64, _Y: f64) -> f64;
    pub fn frexp(_X: f64, _Y: *mut ::std::os::raw::c_int) -> f64;
    pub fn hypot(_X: f64, _Y: f64) -> f64;
    pub fn _hypot(_X: f64, _Y: f64) -> f64;
    pub fn ilogb(_X: f64) -> ::std::os::raw::c_int;
    pub fn ldexp(_X: f64, _Y: ::std::os::raw::c_int) -> f64;
    pub fn lgamma(_X: f64) -> f64;
    pub fn llrint(_X: f64) -> ::std::os::raw::c_longlong;
    pub fn llround(_X: f64) -> ::std::os::raw::c_longlong;
    pub fn log1p(_X: f64) -> f64;
    pub fn log2(_X: f64) -> f64;
    pub fn logb(_X: f64) -> f64;
    pub fn lrint(_X: f64) -> ::std::os::raw::c_long;
    pub fn lround(_X: f64) -> ::std::os::raw::c_long;
    pub fn _matherr(_Except: *mut _exception) -> ::std::os::raw::c_int;
    pub fn modf(_X: f64, _Y: *mut f64) -> f64;
    pub fn nan(_X: *const ::std::os::raw::c_char) -> f64;
    pub fn nearbyint(_X: f64) -> f64;
    pub fn nextafter(_X: f64, _Y: f64) -> f64;
    pub fn nexttoward(_X: f64, _Y: f64) -> f64;
    pub fn remainder(_X: f64, _Y: f64) -> f64;
    pub fn remquo(_X: f64, _Y: f64, _Z: *mut ::std::os::raw::c_int) -> f64;
    pub fn rint(_X: f64) -> f64;
    pub fn round(_X: f64) -> f64;
    pub fn scalbln(_X: f64, _Y: ::std::os::raw::c_long) -> f64;
    pub fn scalbn(_X: f64, _Y: ::std::os::raw::c_int) -> f64;
    pub fn tgamma(_X: f64) -> f64;
    pub fn trunc(_X: f64) -> f64;
    pub fn _j0(_X: f64) -> f64;
    pub fn _j1(_X: f64) -> f64;
    pub fn _jn(_X: ::std::os::raw::c_int, _Y: f64) -> f64;
    pub fn _y0(_X: f64) -> f64;
    pub fn _y1(_X: f64) -> f64;
    pub fn _yn(_X: ::std::os::raw::c_int, _Y: f64) -> f64;
    pub fn acoshf(_X: f32) -> f32;
    pub fn asinhf(_X: f32) -> f32;
    pub fn atanhf(_X: f32) -> f32;
    pub fn cbrtf(_X: f32) -> f32;
    pub fn _chgsignf(_X: f32) -> f32;
    pub fn copysignf(_Number: f32, _Sign: f32) -> f32;
    pub fn _copysignf(_Number: f32, _Sign: f32) -> f32;
    pub fn erff(_X: f32) -> f32;
    pub fn erfcf(_X: f32) -> f32;
    pub fn expm1f(_X: f32) -> f32;
    pub fn exp2f(_X: f32) -> f32;
    pub fn fdimf(_X: f32, _Y: f32) -> f32;
    pub fn fmaf(_X: f32, _Y: f32, _Z: f32) -> f32;
    pub fn fmaxf(_X: f32, _Y: f32) -> f32;
    pub fn fminf(_X: f32, _Y: f32) -> f32;
    pub fn _hypotf(_X: f32, _Y: f32) -> f32;
    pub fn ilogbf(_X: f32) -> ::std::os::raw::c_int;
    pub fn lgammaf(_X: f32) -> f32;
    pub fn llrintf(_X: f32) -> ::std::os::raw::c_longlong;
    pub fn llroundf(_X: f32) -> ::std::os::raw::c_longlong;
    pub fn log1pf(_X: f32) -> f32;
    pub fn log2f(_X: f32) -> f32;
    pub fn logbf(_X: f32) -> f32;
    pub fn lrintf(_X: f32) -> ::std::os::raw::c_long;
    pub fn lroundf(_X: f32) -> ::std::os::raw::c_long;
    pub fn nanf(_X: *const ::std::os::raw::c_char) -> f32;
    pub fn nearbyintf(_X: f32) -> f32;
    pub fn nextafterf(_X: f32, _Y: f32) -> f32;
    pub fn nexttowardf(_X: f32, _Y: f64) -> f32;
    pub fn remainderf(_X: f32, _Y: f32) -> f32;
    pub fn remquof(_X: f32, _Y: f32, _Z: *mut ::std::os::raw::c_int) -> f32;
    pub fn rintf(_X: f32) -> f32;
    pub fn roundf(_X: f32) -> f32;
    pub fn scalblnf(_X: f32, _Y: ::std::os::raw::c_long) -> f32;
    pub fn scalbnf(_X: f32, _Y: ::std::os::raw::c_int) -> f32;
    pub fn tgammaf(_X: f32) -> f32;
    pub fn truncf(_X: f32) -> f32;
    pub fn _logbf(_X: f32) -> f32;
    pub fn _nextafterf(_X: f32, _Y: f32) -> f32;
    pub fn _finitef(_X: f32) -> ::std::os::raw::c_int;
    pub fn _isnanf(_X: f32) -> ::std::os::raw::c_int;
    pub fn _fpclassf(_X: f32) -> ::std::os::raw::c_int;
    pub fn _set_FMA3_enable(_Flag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn _get_FMA3_enable() -> ::std::os::raw::c_int;
    pub fn acosf(_X: f32) -> f32;
    pub fn asinf(_X: f32) -> f32;
    pub fn atan2f(_Y: f32, _X: f32) -> f32;
    pub fn atanf(_X: f32) -> f32;
    pub fn ceilf(_X: f32) -> f32;
    pub fn cosf(_X: f32) -> f32;
    pub fn coshf(_X: f32) -> f32;
    pub fn expf(_X: f32) -> f32;
    pub fn floorf(_X: f32) -> f32;
    pub fn fmodf(_X: f32, _Y: f32) -> f32;
    pub fn log10f(_X: f32) -> f32;
    pub fn logf(_X: f32) -> f32;
    pub fn modff(_X: f32, _Y: *mut f32) -> f32;
    pub fn powf(_X: f32, _Y: f32) -> f32;
    pub fn sinf(_X: f32) -> f32;
    pub fn sinhf(_X: f32) -> f32;
    pub fn sqrtf(_X: f32) -> f32;
    pub fn tanf(_X: f32) -> f32;
    pub fn tanhf(_X: f32) -> f32;
    pub fn acoshl(_X: f64) -> f64;
    pub fn asinhl(_X: f64) -> f64;
    pub fn atanhl(_X: f64) -> f64;
    pub fn cbrtl(_X: f64) -> f64;
    pub fn copysignl(_Number: f64, _Sign: f64) -> f64;
    pub fn erfl(_X: f64) -> f64;
    pub fn erfcl(_X: f64) -> f64;
    pub fn exp2l(_X: f64) -> f64;
    pub fn expm1l(_X: f64) -> f64;
    pub fn fdiml(_X: f64, _Y: f64) -> f64;
    pub fn fmal(_X: f64, _Y: f64, _Z: f64) -> f64;
    pub fn fmaxl(_X: f64, _Y: f64) -> f64;
    pub fn fminl(_X: f64, _Y: f64) -> f64;
    pub fn ilogbl(_X: f64) -> ::std::os::raw::c_int;
    pub fn lgammal(_X: f64) -> f64;
    pub fn llrintl(_X: f64) -> ::std::os::raw::c_longlong;
    pub fn llroundl(_X: f64) -> ::std::os::raw::c_longlong;
    pub fn log1pl(_X: f64) -> f64;
    pub fn log2l(_X: f64) -> f64;
    pub fn logbl(_X: f64) -> f64;
    pub fn lrintl(_X: f64) -> ::std::os::raw::c_long;
    pub fn lroundl(_X: f64) -> ::std::os::raw::c_long;
    pub fn nanl(_X: *const ::std::os::raw::c_char) -> f64;
    pub fn nearbyintl(_X: f64) -> f64;
    pub fn nextafterl(_X: f64, _Y: f64) -> f64;
    pub fn nexttowardl(_X: f64, _Y: f64) -> f64;
    pub fn remainderl(_X: f64, _Y: f64) -> f64;
    pub fn remquol(_X: f64, _Y: f64, _Z: *mut ::std::os::raw::c_int) -> f64;
    pub fn rintl(_X: f64) -> f64;
    pub fn roundl(_X: f64) -> f64;
    pub fn scalblnl(_X: f64, _Y: ::std::os::raw::c_long) -> f64;
    pub fn scalbnl(_X: f64, _Y: ::std::os::raw::c_int) -> f64;
    pub fn tgammal(_X: f64) -> f64;
    pub fn truncl(_X: f64) -> f64;
    pub static mut HUGE: f64;
    pub fn j0(_X: f64) -> f64;
    pub fn j1(_X: f64) -> f64;
    pub fn jn(_X: ::std::os::raw::c_int, _Y: f64) -> f64;
    pub fn y0(_X: f64) -> f64;
    pub fn y1(_X: f64) -> f64;
    pub fn yn(_X: ::std::os::raw::c_int, _Y: f64) -> f64;
    pub static mut R_NaN: f64;
    pub static mut R_PosInf: f64;
    pub static mut R_NegInf: f64;
    pub static mut R_NaReal: f64;
    pub static mut R_NaInt: ::std::os::raw::c_int;
    pub fn R_IsNA(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_IsNaN(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_finite(arg1: f64) -> ::std::os::raw::c_int;
}
