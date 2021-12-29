#![allow(non_snake_case)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ISample(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISample {
    type Vtable = ISampleVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb744179f_d8fb_5e3a_8dfd_4d1b5d7c4351);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISampleVtbl(
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iid: &::windows::core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISampleFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISampleFactory {
    type Vtable = ISampleFactoryVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeff2666b_c949_5b99_8705_7ebf5a0bcac8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISampleFactoryVtbl(
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iid: &::windows::core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        greeting: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
