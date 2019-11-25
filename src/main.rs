use druid::widget::{Button, Column, Label};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};

fn main() {
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text);
    let button = Button::new("increment", |_ctx, data, _env| *data += 1);

    let mut col = Column::new();
    col.add_child(label, 1.0);
    col.add_child(button, 1.0);
    col
}
