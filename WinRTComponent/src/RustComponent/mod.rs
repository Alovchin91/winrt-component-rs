#![allow(non_snake_case)]
#[repr(transparent)]
pub struct ISample(::windows::core::IUnknown);
impl ISample {
    pub fn Greeting(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetGreeting<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PrintGreeting(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
impl ::core::convert::From<ISample> for ::windows::core::IInspectable {
    fn from(value: ISample) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISample> for ::windows::core::IInspectable {
    fn from(value: &ISample) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISample {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISample {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISample> for ::windows::core::IUnknown {
    fn from(value: ISample) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISample> for ::windows::core::IUnknown {
    fn from(value: &ISample) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISample {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISample {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISample {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISample {}
unsafe impl ::windows::core::RuntimeType for ISample {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{b744179f-d8fb-5e3a-8dfd-4d1b5d7c4351}");
}
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
