/* automatically generated by rust-bindgen 0.66.1 */

pub type max_align_t = f64;
pub type R_allocator_t = R_allocator;
pub type custom_alloc_t = ::std::option::Option<
    unsafe extern "C" fn(allocator: *mut R_allocator_t, arg1: usize) -> *mut ::std::os::raw::c_void,
>;
pub type custom_free_t = ::std::option::Option<
    unsafe extern "C" fn(allocator: *mut R_allocator_t, arg1: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_allocator {
    #[doc = " malloc equivalent"]
    pub mem_alloc: custom_alloc_t,
    #[doc = " free equivalent"]
    pub mem_free: custom_free_t,
    #[doc = " reserved (maybe for copy) - must be NULL"]
    pub res: *mut ::std::os::raw::c_void,
    #[doc = " custom data for the allocator implementation"]
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_R_allocator() {
    const UNINIT: ::std::mem::MaybeUninit<R_allocator> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<R_allocator>(),
        32usize,
        concat!("Size of: ", stringify!(R_allocator))
    );
    assert_eq!(
        ::std::mem::align_of::<R_allocator>(),
        8usize,
        concat!("Alignment of ", stringify!(R_allocator))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mem_alloc) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(R_allocator),
            "::",
            stringify!(mem_alloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mem_free) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(R_allocator),
            "::",
            stringify!(mem_free)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).res) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(R_allocator),
            "::",
            stringify!(res)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(R_allocator),
            "::",
            stringify!(data)
        )
    );
}
