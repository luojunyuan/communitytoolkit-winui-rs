use windows::core::HSTRING;
use xamltoolkit_winui::{HslColor, HsvColor};
use xamltoolkit_winui_helpers::{
    CameraHelper, CameraHelperResult, ColorHelper, DesignTimeHelpers, FrameEventArgs,
    ThemeChangedHandler, ThemeListener,
};

fn main() {
    run_smoke();
    eprintln!("helpers-example: smoke OK");
}

fn run_smoke() {
    let _ = [
        CameraHelperResult::Success,
        CameraHelperResult::CreateFrameReaderFailed,
        CameraHelperResult::StartFrameReaderFailed,
        CameraHelperResult::NoFrameSourceGroupAvailable,
        CameraHelperResult::NoFrameSourceAvailable,
        CameraHelperResult::CameraAccessDenied,
        CameraHelperResult::InitializationFailed_UnknownError,
        CameraHelperResult::NoCompatibleFrameFormatAvailable,
    ];
    let _ = HslColor {
        H: 210.0,
        S: 0.4,
        L: 0.5,
        A: 1.0,
    };
    let _ = HsvColor {
        H: 210.0,
        S: 0.5,
        V: 0.75,
        A: 1.0,
    };
    let _ = HSTRING::from("#336699");

    type_seen::<Option<CameraHelper>>();
    type_seen::<Option<ColorHelper>>();
    type_seen::<Option<DesignTimeHelpers>>();
    type_seen::<Option<FrameEventArgs>>();
    type_seen::<Option<ThemeChangedHandler>>();
    type_seen::<Option<ThemeListener>>();
}

fn type_seen<T>() {
    let _ = std::mem::size_of::<T>();
}
