use druid::lens;
use druid::widget::{Button, TextBox};
use druid::{Data, Lens, LensWrap, Widget};

#[derive(Copy, Clone, Debug, Data, PartialEq)]
pub enum SubMenuID {
    Discover,
    Arcade,
    Create,
    Work,
    Play,
    Develop,
    Categories,
    Updates,
}

impl From<u8> for SubMenuID {
    fn from(i: u8) -> Self {
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

fn sidebar_menu() -> impl Widget<SubMenuID> {
    let mut col = druid::widget::Flex::column();

    for id in (0..8).map(SubMenuID::from) {
        let button = Button::new(format!("{:?}", id)).on_click(
            // Function to be called on click
            move |_ctx, data: &mut SubMenuID, _env| {
                *data = id;
            },
        );
        col.add_child(button);
    }

    col
}

pub fn sidebar() -> impl Widget<SideBarState> {
    let mut col = druid::widget::Flex::column();

    let entry = LensWrap::new(TextBox::new(), lens!(SideBarState, search_term));
    col.add_child(entry);
    let side = LensWrap::new(sidebar_menu(), lens!(SideBarState, current_menu));
    col.add_child(side);

    col
}

#[derive(Clone, Debug, Data, Lens)]
pub struct SideBarState {
    pub search_term: String,
    pub current_menu: SubMenuID,
}

impl Default for SideBarState {
    fn default() -> Self {
        SideBarState {
            search_term: String::new(),
            current_menu: SubMenuID::Discover,
        }
    }
}
