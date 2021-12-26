#![cfg(target_os = "windows")]

pub mod RustComponent;

use windows::core::{implement, HSTRING, IInspectable};
use windows as Windows;

#[implement(RustComponent::ISample)]
struct Sample;

#[allow(non_snake_case)]
impl Sample {
    pub fn Greeting(&self) -> Result<String, ::windows::core::Error> {
        todo!()
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

#[allow(non_snake_case)]
impl SampleFactory {
    pub unsafe fn ActivateInstance(&self) -> ::windows::core::Result<IInspectable> {
        todo!()
    }
}
