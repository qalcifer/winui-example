fn main() {
    windows::build! {
        Windows::Win32::System::Com::CoInitializeEx,
        Windows::UI::Xaml::Controls::{NavigationView},
        Windows::UI::Xaml::{Application, Window},
    };
}