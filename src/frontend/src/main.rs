use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

use algorithms as alg;
mod widgets;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).title("Pathfinder");
    let data = String::new();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<String> {
    widgets::MapWidget::from_map(alg::get_random_data())
}
