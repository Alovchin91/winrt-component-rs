use crate::component as RustComponent;

use windows as Windows;
use windows::core::{implement, IInspectable, HSTRING};

#[implement(RustComponent::ISample)]
struct Sample {
    greeting: HSTRING,
}

impl Sample {
    pub fn new() -> Self {
        Self {
            greeting: "Hello, world!".into(),
        }
    }

    pub fn new_with_greeting(greeting: &HSTRING) -> Self {
        Self {
            greeting: greeting.to_owned(),
        }
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

pub(crate) fn winrt_get_activation_factory(
    name: &HSTRING,
) -> core::option::Option<Windows::Win32::System::WinRT::IActivationFactory> {
    match name {
        id if id == "RustComponent.Sample" => Some(SampleFactory.into()),
        _ => None,
    }
}
