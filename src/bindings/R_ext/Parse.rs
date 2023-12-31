/* automatically generated by rust-bindgen 0.66.1 */

pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
pub type va_list = *mut ::std::os::raw::c_char;
pub type __vcrt_bool = bool;
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
pub type FILE = _iobuf;
pub type fpos_t = ::std::os::raw::c_longlong;
pub type float_t = f32;
pub type double_t = f64;
pub type __gnuc_va_list = __builtin_va_list;
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
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
pub struct _iobuf {
    pub _Placeholder: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _exception {
    pub type_: ::std::os::raw::c_int,
    pub name: *mut ::std::os::raw::c_char,
    pub arg1: f64,
    pub arg2: f64,
    pub retval: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _complex {
    pub x: f64,
    pub y: f64,
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
pub const _CRT_INTERNAL_STDIO_SYMBOL_PREFIX: &[u8; 1] = b"\0";
pub const _CRT_INTERNAL_PRINTF_LEGACY_VSPRINTF_NULL_TERMINATION: u32 = 1;
pub const _CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR: u32 = 2;
pub const _CRT_INTERNAL_PRINTF_LEGACY_WIDE_SPECIFIERS: u32 = 4;
pub const _CRT_INTERNAL_PRINTF_LEGACY_MSVCRT_COMPATIBILITY: u32 = 8;
pub const _CRT_INTERNAL_PRINTF_LEGACY_THREE_DIGIT_EXPONENTS: u32 = 16;
pub const _CRT_INTERNAL_PRINTF_STANDARD_ROUNDING: u32 = 32;
pub const _CRT_INTERNAL_SCANF_SECURECRT: u32 = 1;
pub const _CRT_INTERNAL_SCANF_LEGACY_WIDE_SPECIFIERS: u32 = 2;
pub const _CRT_INTERNAL_SCANF_LEGACY_MSVCRT_COMPATIBILITY: u32 = 4;
pub const BUFSIZ: u32 = 512;
pub const _NSTREAM_: u32 = 512;
pub const _IOB_ENTRIES: u32 = 3;
pub const EOF: i32 = -1;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 64;
pub const _IONBF: u32 = 4;
pub const L_tmpnam: u32 = 260;
pub const L_tmpnam_s: u32 = 260;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_SET: u32 = 0;
pub const FILENAME_MAX: u32 = 260;
pub const FOPEN_MAX: u32 = 20;
pub const _SYS_OPEN: u32 = 20;
pub const TMP_MAX: u32 = 2147483647;
pub const TMP_MAX_S: u32 = 2147483647;
pub const _TMP_MAX_S: u32 = 2147483647;
pub const SYS_OPEN: u32 = 20;
pub const CHAR_BIT: u32 = 8;
pub const SCHAR_MIN: i32 = -128;
pub const SCHAR_MAX: u32 = 127;
pub const UCHAR_MAX: u32 = 255;
pub const CHAR_MIN: i32 = -128;
pub const CHAR_MAX: u32 = 127;
pub const MB_LEN_MAX: u32 = 5;
pub const SHRT_MIN: i32 = -32768;
pub const SHRT_MAX: u32 = 32767;
pub const USHRT_MAX: u32 = 65535;
pub const INT_MIN: i32 = -2147483648;
pub const INT_MAX: u32 = 2147483647;
pub const UINT_MAX: u32 = 4294967295;
pub const LONG_MIN: i32 = -2147483648;
pub const LONG_MAX: u32 = 2147483647;
pub const ULONG_MAX: u32 = 4294967295;
pub const CHAR_WIDTH: u32 = 8;
pub const SCHAR_WIDTH: u32 = 8;
pub const UCHAR_WIDTH: u32 = 8;
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
#[repr(i32)]
#[doc = "PARSE_NULL will not be returned by R_ParseVector"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ParseStatus {
    PARSE_NULL = 0,
    PARSE_OK = 1,
    PARSE_INCOMPLETE = 2,
    PARSE_ERROR = 3,
    PARSE_EOF = 4,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _double_val {
    pub _Sh: [::std::os::raw::c_ushort; 4usize],
    pub _Val: f64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _float_val {
    pub _Sh: [::std::os::raw::c_ushort; 2usize],
    pub _Val: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _ldouble_val {
    pub _Sh: [::std::os::raw::c_ushort; 4usize],
    pub _Val: f64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _float_const {
    pub _Word: [::std::os::raw::c_ushort; 4usize],
    pub _Float: f32,
    pub _Double: f64,
    pub _Long_double: f64,
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
    pub fn __acrt_iob_func(_Ix: ::std::os::raw::c_uint) -> *mut FILE;
    pub fn fgetwc(_Stream: *mut FILE) -> wint_t;
    pub fn _fgetwchar() -> wint_t;
    pub fn fputwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
    pub fn _fputwchar(_Character: wchar_t) -> wint_t;
    pub fn getwc(_Stream: *mut FILE) -> wint_t;
    pub fn getwchar() -> wint_t;
    pub fn fgetws(
        _Buffer: *mut wchar_t,
        _BufferCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut wchar_t;
    pub fn fputws(_Buffer: *const wchar_t, _Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _getws_s(_Buffer: *mut wchar_t, _BufferCount: usize) -> *mut wchar_t;
    pub fn putwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
    pub fn putwchar(_Character: wchar_t) -> wint_t;
    pub fn _putws(_Buffer: *const wchar_t) -> ::std::os::raw::c_int;
    pub fn ungetwc(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
    pub fn _wfdopen(_FileHandle: ::std::os::raw::c_int, _Mode: *const wchar_t) -> *mut FILE;
    pub fn _wfopen(_FileName: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
    pub fn _wfopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
    ) -> errno_t;
    pub fn _wfreopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> *mut FILE;
    pub fn _wfreopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> errno_t;
    pub fn _wfsopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
    pub fn _wperror(_ErrorMessage: *const wchar_t);
    pub fn _wpopen(_Command: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
    pub fn _wremove(_FileName: *const wchar_t) -> ::std::os::raw::c_int;
    pub fn _wtempnam(_Directory: *const wchar_t, _FilePrefix: *const wchar_t) -> *mut wchar_t;
    pub fn _wtmpnam_s(_Buffer: *mut wchar_t, _BufferCount: usize) -> errno_t;
    pub fn _wtmpnam(_Buffer: *mut wchar_t) -> *mut wchar_t;
    pub fn _fgetwc_nolock(_Stream: *mut FILE) -> wint_t;
    pub fn _fputwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
    pub fn _getwc_nolock(_Stream: *mut FILE) -> wint_t;
    pub fn _putwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
    pub fn _ungetwc_nolock(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
    pub fn __stdio_common_vfwprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vfwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vfwprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vfwscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vswprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vswprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vsnwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _MaxCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vswprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vswscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn _get_stream_buffer_pointers(
        _Stream: *mut FILE,
        _Base: *mut *mut *mut ::std::os::raw::c_char,
        _Pointer: *mut *mut *mut ::std::os::raw::c_char,
        _Count: *mut *mut ::std::os::raw::c_int,
    ) -> errno_t;
    pub fn clearerr_s(_Stream: *mut FILE) -> errno_t;
    pub fn fopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> errno_t;
    pub fn fread_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: usize,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
    pub fn freopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _OldStream: *mut FILE,
    ) -> errno_t;
    pub fn gets_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _Size: rsize_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn tmpfile_s(_Stream: *mut *mut FILE) -> errno_t;
    pub fn tmpnam_s(_Buffer: *mut ::std::os::raw::c_char, _Size: rsize_t) -> errno_t;
    pub fn clearerr(_Stream: *mut FILE);
    pub fn fclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _fcloseall() -> ::std::os::raw::c_int;
    pub fn _fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn feof(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn ferror(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fflush(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fgetc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _fgetchar() -> ::std::os::raw::c_int;
    pub fn fgetpos(_Stream: *mut FILE, _Position: *mut fpos_t) -> ::std::os::raw::c_int;
    pub fn fgets(
        _Buffer: *mut ::std::os::raw::c_char,
        _MaxCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
    pub fn _fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _flushall() -> ::std::os::raw::c_int;
    pub fn fopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn fputc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _fputchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn fputs(
        _Buffer: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn fread(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: ::std::os::raw::c_ulonglong,
        _ElementCount: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_ulonglong;
    pub fn freopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> *mut FILE;
    pub fn _fsopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
    pub fn fsetpos(_Stream: *mut FILE, _Position: *const fpos_t) -> ::std::os::raw::c_int;
    pub fn fseek(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn _fseeki64(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftell(_Stream: *mut FILE) -> ::std::os::raw::c_long;
    pub fn _ftelli64(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
    pub fn fwrite(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: ::std::os::raw::c_ulonglong,
        _ElementCount: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_ulonglong;
    pub fn getc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getchar() -> ::std::os::raw::c_int;
    pub fn _getmaxstdio() -> ::std::os::raw::c_int;
    pub fn _getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn perror(_ErrorMessage: *const ::std::os::raw::c_char);
    pub fn _pclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _popen(
        _Command: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn putc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn putchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn puts(_Buffer: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn _putw(_Word: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn remove(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn rename(
        _OldFileName: *const ::std::os::raw::c_char,
        _NewFileName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn _unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn rewind(_Stream: *mut FILE);
    pub fn _rmtmp() -> ::std::os::raw::c_int;
    pub fn setbuf(_Stream: *mut FILE, _Buffer: *mut ::std::os::raw::c_char);
    pub fn _setmaxstdio(_Maximum: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn setvbuf(
        _Stream: *mut FILE,
        _Buffer: *mut ::std::os::raw::c_char,
        _Mode: ::std::os::raw::c_int,
        _Size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn _tempnam(
        _DirectoryName: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpnam(_Buffer: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn ungetc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _lock_file(_Stream: *mut FILE);
    pub fn _unlock_file(_Stream: *mut FILE);
    pub fn _fclose_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _fflush_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _fgetc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _fputc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn _fread_nolock(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
    pub fn _fread_nolock_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: usize,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
    pub fn _fseek_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn _fseeki64_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn _ftell_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_long;
    pub fn _ftelli64_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
    pub fn _fwrite_nolock(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
    pub fn _getc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn _putc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn _ungetc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn __p__commode() -> *mut ::std::os::raw::c_int;
    pub fn __stdio_common_vfprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vfprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vfprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn _set_printf_count_output(_Value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn _get_printf_count_output() -> ::std::os::raw::c_int;
    pub fn __stdio_common_vfscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _Arglist: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vsprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vsprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vsnprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _MaxCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vsprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn __stdio_common_vsscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
    pub fn tempnam(
        _Directory: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn fcloseall() -> ::std::os::raw::c_int;
    pub fn fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Format: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn fgetchar() -> ::std::os::raw::c_int;
    pub fn fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn flushall() -> ::std::os::raw::c_int;
    pub fn fputchar(_Ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn putw(_Ch: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn rmtmp() -> ::std::os::raw::c_int;
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
    pub fn R_ParseVector(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ParseStatus,
        arg4: SEXP,
    ) -> SEXP;
}
