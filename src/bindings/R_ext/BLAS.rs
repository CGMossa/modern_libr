/* automatically generated by rust-bindgen 0.66.1 */

pub type wchar_t = ::std::os::raw::c_int;
pub const _STRING_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 35;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 1;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 1;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
extern "C" {
    pub fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memmove(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memccpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memset(
        __s: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
    pub fn __memcmpeq(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
    pub fn memchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
    pub fn strcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strncat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn strxfrm(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
    pub fn strdup(__s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn strndup(
        __string: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strrchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __reject: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
    pub fn strpbrk(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strstr(
        __haystack: *const ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strtok(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn __strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    pub fn dasum_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn daxpy_(
        n: *const ::std::os::raw::c_int,
        da: *const f64,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn dcopy_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn ddot_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
        dy: *const f64,
        incy: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn dnrm2_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn drot_(
        n: *const ::std::os::raw::c_int,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
        c: *const f64,
        s: *const f64,
    );
    pub fn drotg_(a: *const f64, b: *const f64, c: *mut f64, s: *mut f64);
    pub fn drotm_(
        n: *const ::std::os::raw::c_int,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
        dparam: *const f64,
    );
    pub fn drotmg_(
        dd1: *const f64,
        dd2: *const f64,
        dx1: *const f64,
        dy1: *const f64,
        param: *mut f64,
    );
    pub fn dscal_(
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
    );
    pub fn dswap_(
        n: *const ::std::os::raw::c_int,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn idamax_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn dgbmv_(
        trans: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        kl: *const ::std::os::raw::c_int,
        ku: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dgemv_(
        trans: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dsbmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dspmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        ap: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dsymv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dtbmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtpmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const f64,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtrmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtbsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtpsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const f64,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtrsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dger_(
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *const f64,
        incy: *const ::std::os::raw::c_int,
        a: *mut f64,
        lda: *const ::std::os::raw::c_int,
    );
    pub fn dsyr_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        a: *mut f64,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dspr_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        ap: *mut f64,
        arg1: usize,
    );
    pub fn dsyr2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *const f64,
        incy: *const ::std::os::raw::c_int,
        a: *mut f64,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dspr2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *const f64,
        incy: *const ::std::os::raw::c_int,
        ap: *mut f64,
        arg1: usize,
    );
    pub fn dgemm_(
        transa: *const ::std::os::raw::c_char,
        transb: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dtrsm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *mut f64,
        ldb: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn dtrmm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *mut f64,
        ldb: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn dsymm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dsyrk_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dsyr2k_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dcabs1_(z: *const Rcomplex) -> f64;
    pub fn dzasum_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn dznrm2_(
        n: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn izamax_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn zaxpy_(
        n: *const ::std::os::raw::c_int,
        za: *const Rcomplex,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn zcopy_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn zdotc_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    ) -> Rcomplex;
    pub fn zdotu_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    ) -> Rcomplex;
    pub fn zdrot_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        c: *const f64,
        s: *const f64,
    );
    pub fn zdscal_(
        n: *const ::std::os::raw::c_int,
        da: *const f64,
        zx: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
    );
    pub fn zgbmv_(
        trans: *const ::std::os::raw::c_char,
        m: *mut ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        kl: *mut ::std::os::raw::c_int,
        ku: *mut ::std::os::raw::c_int,
        alpha: *mut Rcomplex,
        a: *mut Rcomplex,
        lda: *mut ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *mut ::std::os::raw::c_int,
        beta: *mut Rcomplex,
        y: *mut Rcomplex,
        incy: *mut ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zgemm_(
        transa: *const ::std::os::raw::c_char,
        transb: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zgemv_(
        trans: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zgerc_(
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
    );
    pub fn zgeru_(
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
    );
    pub fn zhbmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zhemm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zhemv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zher_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zher2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zher2k_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zherk_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zhpmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        ap: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zhpr_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        ap: *mut Rcomplex,
        arg1: usize,
    );
    pub fn zhpr2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        ap: *mut Rcomplex,
        arg1: usize,
    );
    pub fn zrotg_(ca: *const Rcomplex, cb: *const Rcomplex, c: *mut f64, s: *mut Rcomplex);
    pub fn zscal_(
        n: *const ::std::os::raw::c_int,
        za: *const Rcomplex,
        zx: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
    );
    pub fn zswap_(
        n: *const ::std::os::raw::c_int,
        zx: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn zsymm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zsyr2k_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        alpha: *mut Rcomplex,
        a: *mut Rcomplex,
        lda: *mut ::std::os::raw::c_int,
        b: *mut Rcomplex,
        ldb: *mut ::std::os::raw::c_int,
        beta: *mut Rcomplex,
        c: *mut Rcomplex,
        ldc: *mut ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zsyrk_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn ztbmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztbsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztpmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const Rcomplex,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztpsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const Rcomplex,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztrmm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *mut Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn ztrmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztrsm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *mut ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        alpha: *mut Rcomplex,
        a: *mut Rcomplex,
        lda: *mut ::std::os::raw::c_int,
        b: *mut Rcomplex,
        ldb: *mut ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn ztrsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
}
