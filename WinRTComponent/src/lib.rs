#![cfg(target_os = "windows")]
#![allow(non_snake_case)]

mod component;
mod implementation;

use windows as Windows;
use Windows::Win32::Foundation::{CLASS_E_CLASSNOTAVAILABLE, S_FALSE, S_OK};

#[no_mangle]
pub extern "stdcall" fn DllCanUnloadNow() -> windows::core::HRESULT {
    S_FALSE
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllGetActivationFactory(
    class_id: core::mem::ManuallyDrop<windows::core::HSTRING>,
    factory: *mut *mut Windows::Win32::System::WinRT::IActivationFactory,
) -> windows::core::HRESULT {
    if let Some(f) = implementation::winrt_get_activation_factory(&*class_id) {
        *factory = core::mem::transmute(f);
        S_OK
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    }
}
