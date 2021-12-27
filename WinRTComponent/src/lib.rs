#![cfg(target_os = "windows")]
#![allow(non_snake_case)]

mod component;

use std::mem::ManuallyDrop;

use component as RustComponent;

use windows::core::{implement, HRESULT, HSTRING, IInspectable};
use windows as Windows;

#[implement(RustComponent::ISample)]
struct Sample;

impl Sample {
    pub fn Greeting(&self) -> Result<HSTRING, ::windows::core::Error> {
        Ok(HSTRING::from("Hello, world!"))
    }

    pub fn SetGreeting(&self, value: &HSTRING) -> Result<(), ::windows::core::Error> {
        todo!()
    }

    pub fn PrintGreeting(&self) -> Result<(), ::windows::core::Error> {
        todo!()
    }
}

#[implement(Windows::Win32::System::WinRT::IActivationFactory)]
struct SampleFactory;

impl SampleFactory {
    pub unsafe fn ActivateInstance(&self) -> ::windows::core::Result<IInspectable> {
        Ok(Sample.into())
    }
}

#[allow(overflowing_literals)]
mod consts {
    use windows::core::HRESULT;

    pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = HRESULT(0x80040111);
    pub const S_OK: HRESULT = HRESULT(0);
}
use consts::*;

#[no_mangle]
pub unsafe extern "stdcall" fn DllCanUnloadNow() -> i32 {
    0 // FALSE
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllGetActivationFactory(
    class_id: ManuallyDrop<HSTRING>,
    factory: *mut Option<Windows::Win32::System::WinRT::IActivationFactory>
) -> HRESULT {
    match &*class_id {
        id if id == "RustComponent.Sample" => {
            *factory = Some(SampleFactory.into());
            S_OK
        },
        _ => {
            *factory = None;
            CLASS_E_CLASSNOTAVAILABLE
        }
    }
}
