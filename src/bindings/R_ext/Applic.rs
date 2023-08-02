/* automatically generated by rust-bindgen 0.66.1 */

pub type wchar_t = ::std::os::raw::c_int;
#[doc = "../../appl/integrate.c"]
pub type integr_fn = ::std::option::Option<
    unsafe extern "C" fn(x: *mut f64, n: ::std::os::raw::c_int, ex: *mut ::std::os::raw::c_void),
>;
#[doc = "main/optim.c"]
pub type optimfn = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut f64,
        arg3: *mut ::std::os::raw::c_void,
    ) -> f64,
>;
pub type optimgr = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut ::std::os::raw::c_void,
    ),
>;
#[doc = "type of pointer to the target and gradient functions"]
pub type fcn_p = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut ::std::os::raw::c_void,
    ),
>;
#[doc = "type of pointer to the hessian functions"]
pub type d2fcn_p = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
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
    #[doc = "vectorizing function   f(x`[1:n]`, ...) -> x`[]`  {overwriting x`[]`}."]
    pub fn Rdqags(
        f: integr_fn,
        ex: *mut ::std::os::raw::c_void,
        a: *mut f64,
        b: *mut f64,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut ::std::os::raw::c_int,
        ier: *mut ::std::os::raw::c_int,
        limit: *mut ::std::os::raw::c_int,
        lenw: *mut ::std::os::raw::c_int,
        last: *mut ::std::os::raw::c_int,
        iwork: *mut ::std::os::raw::c_int,
        work: *mut f64,
    );
    pub fn Rdqagi(
        f: integr_fn,
        ex: *mut ::std::os::raw::c_void,
        bound: *mut f64,
        inf: *mut ::std::os::raw::c_int,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut ::std::os::raw::c_int,
        ier: *mut ::std::os::raw::c_int,
        limit: *mut ::std::os::raw::c_int,
        lenw: *mut ::std::os::raw::c_int,
        last: *mut ::std::os::raw::c_int,
        iwork: *mut ::std::os::raw::c_int,
        work: *mut f64,
    );
    pub fn vmmin(
        n: ::std::os::raw::c_int,
        b: *mut f64,
        Fmin: *mut f64,
        fn_: optimfn,
        gr: optimgr,
        maxit: ::std::os::raw::c_int,
        trace: ::std::os::raw::c_int,
        mask: *mut ::std::os::raw::c_int,
        abstol: f64,
        reltol: f64,
        nREPORT: ::std::os::raw::c_int,
        ex: *mut ::std::os::raw::c_void,
        fncount: *mut ::std::os::raw::c_int,
        grcount: *mut ::std::os::raw::c_int,
        fail: *mut ::std::os::raw::c_int,
    );
    pub fn nmmin(
        n: ::std::os::raw::c_int,
        Bvec: *mut f64,
        X: *mut f64,
        Fmin: *mut f64,
        fn_: optimfn,
        fail: *mut ::std::os::raw::c_int,
        abstol: f64,
        intol: f64,
        ex: *mut ::std::os::raw::c_void,
        alpha: f64,
        bet: f64,
        gamm: f64,
        trace: ::std::os::raw::c_int,
        fncount: *mut ::std::os::raw::c_int,
        maxit: ::std::os::raw::c_int,
    );
    pub fn cgmin(
        n: ::std::os::raw::c_int,
        Bvec: *mut f64,
        X: *mut f64,
        Fmin: *mut f64,
        fn_: optimfn,
        gr: optimgr,
        fail: *mut ::std::os::raw::c_int,
        abstol: f64,
        intol: f64,
        ex: *mut ::std::os::raw::c_void,
        type_: ::std::os::raw::c_int,
        trace: ::std::os::raw::c_int,
        fncount: *mut ::std::os::raw::c_int,
        grcount: *mut ::std::os::raw::c_int,
        maxit: ::std::os::raw::c_int,
    );
    pub fn lbfgsb(
        n: ::std::os::raw::c_int,
        m: ::std::os::raw::c_int,
        x: *mut f64,
        l: *mut f64,
        u: *mut f64,
        nbd: *mut ::std::os::raw::c_int,
        Fmin: *mut f64,
        fn_: optimfn,
        gr: optimgr,
        fail: *mut ::std::os::raw::c_int,
        ex: *mut ::std::os::raw::c_void,
        factr: f64,
        pgtol: f64,
        fncount: *mut ::std::os::raw::c_int,
        grcount: *mut ::std::os::raw::c_int,
        maxit: ::std::os::raw::c_int,
        msg: *mut ::std::os::raw::c_char,
        trace: ::std::os::raw::c_int,
        nREPORT: ::std::os::raw::c_int,
    );
    pub fn samin(
        n: ::std::os::raw::c_int,
        pb: *mut f64,
        yb: *mut f64,
        fn_: optimfn,
        maxit: ::std::os::raw::c_int,
        tmax: ::std::os::raw::c_int,
        ti: f64,
        trace: ::std::os::raw::c_int,
        ex: *mut ::std::os::raw::c_void,
    );
    #[doc = "appl/interv.c: Also in Utils.h, used in package eco"]
    pub fn findInterval(
        xt: *mut f64,
        n: ::std::os::raw::c_int,
        x: f64,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        ilo: ::std::os::raw::c_int,
        mflag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn dqrqty_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        qty: *mut f64,
    );
    pub fn dqrqy_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        qy: *mut f64,
    );
    pub fn dqrcf_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        b: *mut f64,
        info: *mut ::std::os::raw::c_int,
    );
    pub fn dqrrsd_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        rsd: *mut f64,
    );
    pub fn dqrxb_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        xb: *mut f64,
    );
    #[doc = "appl/pretty.c: for use in engine.c and util.c"]
    pub fn R_pretty(
        lo: *mut f64,
        up: *mut f64,
        ndiv: *mut ::std::os::raw::c_int,
        min_n: ::std::os::raw::c_int,
        shrink_sml: f64,
        high_u_fact: *const f64,
        eps_correction: ::std::os::raw::c_int,
        return_bounds: ::std::os::raw::c_int,
    ) -> f64;
    pub fn fdhess(
        n: ::std::os::raw::c_int,
        x: *mut f64,
        fval: f64,
        fun: fcn_p,
        state: *mut ::std::os::raw::c_void,
        h: *mut f64,
        nfd: ::std::os::raw::c_int,
        step: *mut f64,
        f: *mut f64,
        ndigit: ::std::os::raw::c_int,
        typx: *mut f64,
    );
    #[doc = "Also used in packages nlme, pcaPP"]
    pub fn optif9(
        nr: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        x: *mut f64,
        fcn: fcn_p,
        d1fcn: fcn_p,
        d2fcn: d2fcn_p,
        state: *mut ::std::os::raw::c_void,
        typsiz: *mut f64,
        fscale: f64,
        method: ::std::os::raw::c_int,
        iexp: ::std::os::raw::c_int,
        msg: *mut ::std::os::raw::c_int,
        ndigit: ::std::os::raw::c_int,
        itnlim: ::std::os::raw::c_int,
        iagflg: ::std::os::raw::c_int,
        iahflg: ::std::os::raw::c_int,
        dlt: f64,
        gradtl: f64,
        stepmx: f64,
        steptl: f64,
        xpls: *mut f64,
        fpls: *mut f64,
        gpls: *mut f64,
        itrmcd: *mut ::std::os::raw::c_int,
        a: *mut f64,
        wrk: *mut f64,
        itncnt: *mut ::std::os::raw::c_int,
    );
    pub fn dqrdc2_(
        x: *mut f64,
        ldx: *mut ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        p: *mut ::std::os::raw::c_int,
        tol: *mut f64,
        rank: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        pivot: *mut ::std::os::raw::c_int,
        work: *mut f64,
    );
    pub fn dqrls_(
        x: *mut f64,
        n: *mut ::std::os::raw::c_int,
        p: *mut ::std::os::raw::c_int,
        y: *mut f64,
        ny: *mut ::std::os::raw::c_int,
        tol: *mut f64,
        b: *mut f64,
        rsd: *mut f64,
        qty: *mut f64,
        k: *mut ::std::os::raw::c_int,
        jpvt: *mut ::std::os::raw::c_int,
        qraux: *mut f64,
        work: *mut f64,
    );
}
