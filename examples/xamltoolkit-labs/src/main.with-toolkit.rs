use windows_reactor::*;
use xamltoolkit_winui_controls::XamlToolkit::WinUI::Controls::{StretchChild, WrapPanel};

fn main() {
    if let Err(error) = App::new().title("XamlToolkit Labs").render(app) {
        eprintln!("xamltoolkit-labs failed: {error:?}");
        std::process::exit(1);
    }
}

fn app(_cx: &mut RenderCx) -> Element {
    let projection_summary = format!(
        "Controls projection: WrapPanel, StretchChild::{:?}",
        StretchChild::Last,
    );

    let control_status = match WrapPanel::new() {
        Ok(panel) => {
            let spacing = panel.SetHorizontalSpacing(12.0);
            let stretch = panel.SetStretchChild(StretchChild::Last);
            format!(
                "WrapPanel activation: OK; SetHorizontalSpacing={:?}; SetStretchChild={:?}",
                spacing, stretch
            )
        }
        Err(error) => format!(
            "WrapPanel activation failed: {error:?}. Deploy XamlToolkit.WinUI.Controls.dll/.pri next to the exe to complete runtime validation."
        ),
    };

    vstack((
        text_block("XamlToolkit Labs")
            .font_size(22.0)
            .bold(),
        text_block(projection_summary),
        text_block(control_status),
        text_block("This Rust demo is the Labs executable. Binding crates under crates/ are generated incrementally and consumed here."),
    ))
    .spacing(8.0)
    .padding(24.0)
    .into()
}

