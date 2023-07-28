/* automatically generated by rust-bindgen 0.66.1 */

pub const _LIBINTL_H: u32 = 1;
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
pub const LC_ALL: u32 = 0;
pub const LC_COLLATE: u32 = 1;
pub const LC_CTYPE: u32 = 2;
pub const LC_MONETARY: u32 = 3;
pub const LC_NUMERIC: u32 = 4;
pub const LC_TIME: u32 = 5;
pub const LC_MIN: u32 = 0;
pub const LC_MAX: u32 = 5;
pub const _ENABLE_PER_THREAD_LOCALE: u32 = 1;
pub const _DISABLE_PER_THREAD_LOCALE: u32 = 2;
pub const _ENABLE_PER_THREAD_LOCALE_GLOBAL: u32 = 16;
pub const _DISABLE_PER_THREAD_LOCALE_GLOBAL: u32 = 32;
pub const _ENABLE_PER_THREAD_LOCALE_NEW: u32 = 256;
pub const _DISABLE_PER_THREAD_LOCALE_NEW: u32 = 512;
pub const LC_MESSAGES: u32 = 1729;
pub const __USE_GNU_GETTEXT: u32 = 1;
pub const LIBINTL_VERSION: u32 = 4352;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize) -> !;
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type __crt_bool = bool;
extern "C" {
    pub fn _invalid_parameter_noinfo();
}
extern "C" {
    pub fn _invalid_parameter_noinfo_noreturn() -> !;
}
extern "C" {
    pub fn _invoke_watson(
        _Expression: *const wchar_t,
        _FunctionName: *const wchar_t,
        _FileName: *const wchar_t,
        _LineNo: ::std::os::raw::c_uint,
        _Reserved: usize,
    ) -> !;
}
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
pub struct lconv {
    pub decimal_point: *mut ::std::os::raw::c_char,
    pub thousands_sep: *mut ::std::os::raw::c_char,
    pub grouping: *mut ::std::os::raw::c_char,
    pub int_curr_symbol: *mut ::std::os::raw::c_char,
    pub currency_symbol: *mut ::std::os::raw::c_char,
    pub mon_decimal_point: *mut ::std::os::raw::c_char,
    pub mon_thousands_sep: *mut ::std::os::raw::c_char,
    pub mon_grouping: *mut ::std::os::raw::c_char,
    pub positive_sign: *mut ::std::os::raw::c_char,
    pub negative_sign: *mut ::std::os::raw::c_char,
    pub int_frac_digits: ::std::os::raw::c_char,
    pub frac_digits: ::std::os::raw::c_char,
    pub p_cs_precedes: ::std::os::raw::c_char,
    pub p_sep_by_space: ::std::os::raw::c_char,
    pub n_cs_precedes: ::std::os::raw::c_char,
    pub n_sep_by_space: ::std::os::raw::c_char,
    pub p_sign_posn: ::std::os::raw::c_char,
    pub n_sign_posn: ::std::os::raw::c_char,
    pub _W_decimal_point: *mut wchar_t,
    pub _W_thousands_sep: *mut wchar_t,
    pub _W_int_curr_symbol: *mut wchar_t,
    pub _W_currency_symbol: *mut wchar_t,
    pub _W_mon_decimal_point: *mut wchar_t,
    pub _W_mon_thousands_sep: *mut wchar_t,
    pub _W_positive_sign: *mut wchar_t,
    pub _W_negative_sign: *mut wchar_t,
}
#[test]
fn bindgen_test_layout_lconv() {
    const UNINIT: ::std::mem::MaybeUninit<lconv> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<lconv>(),
        152usize,
        concat!("Size of: ", stringify!(lconv))
    );
    assert_eq!(
        ::std::mem::align_of::<lconv>(),
        8usize,
        concat!("Alignment of ", stringify!(lconv))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).decimal_point) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(decimal_point)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).thousands_sep) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(thousands_sep)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).grouping) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(grouping)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).int_curr_symbol) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_curr_symbol)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).currency_symbol) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(currency_symbol)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mon_decimal_point) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(mon_decimal_point)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mon_thousands_sep) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(mon_thousands_sep)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mon_grouping) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(mon_grouping)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).positive_sign) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(positive_sign)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).negative_sign) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(negative_sign)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).int_frac_digits) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(int_frac_digits)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frac_digits) as usize - ptr as usize },
        81usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(frac_digits)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).p_cs_precedes) as usize - ptr as usize },
        82usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(p_cs_precedes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).p_sep_by_space) as usize - ptr as usize },
        83usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(p_sep_by_space)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_cs_precedes) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(n_cs_precedes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_sep_by_space) as usize - ptr as usize },
        85usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(n_sep_by_space)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).p_sign_posn) as usize - ptr as usize },
        86usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(p_sign_posn)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_sign_posn) as usize - ptr as usize },
        87usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(n_sign_posn)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._W_decimal_point) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(_W_decimal_point)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._W_thousands_sep) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(_W_thousands_sep)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._W_int_curr_symbol) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(_W_int_curr_symbol)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._W_currency_symbol) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(_W_currency_symbol)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._W_mon_decimal_point) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(_W_mon_decimal_point)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._W_mon_thousands_sep) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(_W_mon_thousands_sep)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._W_positive_sign) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(_W_positive_sign)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._W_negative_sign) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(lconv),
            "::",
            stringify!(_W_negative_sign)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    _unused: [u8; 0],
}
extern "C" {
    pub fn _lock_locales();
}
extern "C" {
    pub fn _unlock_locales();
}
extern "C" {
    pub fn _configthreadlocale(_Flag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setlocale(
        _Category: ::std::os::raw::c_int,
        _Locale: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn localeconv() -> *mut lconv;
}
extern "C" {
    pub fn _get_current_locale() -> _locale_t;
}
extern "C" {
    pub fn _create_locale(
        _Category: ::std::os::raw::c_int,
        _Locale: *const ::std::os::raw::c_char,
    ) -> _locale_t;
}
extern "C" {
    pub fn _free_locale(_Locale: _locale_t);
}
extern "C" {
    pub fn _wsetlocale(_Category: ::std::os::raw::c_int, _Locale: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wcreate_locale(_Category: ::std::os::raw::c_int, _Locale: *const wchar_t) -> _locale_t;
}
extern "C" {
    pub fn ___lc_locale_name_func() -> *mut *mut wchar_t;
}
extern "C" {
    pub fn ___lc_codepage_func() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn ___lc_collate_cp_func() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn _Getdays() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _Getmonths() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _Gettnames() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _W_Getdays() -> *mut wchar_t;
}
extern "C" {
    pub fn _W_Getmonths() -> *mut wchar_t;
}
extern "C" {
    pub fn _W_Gettnames() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn _Strftime(
        _Buffer: *mut ::std::os::raw::c_char,
        _Max_size: usize,
        _Format: *const ::std::os::raw::c_char,
        _Timeptr: *const tm,
        _Lc_time_arg: *mut ::std::os::raw::c_void,
    ) -> usize;
}
extern "C" {
    pub fn _Wcsftime(
        _Buffer: *mut wchar_t,
        _Max_size: usize,
        _Format: *const wchar_t,
        _Timeptr: *const tm,
        _Lc_time_arg: *mut ::std::os::raw::c_void,
    ) -> usize;
}
extern "C" {
    pub static mut libintl_version: ::std::os::raw::c_int;
}
extern "C" {
    pub fn libintl_gettext(__msgid: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn libintl_dgettext(
        __domainname: *const ::std::os::raw::c_char,
        __msgid: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn libintl_dcgettext(
        __domainname: *const ::std::os::raw::c_char,
        __msgid: *const ::std::os::raw::c_char,
        __category: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn libintl_ngettext(
        __msgid1: *const ::std::os::raw::c_char,
        __msgid2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn libintl_dngettext(
        __domainname: *const ::std::os::raw::c_char,
        __msgid1: *const ::std::os::raw::c_char,
        __msgid2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn libintl_dcngettext(
        __domainname: *const ::std::os::raw::c_char,
        __msgid1: *const ::std::os::raw::c_char,
        __msgid2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
        __category: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn libintl_textdomain(
        __domainname: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn libintl_bindtextdomain(
        __domainname: *const ::std::os::raw::c_char,
        __dirname: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn libintl_bind_textdomain_codeset(
        __domainname: *const ::std::os::raw::c_char,
        __codeset: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn libintl_set_relocation_prefix(
        orig_prefix: *const ::std::os::raw::c_char,
        curr_prefix: *const ::std::os::raw::c_char,
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
