#![cfg(target_os = "windows")]
#![allow(non_snake_case)]

mod component;
use component as RustComponent;

use windows::core::{implement, HRESULT, HSTRING, IInspectable};
use windows::Win32::Foundation::{CLASS_E_CLASSNOTAVAILABLE, S_OK, S_FALSE};
use windows as Windows;

#[implement(RustComponent::ISample)]
struct Sample {
    greeting: HSTRING
}

impl Sample {
    pub fn new() -> Self {
        Self { greeting: "Hello, world!".into() }
    }

    pub fn new_with_greeting(greeting: &HSTRING) -> Self {
        Self { greeting: greeting.to_owned() }
    }

    pub fn Greeting(&self) -> windows::core::Result<HSTRING> {
        Ok(self.greeting.clone())
    }

    pub fn SetGreeting(&mut self, value: &HSTRING) -> windows::core::Result<()> {
        self.greeting = value.to_owned();
        Ok(())
    }

    pub fn PrintGreeting(&self) -> windows::core::Result<()> {
        println!("{}", self.greeting);
        Ok(())
    }
}

#[implement(
    Windows::Win32::System::WinRT::IActivationFactory,
    RustComponent::ISampleFactory
)]
struct SampleFactory;

impl SampleFactory {
    pub fn ActivateInstance(&self) -> windows::core::Result<IInspectable> {
        Ok(Sample::new().into())
    }

    pub fn CreateInstance(&self, greeting: &HSTRING) -> windows::core::Result<IInspectable> {
        Ok(Sample::new_with_greeting(greeting).into())
    }
}

#[no_mangle]
pub extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    S_FALSE
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllGetActivationFactory(
    class_id: std::mem::ManuallyDrop<HSTRING>,
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
