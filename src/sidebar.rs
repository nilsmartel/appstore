use druid::widget::{Button, Column, TextBox};
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

fn sidebar_menu() -> impl Widget<SubMenuID> {
    let mut col = Column::new();

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

pub fn sidebar() -> impl Widget<SideBarState> {
    let mut col = Column::new();

    let entry = LensWrap::new(TextBox::new(), lenses::side_bar_state::search_term);
    col.add_child(entry, 0.5);
    let side = LensWrap::new(sidebar_menu(), lenses::side_bar_state::current_menu);
    col.add_child(side, 1.0);

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
