/* automatically generated by rust-bindgen 0.66.1 */

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
pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const ESRCH: u32 = 3;
pub const EINTR: u32 = 4;
pub const EIO: u32 = 5;
pub const ENXIO: u32 = 6;
pub const E2BIG: u32 = 7;
pub const ENOEXEC: u32 = 8;
pub const EBADF: u32 = 9;
pub const ECHILD: u32 = 10;
pub const EAGAIN: u32 = 11;
pub const ENOMEM: u32 = 12;
pub const EACCES: u32 = 13;
pub const EFAULT: u32 = 14;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const EXDEV: u32 = 18;
pub const ENODEV: u32 = 19;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const ENFILE: u32 = 23;
pub const EMFILE: u32 = 24;
pub const ENOTTY: u32 = 25;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const ESPIPE: u32 = 29;
pub const EROFS: u32 = 30;
pub const EMLINK: u32 = 31;
pub const EPIPE: u32 = 32;
pub const EDOM: u32 = 33;
pub const EDEADLK: u32 = 36;
pub const ENAMETOOLONG: u32 = 38;
pub const ENOLCK: u32 = 39;
pub const ENOSYS: u32 = 40;
pub const ENOTEMPTY: u32 = 41;
pub const EINVAL: u32 = 22;
pub const ERANGE: u32 = 34;
pub const EILSEQ: u32 = 42;
pub const STRUNCATE: u32 = 80;
pub const EDEADLOCK: u32 = 36;
pub const EADDRINUSE: u32 = 100;
pub const EADDRNOTAVAIL: u32 = 101;
pub const EAFNOSUPPORT: u32 = 102;
pub const EALREADY: u32 = 103;
pub const EBADMSG: u32 = 104;
pub const ECANCELED: u32 = 105;
pub const ECONNABORTED: u32 = 106;
pub const ECONNREFUSED: u32 = 107;
pub const ECONNRESET: u32 = 108;
pub const EDESTADDRREQ: u32 = 109;
pub const EHOSTUNREACH: u32 = 110;
pub const EIDRM: u32 = 111;
pub const EINPROGRESS: u32 = 112;
pub const EISCONN: u32 = 113;
pub const ELOOP: u32 = 114;
pub const EMSGSIZE: u32 = 115;
pub const ENETDOWN: u32 = 116;
pub const ENETRESET: u32 = 117;
pub const ENETUNREACH: u32 = 118;
pub const ENOBUFS: u32 = 119;
pub const ENODATA: u32 = 120;
pub const ENOLINK: u32 = 121;
pub const ENOMSG: u32 = 122;
pub const ENOPROTOOPT: u32 = 123;
pub const ENOSR: u32 = 124;
pub const ENOSTR: u32 = 125;
pub const ENOTCONN: u32 = 126;
pub const ENOTRECOVERABLE: u32 = 127;
pub const ENOTSOCK: u32 = 128;
pub const ENOTSUP: u32 = 129;
pub const EOPNOTSUPP: u32 = 130;
pub const EOTHER: u32 = 131;
pub const EOVERFLOW: u32 = 132;
pub const EOWNERDEAD: u32 = 133;
pub const EPROTO: u32 = 134;
pub const EPROTONOSUPPORT: u32 = 135;
pub const EPROTOTYPE: u32 = 136;
pub const ETIME: u32 = 137;
pub const ETIMEDOUT: u32 = 138;
pub const ETXTBSY: u32 = 139;
pub const EWOULDBLOCK: u32 = 140;
pub const _NLSCMPERROR: u32 = 2147483647;
pub const HAVE_F77_UNDERSCORE: u32 = 1;
pub const IEEE_754: u32 = 1;
pub const SUPPORT_UTF8: u32 = 1;
pub const SUPPORT_MBCS: u32 = 1;
pub const ENABLE_NLS: u32 = 1;
pub const PR18534fixed: u32 = 1;
pub const SIZEOF_SIZE_T: u32 = 8;
pub const HAVE_UINTPTR_T: u32 = 1;
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
    pub fn _errno() -> *mut ::std::os::raw::c_int;
    pub fn _set_errno(_Value: ::std::os::raw::c_int) -> errno_t;
    pub fn _get_errno(_Value: *mut ::std::os::raw::c_int) -> errno_t;
    pub fn __doserrno() -> *mut ::std::os::raw::c_ulong;
    pub fn _set_doserrno(_Value: ::std::os::raw::c_ulong) -> errno_t;
    pub fn _get_doserrno(_Value: *mut ::std::os::raw::c_ulong) -> errno_t;
    pub fn memchr(
        _Buf: *const ::std::os::raw::c_void,
        _Val: ::std::os::raw::c_int,
        _MaxCount: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memcmp(
        _Buf1: *const ::std::os::raw::c_void,
        _Buf2: *const ::std::os::raw::c_void,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
    pub fn memcpy(
        _Dst: *mut ::std::os::raw::c_void,
        _Src: *const ::std::os::raw::c_void,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memmove(
        _Dst: *mut ::std::os::raw::c_void,
        _Src: *const ::std::os::raw::c_void,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memset(
        _Dst: *mut ::std::os::raw::c_void,
        _Val: ::std::os::raw::c_int,
        _Size: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_void;
    pub fn strchr(
        _Str: *const ::std::os::raw::c_char,
        _Val: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strrchr(
        _Str: *const ::std::os::raw::c_char,
        _Ch: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strstr(
        _Str: *const ::std::os::raw::c_char,
        _SubStr: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn wcschr(
        _Str: *const ::std::os::raw::c_ushort,
        _Ch: ::std::os::raw::c_ushort,
    ) -> *mut ::std::os::raw::c_ushort;
    pub fn wcsrchr(_Str: *const wchar_t, _Ch: wchar_t) -> *mut wchar_t;
    pub fn wcsstr(_Str: *const wchar_t, _SubStr: *const wchar_t) -> *mut wchar_t;
    pub fn _memicmp(
        _Buf1: *const ::std::os::raw::c_void,
        _Buf2: *const ::std::os::raw::c_void,
        _Size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn _memicmp_l(
        _Buf1: *const ::std::os::raw::c_void,
        _Buf2: *const ::std::os::raw::c_void,
        _Size: usize,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn memccpy(
        _Dst: *mut ::std::os::raw::c_void,
        _Src: *const ::std::os::raw::c_void,
        _Val: ::std::os::raw::c_int,
        _Size: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memicmp(
        _Buf1: *const ::std::os::raw::c_void,
        _Buf2: *const ::std::os::raw::c_void,
        _Size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn wcscat_s(
        _Destination: *mut wchar_t,
        _SizeInWords: rsize_t,
        _Source: *const wchar_t,
    ) -> errno_t;
    pub fn wcscpy_s(
        _Destination: *mut wchar_t,
        _SizeInWords: rsize_t,
        _Source: *const wchar_t,
    ) -> errno_t;
    pub fn wcsncat_s(
        _Destination: *mut wchar_t,
        _SizeInWords: rsize_t,
        _Source: *const wchar_t,
        _MaxCount: rsize_t,
    ) -> errno_t;
    pub fn wcsncpy_s(
        _Destination: *mut wchar_t,
        _SizeInWords: rsize_t,
        _Source: *const wchar_t,
        _MaxCount: rsize_t,
    ) -> errno_t;
    pub fn wcstok_s(
        _String: *mut wchar_t,
        _Delimiter: *const wchar_t,
        _Context: *mut *mut wchar_t,
    ) -> *mut wchar_t;
    pub fn _wcsdup(_String: *const wchar_t) -> *mut wchar_t;
    pub fn wcscat(_Destination: *mut wchar_t, _Source: *const wchar_t) -> *mut wchar_t;
    pub fn wcscmp(
        _String1: *const ::std::os::raw::c_ushort,
        _String2: *const ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
    pub fn wcscpy(_Destination: *mut wchar_t, _Source: *const wchar_t) -> *mut wchar_t;
    pub fn wcscspn(_String: *const wchar_t, _Control: *const wchar_t) -> usize;
    pub fn wcslen(_String: *const ::std::os::raw::c_ushort) -> ::std::os::raw::c_ulonglong;
    pub fn wcsnlen(_Source: *const wchar_t, _MaxCount: usize) -> usize;
    pub fn wcsncat(
        _Destination: *mut wchar_t,
        _Source: *const wchar_t,
        _Count: usize,
    ) -> *mut wchar_t;
    pub fn wcsncmp(
        _String1: *const ::std::os::raw::c_ushort,
        _String2: *const ::std::os::raw::c_ushort,
        _MaxCount: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
    pub fn wcsncpy(
        _Destination: *mut wchar_t,
        _Source: *const wchar_t,
        _Count: usize,
    ) -> *mut wchar_t;
    pub fn wcspbrk(_String: *const wchar_t, _Control: *const wchar_t) -> *mut wchar_t;
    pub fn wcsspn(_String: *const wchar_t, _Control: *const wchar_t) -> usize;
    pub fn wcstok(
        _String: *mut wchar_t,
        _Delimiter: *const wchar_t,
        _Context: *mut *mut wchar_t,
    ) -> *mut wchar_t;
    pub fn _wcserror(_ErrorNumber: ::std::os::raw::c_int) -> *mut wchar_t;
    pub fn _wcserror_s(
        _Buffer: *mut wchar_t,
        _SizeInWords: usize,
        _ErrorNumber: ::std::os::raw::c_int,
    ) -> errno_t;
    pub fn __wcserror(_String: *const wchar_t) -> *mut wchar_t;
    pub fn __wcserror_s(
        _Buffer: *mut wchar_t,
        _SizeInWords: usize,
        _ErrorMessage: *const wchar_t,
    ) -> errno_t;
    pub fn _wcsicmp(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
    pub fn _wcsicmp_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn _wcsnicmp(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: usize,
    ) -> ::std::os::raw::c_int;
    pub fn _wcsnicmp_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: usize,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn _wcsnset_s(
        _Destination: *mut wchar_t,
        _SizeInWords: usize,
        _Value: wchar_t,
        _MaxCount: usize,
    ) -> errno_t;
    pub fn _wcsnset(_String: *mut wchar_t, _Value: wchar_t, _MaxCount: usize) -> *mut wchar_t;
    pub fn _wcsrev(_String: *mut wchar_t) -> *mut wchar_t;
    pub fn _wcsset_s(_Destination: *mut wchar_t, _SizeInWords: usize, _Value: wchar_t) -> errno_t;
    pub fn _wcsset(_String: *mut wchar_t, _Value: wchar_t) -> *mut wchar_t;
    pub fn _wcslwr_s(_String: *mut wchar_t, _SizeInWords: usize) -> errno_t;
    pub fn _wcslwr(_String: *mut wchar_t) -> *mut wchar_t;
    pub fn _wcslwr_s_l(_String: *mut wchar_t, _SizeInWords: usize, _Locale: _locale_t) -> errno_t;
    pub fn _wcslwr_l(_String: *mut wchar_t, _Locale: _locale_t) -> *mut wchar_t;
    pub fn _wcsupr_s(_String: *mut wchar_t, _Size: usize) -> errno_t;
    pub fn _wcsupr(_String: *mut wchar_t) -> *mut wchar_t;
    pub fn _wcsupr_s_l(_String: *mut wchar_t, _Size: usize, _Locale: _locale_t) -> errno_t;
    pub fn _wcsupr_l(_String: *mut wchar_t, _Locale: _locale_t) -> *mut wchar_t;
    pub fn wcsxfrm(_Destination: *mut wchar_t, _Source: *const wchar_t, _MaxCount: usize) -> usize;
    pub fn _wcsxfrm_l(
        _Destination: *mut wchar_t,
        _Source: *const wchar_t,
        _MaxCount: usize,
        _Locale: _locale_t,
    ) -> usize;
    pub fn wcscoll(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
    pub fn _wcscoll_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn _wcsicoll(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
    pub fn _wcsicoll_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn _wcsncoll(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: usize,
    ) -> ::std::os::raw::c_int;
    pub fn _wcsncoll_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: usize,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn _wcsnicoll(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: usize,
    ) -> ::std::os::raw::c_int;
    pub fn _wcsnicoll_l(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: usize,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn wcsdup(_String: *const wchar_t) -> *mut wchar_t;
    pub fn wcsicmp(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
    pub fn wcsnicmp(
        _String1: *const wchar_t,
        _String2: *const wchar_t,
        _MaxCount: usize,
    ) -> ::std::os::raw::c_int;
    pub fn wcsnset(_String: *mut wchar_t, _Value: wchar_t, _MaxCount: usize) -> *mut wchar_t;
    pub fn wcsrev(_String: *mut wchar_t) -> *mut wchar_t;
    pub fn wcsset(_String: *mut wchar_t, _Value: wchar_t) -> *mut wchar_t;
    pub fn wcslwr(_String: *mut wchar_t) -> *mut wchar_t;
    pub fn wcsupr(_String: *mut wchar_t) -> *mut wchar_t;
    pub fn wcsicoll(_String1: *const wchar_t, _String2: *const wchar_t) -> ::std::os::raw::c_int;
    pub fn strcpy_s(
        _Destination: *mut ::std::os::raw::c_char,
        _SizeInBytes: rsize_t,
        _Source: *const ::std::os::raw::c_char,
    ) -> errno_t;
    pub fn strcat_s(
        _Destination: *mut ::std::os::raw::c_char,
        _SizeInBytes: rsize_t,
        _Source: *const ::std::os::raw::c_char,
    ) -> errno_t;
    pub fn strerror_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _SizeInBytes: usize,
        _ErrorNumber: ::std::os::raw::c_int,
    ) -> errno_t;
    pub fn strncat_s(
        _Destination: *mut ::std::os::raw::c_char,
        _SizeInBytes: rsize_t,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: rsize_t,
    ) -> errno_t;
    pub fn strncpy_s(
        _Destination: *mut ::std::os::raw::c_char,
        _SizeInBytes: rsize_t,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: rsize_t,
    ) -> errno_t;
    pub fn strtok_s(
        _String: *mut ::std::os::raw::c_char,
        _Delimiter: *const ::std::os::raw::c_char,
        _Context: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn _memccpy(
        _Dst: *mut ::std::os::raw::c_void,
        _Src: *const ::std::os::raw::c_void,
        _Val: ::std::os::raw::c_int,
        _MaxCount: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn strcat(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcmp(
        _Str1: *const ::std::os::raw::c_char,
        _Str2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn _strcmpi(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn strcoll(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn _strcoll_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn strcpy(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcspn(
        _Str: *const ::std::os::raw::c_char,
        _Control: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulonglong;
    pub fn _strdup(_Source: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn _strerror(_ErrorMessage: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn _strerror_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _SizeInBytes: usize,
        _ErrorMessage: *const ::std::os::raw::c_char,
    ) -> errno_t;
    pub fn strerror(_ErrorMessage: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    pub fn _stricmp(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn _stricoll(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn _stricoll_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn _stricmp_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn strlen(_Str: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulonglong;
    pub fn _strlwr_s(_String: *mut ::std::os::raw::c_char, _Size: usize) -> errno_t;
    pub fn _strlwr(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn _strlwr_s_l(
        _String: *mut ::std::os::raw::c_char,
        _Size: usize,
        _Locale: _locale_t,
    ) -> errno_t;
    pub fn _strlwr_l(
        _String: *mut ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strncat(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
        _Count: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strncmp(
        _Str1: *const ::std::os::raw::c_char,
        _Str2: *const ::std::os::raw::c_char,
        _MaxCount: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
    pub fn _strnicmp(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: usize,
    ) -> ::std::os::raw::c_int;
    pub fn _strnicmp_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: usize,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn _strnicoll(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: usize,
    ) -> ::std::os::raw::c_int;
    pub fn _strnicoll_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: usize,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn _strncoll(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: usize,
    ) -> ::std::os::raw::c_int;
    pub fn _strncoll_l(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: usize,
        _Locale: _locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn __strncnt(_String: *const ::std::os::raw::c_char, _Count: usize) -> usize;
    pub fn strncpy(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
        _Count: ::std::os::raw::c_ulonglong,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strnlen(_String: *const ::std::os::raw::c_char, _MaxCount: usize) -> usize;
    pub fn _strnset_s(
        _String: *mut ::std::os::raw::c_char,
        _SizeInBytes: usize,
        _Value: ::std::os::raw::c_int,
        _MaxCount: usize,
    ) -> errno_t;
    pub fn _strnset(
        _Destination: *mut ::std::os::raw::c_char,
        _Value: ::std::os::raw::c_int,
        _Count: usize,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strpbrk(
        _Str: *const ::std::os::raw::c_char,
        _Control: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn _strrev(_Str: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn _strset_s(
        _Destination: *mut ::std::os::raw::c_char,
        _DestinationSize: usize,
        _Value: ::std::os::raw::c_int,
    ) -> errno_t;
    pub fn _strset(
        _Destination: *mut ::std::os::raw::c_char,
        _Value: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strspn(
        _Str: *const ::std::os::raw::c_char,
        _Control: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulonglong;
    pub fn strtok(
        _String: *mut ::std::os::raw::c_char,
        _Delimiter: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn _strupr_s(_String: *mut ::std::os::raw::c_char, _Size: usize) -> errno_t;
    pub fn _strupr(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn _strupr_s_l(
        _String: *mut ::std::os::raw::c_char,
        _Size: usize,
        _Locale: _locale_t,
    ) -> errno_t;
    pub fn _strupr_l(
        _String: *mut ::std::os::raw::c_char,
        _Locale: _locale_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strxfrm(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_ulonglong;
    pub fn _strxfrm_l(
        _Destination: *mut ::std::os::raw::c_char,
        _Source: *const ::std::os::raw::c_char,
        _MaxCount: usize,
        _Locale: _locale_t,
    ) -> usize;
    pub fn strdup(_String: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn strcmpi(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn stricmp(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn strlwr(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn strnicmp(
        _String1: *const ::std::os::raw::c_char,
        _String2: *const ::std::os::raw::c_char,
        _MaxCount: usize,
    ) -> ::std::os::raw::c_int;
    pub fn strnset(
        _String: *mut ::std::os::raw::c_char,
        _Value: ::std::os::raw::c_int,
        _MaxCount: usize,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strrev(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn strset(
        _String: *mut ::std::os::raw::c_char,
        _Value: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strupr(_String: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    #[doc = " S Like Memory Management"]
    pub fn R_chk_calloc(arg1: usize, arg2: usize) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_realloc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_free(arg1: *mut ::std::os::raw::c_void);
}
