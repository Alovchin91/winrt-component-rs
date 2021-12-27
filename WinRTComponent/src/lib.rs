#![cfg(target_os = "windows")]

mod component;

use std::ffi::c_void;
use std::mem::ManuallyDrop;

use component as RustComponent;

use windows::core::{implement, HRESULT, HSTRING, IInspectable};
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
        let sample = Sample;
        sample.cast()
    }
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllCanUnloadNow() -> i32 {
    0 // FALSE
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllGetActivationFactory(class_id: ManuallyDrop<HSTRING>, factory: *mut *mut c_void) -> HRESULT {
    if *class_id != "RustComponent.Sample" {
        return HRESULT(-2147221231i32); // CLASS_E_CLASSNOTAVAILABLE
    }

    let sample_factory = SampleFactory;

    if let Ok(val) = sample_factory.cast::<Windows::Win32::System::WinRT::IActivationFactory>() {
        let boxed_val = Box::new(val);
        *factory = Box::into_raw(boxed_val) as *mut c_void;

        HRESULT(0)
    } else {
        HRESULT(-2147221231i32) // CLASS_E_CLASSNOTAVAILABLE
    }
}
