#![windows_subsystem = "windows"]

use bindings::*;
use windows::*;

use bindings::{
    Windows::ApplicationModel::Activation::LaunchActivatedEventArgs,
    Windows::Win32::System::Com::*,
    Windows::UI::Xaml::{Application, ApplicationInitializationCallback, Window},
    Windows::UI::Xaml::Controls::{NavigationView},
};

#[implement(
    extend Windows::UI::Xaml::Application,
    override OnLaunched
)]
struct App();

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        let window = Window::Current()?;

        let navigation_view = NavigationView::new()?;

        window.SetContent(navigation_view)?;

        window.Activate()
    }
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    }
    Application::Start(ApplicationInitializationCallback::new(|_| {
        App().new()?;
        Ok(())
    }))
}
