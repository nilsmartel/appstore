use druid::widget::{Button, Column, Label};
use druid::Data;
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};

fn main() {
    let main_window = WindowDesc::new(sidebar_menu);
    let data = SubMenuID::Discover;

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn sidebar_menu() -> impl Widget<SubMenuID> {
    let mut col = Column::new();

    // The label text will be computed dynamically based on the current locale and count
    //
    let text = LocalizedString::new("something").with_arg("count", |data: &SubMenuID, _env| {
        format!("current: {:#?}", data).into()
    });

    let label = Label::new(text);
    col.add_child(label, 1.0);

    for id in (0..8).map(SubMenuID::from_u8) {
        let button = Button::new(
            format!("{:?}", id),
            // Function to be called on click
            move |_ctx, data: &mut SubMenuID, _env| {
                *data = id;
            },
        );
        col.add_child(button, 1.0);
    }

    col
}

#[derive(Copy, Clone, Debug, Data)]
struct AppStoreState {
    current_menu: SubMenuID,
}

#[derive(Copy, Clone, Debug, Data, PartialEq)]
enum SubMenuID {
    Discover,
    Arcade,
    Create,
    Work,
    Play,
    Develop,
    Categories,
    Updates,
}

impl SubMenuID {
    fn from_u8(i: u8) -> Self {
        use SubMenuID::*;
        match i {
            0 => Discover,
            1 => Arcade,
            2 => Create,
            3 => Work,
            4 => Play,
            5 => Develop,
            6 => Categories,
            7 => Updates,
            _ => panic!("invalid index for SubMenuID"),
        }
    }
}
