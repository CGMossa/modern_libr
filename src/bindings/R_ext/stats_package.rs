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
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AlgType {
    NREG = 1,
    OPT = 2,
}
#[repr(u32)]
#[doc = "0-based indices into v"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VPos {
    F = 9,
    F0 = 12,
    FDIF = 10,
    G = 27,
    HC = 70,
}
#[repr(u32)]
#[doc = "0-based indices into iv"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IVPos {
    AI = 90,
    AM = 94,
    ALGSAV = 50,
    COVMAT = 25,
    COVPRT = 13,
    COVREQ = 14,
    DRADPR = 100,
    DTYPE = 15,
    IERR = 74,
    INITH = 24,
    IPIVOT = 75,
    IVNEED = 2,
    LASTIV = 42,
    LASTV = 44,
    LMAT = 41,
    MXFCAL = 16,
    MXITER = 17,
    NEXTV = 46,
    NFCALL = 5,
    NFCOV = 51,
    NFGCAL = 6,
    NGCOV = 52,
    NITER = 30,
    NVDFLT = 49,
    NVSAVE = 8,
    OUTLEV = 18,
    PARPRT = 19,
    PARSAV = 48,
    PERM = 57,
    PRUNIT = 20,
    QRTYP = 79,
    RDREQ = 56,
    RMAT = 77,
    SOLPRT = 21,
    STATPR = 22,
    TOOBIG = 1,
    VNEED = 3,
    VSAVE = 59,
    X0PRT = 23,
}
impl IVPos {
    pub const INITS: IVPos = IVPos::INITH;
}
