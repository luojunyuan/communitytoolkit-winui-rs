use windows::core::Result;
use windows_reactor::{App, Element, RenderCx, text_block};
use xamltoolkit_winui_helpers::XamlToolkit::WinUI::Helpers::{
    DesignTimeHelpers, ThemeChangedHandler, ThemeListener,
};

fn main() {
    if let Err(error) = App::new()
        .title("XamlToolkit WinUI Helpers smoke")
        .render(app)
    {
        eprintln!("helpers-example failed: {error:?}");
        std::process::exit(1);
    }
}

fn app(_cx: &mut RenderCx) -> Element {
    match run_smoke() {
        Ok(()) => {
            eprintln!("helpers-example: smoke OK");
            if std::env::var_os("XAMLTOOLKIT_HELPERS_SMOKE_EXIT").is_some() {
                std::process::exit(0);
            }
        }
        Err(error) => {
            eprintln!("helpers-example: smoke failed: {error:?}");
            std::process::exit(1);
        }
    }

    text_block("XamlToolkit.WinUI.Helpers smoke").into()
}

fn run_smoke() -> Result<()> {
    let _ = DesignTimeHelpers::IsRunningInLegacyDesignerMode()?;
    let _ = DesignTimeHelpers::IsRunningInEnhancedDesignerMode()?;
    let _ = DesignTimeHelpers::IsRunningInApplicationRuntimeMode()?;

    let listener = ThemeListener::new()?;
    let _ = listener.CurrentTheme()?;
    let _ = listener.CurrentThemeName()?;
    let _ = listener.IsHighContrast()?;

    let handler = ThemeChangedHandler::new(|_| Ok(()));
    let token = listener.ThemeChanged(&handler)?;
    listener.RemoveThemeChanged(token)?;

    Ok(())
}
