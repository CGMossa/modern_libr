/* automatically generated by rust-bindgen 0.66.1 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const RSTART_VERSION: u32 = 1;
pub type max_align_t = f64;
pub const Rboolean_FALSE: Rboolean = 0;
pub const Rboolean_TRUE: Rboolean = 1;
pub type Rboolean = ::std::os::raw::c_int;
pub const UImode_RGui: UImode = 0;
pub const UImode_RTerm: UImode = 1;
pub const UImode_LinkDLL: UImode = 2;
pub type UImode = ::std::os::raw::c_int;
pub const SA_TYPE_SA_NORESTORE: SA_TYPE = 0;
pub const SA_TYPE_SA_RESTORE: SA_TYPE = 1;
pub const SA_TYPE_SA_DEFAULT: SA_TYPE = 2;
pub const SA_TYPE_SA_NOSAVE: SA_TYPE = 3;
pub const SA_TYPE_SA_SAVE: SA_TYPE = 4;
pub const SA_TYPE_SA_SAVEASK: SA_TYPE = 5;
pub const SA_TYPE_SA_SUICIDE: SA_TYPE = 6;
pub type SA_TYPE = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct structRstart {
    pub R_Quiet: Rboolean,
    pub R_NoEcho: Rboolean,
    pub R_Interactive: Rboolean,
    pub R_Verbose: Rboolean,
    pub LoadSiteFile: Rboolean,
    pub LoadInitFile: Rboolean,
    pub DebugInitFile: Rboolean,
    pub RestoreAction: SA_TYPE,
    pub SaveAction: SA_TYPE,
    pub vsize: usize,
    pub nsize: usize,
    pub max_vsize: usize,
    pub max_nsize: usize,
    pub ppsize: usize,
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub rhome: *mut ::std::os::raw::c_char,
    pub home: *mut ::std::os::raw::c_char,
    pub ReadConsole: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::std::os::raw::c_uchar,
            arg3: ::std::os::raw::c_int,
            arg4: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub WriteConsole: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char, arg2: ::std::os::raw::c_int),
    >,
    pub CallBack: ::std::option::Option<unsafe extern "C" fn()>,
    pub ShowMessage:
        ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>,
    pub YesNoCancel: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub Busy: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub CharacterMode: UImode,
    pub WriteConsoleEx: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub EmitEmbeddedUTF8: Rboolean,
    pub CleanUp: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: SA_TYPE,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub ClearerrConsole: ::std::option::Option<unsafe extern "C" fn()>,
    pub FlushConsole: ::std::option::Option<unsafe extern "C" fn()>,
    pub ResetConsole: ::std::option::Option<unsafe extern "C" fn()>,
    pub Suicide: ::std::option::Option<unsafe extern "C" fn(s: *const ::std::os::raw::c_char)>,
}
#[test]
fn bindgen_test_layout_structRstart() {
    const UNINIT: ::std::mem::MaybeUninit<structRstart> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<structRstart>(),
        216usize,
        concat!("Size of: ", stringify!(structRstart))
    );
    assert_eq!(
        ::std::mem::align_of::<structRstart>(),
        8usize,
        concat!("Alignment of ", stringify!(structRstart))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).R_Quiet) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(R_Quiet)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).R_NoEcho) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(R_NoEcho)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).R_Interactive) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(R_Interactive)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).R_Verbose) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(R_Verbose)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LoadSiteFile) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(LoadSiteFile)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LoadInitFile) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(LoadInitFile)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DebugInitFile) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(DebugInitFile)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RestoreAction) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(RestoreAction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SaveAction) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(SaveAction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vsize) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(vsize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nsize) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(nsize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_vsize) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(max_vsize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_nsize) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(max_nsize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ppsize) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(ppsize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rhome) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(rhome)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).home) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(home)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ReadConsole) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(ReadConsole)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).WriteConsole) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(WriteConsole)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CallBack) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(CallBack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ShowMessage) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(ShowMessage)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).YesNoCancel) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(YesNoCancel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Busy) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(Busy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CharacterMode) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(CharacterMode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).WriteConsoleEx) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(WriteConsoleEx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).EmitEmbeddedUTF8) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(EmitEmbeddedUTF8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CleanUp) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(CleanUp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ClearerrConsole) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(ClearerrConsole)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).FlushConsole) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(FlushConsole)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ResetConsole) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(ResetConsole)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Suicide) as usize - ptr as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(structRstart),
            "::",
            stringify!(Suicide)
        )
    );
}
impl structRstart {
    #[inline]
    pub fn NoRenviron(&self) -> Rboolean {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_NoRenviron(&mut self, val: Rboolean) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn RstartVersion(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_RstartVersion(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        NoRenviron: Rboolean,
        RstartVersion: ::std::os::raw::c_int,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 16u8, {
            let NoRenviron: u32 = unsafe { ::std::mem::transmute(NoRenviron) };
            NoRenviron as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let RstartVersion: u32 = unsafe { ::std::mem::transmute(RstartVersion) };
            RstartVersion as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type Rstart = *mut structRstart;
extern "C" {
    pub fn R_DefParams(arg1: Rstart);
    pub fn R_DefParamsEx(arg1: Rstart, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn R_SetParams(arg1: Rstart);
    pub fn R_DefCallbacks(arg1: Rstart, arg2: ::std::os::raw::c_int);
    pub fn R_SetWin32(arg1: Rstart);
    pub fn R_SizeFromEnv(arg1: Rstart);
    pub fn R_common_command_line(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *mut *mut ::std::os::raw::c_char,
        arg3: Rstart,
    );
    pub fn R_set_command_line_arguments(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    );
    pub fn setup_Rmainloop();
}
