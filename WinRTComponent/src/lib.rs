#![cfg(target_os = "windows")]
#![allow(non_snake_case)]

mod component;

use std::mem::ManuallyDrop;

use component as RustComponent;

use windows::core::{implement, HRESULT, HSTRING, IInspectable};
use windows as Windows;

#[allow(overflowing_literals)]
mod consts {
    use windows::core::HRESULT;

    pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = HRESULT(0x80040111);
    pub const E_INVALIDARG: HRESULT = HRESULT(0x80070057);
    pub const S_OK: HRESULT = HRESULT(0);
}
use consts::*;

#[implement(RustComponent::ISample)]
struct Sample {
    greeting: String
}

impl Sample {
    pub fn new() -> Self {
        Self { greeting: "Hello, world!".to_string() }
    }

    pub fn Greeting(&self) -> Result<HSTRING, ::windows::core::Error> {
        Ok(HSTRING::from(&self.greeting))
    }

    pub fn SetGreeting(&mut self, value: &HSTRING) -> Result<(), ::windows::core::Error> {
        self.greeting = String::from_utf16(value.as_wide())
            .or_else(|err|
                Err(windows::core::Error::new(E_INVALIDARG, err.to_string().into()))
            )?;
        Ok(())
    }

    pub fn PrintGreeting(&self) -> Result<(), ::windows::core::Error> {
        println!("{}", self.greeting);
        Ok(())
    }
}

#[implement(Windows::Win32::System::WinRT::IActivationFactory)]
struct SampleFactory;

impl SampleFactory {
    pub unsafe fn ActivateInstance(&self) -> ::windows::core::Result<IInspectable> {
        Ok(Sample::new().into())
    }
}

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
