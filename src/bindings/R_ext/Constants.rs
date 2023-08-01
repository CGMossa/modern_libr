/* automatically generated by rust-bindgen 0.66.1 */

pub type va_list = *mut ::std::os::raw::c_char;
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type __crt_bool = bool;
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
pub type _locale_t = *mut __crt_locale_pointers;
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = usize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
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
pub const M_PI: f64 = 3.141592653589793;
pub const PI: f64 = 3.141592653589793;
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
pub const FLT_EVAL_METHOD: u32 = 0;
pub const DBL_DECIMAL_DIG: u32 = 17;
pub const DBL_DIG: u32 = 15;
pub const DBL_HAS_SUBNORM: u32 = 1;
pub const DBL_MANT_DIG: u32 = 53;
pub const DBL_MAX_10_EXP: u32 = 308;
pub const DBL_MAX_EXP: u32 = 1024;
pub const DBL_MIN_10_EXP: i32 = -307;
pub const DBL_MIN_EXP: i32 = -1021;
pub const _DBL_RADIX: u32 = 2;
pub const FLT_DECIMAL_DIG: u32 = 9;
pub const FLT_DIG: u32 = 6;
pub const FLT_HAS_SUBNORM: u32 = 1;
pub const FLT_GUARD: u32 = 0;
pub const FLT_MANT_DIG: u32 = 24;
pub const FLT_MAX_10_EXP: u32 = 38;
pub const FLT_MAX_EXP: u32 = 128;
pub const FLT_MIN_10_EXP: i32 = -37;
pub const FLT_MIN_EXP: i32 = -125;
pub const FLT_NORMALIZE: u32 = 0;
pub const FLT_RADIX: u32 = 2;
pub const LDBL_DIG: u32 = 15;
pub const LDBL_HAS_SUBNORM: u32 = 1;
pub const LDBL_MANT_DIG: u32 = 53;
pub const LDBL_MAX_10_EXP: u32 = 308;
pub const LDBL_MAX_EXP: u32 = 1024;
pub const LDBL_MIN_10_EXP: i32 = -307;
pub const LDBL_MIN_EXP: i32 = -1021;
pub const _LDBL_RADIX: u32 = 2;
pub const DECIMAL_DIG: u32 = 17;
pub const _SW_INEXACT: u32 = 1;
pub const _SW_UNDERFLOW: u32 = 2;
pub const _SW_OVERFLOW: u32 = 4;
pub const _SW_ZERODIVIDE: u32 = 8;
pub const _SW_INVALID: u32 = 16;
pub const _SW_DENORMAL: u32 = 524288;
pub const _EM_AMBIGUIOUS: u32 = 2147483648;
pub const _EM_AMBIGUOUS: u32 = 2147483648;
pub const _MCW_EM: u32 = 524319;
pub const _EM_INEXACT: u32 = 1;
pub const _EM_UNDERFLOW: u32 = 2;
pub const _EM_OVERFLOW: u32 = 4;
pub const _EM_ZERODIVIDE: u32 = 8;
pub const _EM_INVALID: u32 = 16;
pub const _EM_DENORMAL: u32 = 524288;
pub const _MCW_RC: u32 = 768;
pub const _RC_NEAR: u32 = 0;
pub const _RC_DOWN: u32 = 256;
pub const _RC_UP: u32 = 512;
pub const _RC_CHOP: u32 = 768;
pub const _MCW_PC: u32 = 196608;
pub const _PC_64: u32 = 0;
pub const _PC_53: u32 = 65536;
pub const _PC_24: u32 = 131072;
pub const _MCW_IC: u32 = 262144;
pub const _IC_AFFINE: u32 = 262144;
pub const _IC_PROJECTIVE: u32 = 0;
pub const _MCW_DN: u32 = 50331648;
pub const _DN_SAVE: u32 = 0;
pub const _DN_FLUSH: u32 = 16777216;
pub const _DN_FLUSH_OPERANDS_SAVE_RESULTS: u32 = 33554432;
pub const _DN_SAVE_OPERANDS_FLUSH_RESULTS: u32 = 50331648;
pub const _SW_UNEMULATED: u32 = 64;
pub const _SW_SQRTNEG: u32 = 128;
pub const _SW_STACKOVERFLOW: u32 = 512;
pub const _SW_STACKUNDERFLOW: u32 = 1024;
pub const _FPE_INVALID: u32 = 129;
pub const _FPE_DENORMAL: u32 = 130;
pub const _FPE_ZERODIVIDE: u32 = 131;
pub const _FPE_OVERFLOW: u32 = 132;
pub const _FPE_UNDERFLOW: u32 = 133;
pub const _FPE_INEXACT: u32 = 134;
pub const _FPE_UNEMULATED: u32 = 135;
pub const _FPE_SQRTNEG: u32 = 136;
pub const _FPE_STACKOVERFLOW: u32 = 138;
pub const _FPE_STACKUNDERFLOW: u32 = 139;
pub const _FPE_EXPLICITGEN: u32 = 140;
pub const _FPE_MULTIPLE_TRAPS: u32 = 141;
pub const _FPE_MULTIPLE_FAULTS: u32 = 142;
pub const _FPCLASS_SNAN: u32 = 1;
pub const _FPCLASS_QNAN: u32 = 2;
pub const _FPCLASS_NINF: u32 = 4;
pub const _FPCLASS_NN: u32 = 8;
pub const _FPCLASS_ND: u32 = 16;
pub const _FPCLASS_NZ: u32 = 32;
pub const _FPCLASS_PZ: u32 = 64;
pub const _FPCLASS_PD: u32 = 128;
pub const _FPCLASS_PN: u32 = 256;
pub const _FPCLASS_PINF: u32 = 512;
pub const _CW_DEFAULT: u32 = 524319;
pub const DBL_RADIX: u32 = 2;
pub const LDBL_RADIX: u32 = 2;
pub const EM_AMBIGUIOUS: u32 = 2147483648;
pub const EM_AMBIGUOUS: u32 = 2147483648;
pub const MCW_EM: u32 = 524319;
pub const EM_INVALID: u32 = 16;
pub const EM_DENORMAL: u32 = 524288;
pub const EM_ZERODIVIDE: u32 = 8;
pub const EM_OVERFLOW: u32 = 4;
pub const EM_UNDERFLOW: u32 = 2;
pub const EM_INEXACT: u32 = 1;
pub const MCW_IC: u32 = 262144;
pub const IC_AFFINE: u32 = 262144;
pub const IC_PROJECTIVE: u32 = 0;
pub const MCW_RC: u32 = 768;
pub const RC_CHOP: u32 = 768;
pub const RC_UP: u32 = 512;
pub const RC_DOWN: u32 = 256;
pub const RC_NEAR: u32 = 0;
pub const MCW_PC: u32 = 196608;
pub const PC_24: u32 = 131072;
pub const PC_53: u32 = 65536;
pub const PC_64: u32 = 0;
pub const CW_DEFAULT: u32 = 524319;
pub const SW_INVALID: u32 = 16;
pub const SW_DENORMAL: u32 = 524288;
pub const SW_ZERODIVIDE: u32 = 8;
pub const SW_OVERFLOW: u32 = 4;
pub const SW_UNDERFLOW: u32 = 2;
pub const SW_INEXACT: u32 = 1;
pub const SW_UNEMULATED: u32 = 64;
pub const SW_SQRTNEG: u32 = 128;
pub const SW_STACKOVERFLOW: u32 = 512;
pub const SW_STACKUNDERFLOW: u32 = 1024;
pub const FPE_INVALID: u32 = 129;
pub const FPE_DENORMAL: u32 = 130;
pub const FPE_ZERODIVIDE: u32 = 131;
pub const FPE_OVERFLOW: u32 = 132;
pub const FPE_UNDERFLOW: u32 = 133;
pub const FPE_INEXACT: u32 = 134;
pub const FPE_UNEMULATED: u32 = 135;
pub const FPE_SQRTNEG: u32 = 136;
pub const FPE_STACKOVERFLOW: u32 = 138;
pub const FPE_STACKUNDERFLOW: u32 = 139;
pub const FPE_EXPLICITGEN: u32 = 140;
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
    pub fn _clearfp() -> ::std::os::raw::c_uint;
    pub fn _controlfp(
        _NewValue: ::std::os::raw::c_uint,
        _Mask: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
    pub fn _set_controlfp(_NewValue: ::std::os::raw::c_uint, _Mask: ::std::os::raw::c_uint);
    pub fn _controlfp_s(
        _CurrentState: *mut ::std::os::raw::c_uint,
        _NewValue: ::std::os::raw::c_uint,
        _Mask: ::std::os::raw::c_uint,
    ) -> errno_t;
    pub fn _statusfp() -> ::std::os::raw::c_uint;
    pub fn _fpreset();
    pub fn _control87(
        _NewValue: ::std::os::raw::c_uint,
        _Mask: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
    pub fn __fpecode() -> *mut ::std::os::raw::c_int;
    pub fn __fpe_flt_rounds() -> ::std::os::raw::c_int;
    pub fn _copysign(_Number: f64, _Sign: f64) -> f64;
    pub fn _chgsign(_X: f64) -> f64;
    pub fn _scalb(_X: f64, _Y: ::std::os::raw::c_long) -> f64;
    pub fn _logb(_X: f64) -> f64;
    pub fn _nextafter(_X: f64, _Y: f64) -> f64;
    pub fn _finite(_X: f64) -> ::std::os::raw::c_int;
    pub fn _isnan(_X: f64) -> ::std::os::raw::c_int;
    pub fn _fpclass(_X: f64) -> ::std::os::raw::c_int;
    pub fn _scalbf(_X: f32, _Y: ::std::os::raw::c_long) -> f32;
    pub fn fpreset();
}
