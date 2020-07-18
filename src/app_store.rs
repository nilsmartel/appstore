use crate::sidebar::sidebar;
use crate::sidebar::SideBarState;
use druid::lens;
use druid::widget::Flex;
use druid::Data;
use druid::Lens;
use druid::LensWrap;
use druid::Widget;

#[derive(Clone, Debug, Data, Lens)]
pub struct AppStoreState {
    pub sidebar: SideBarState,
}

impl Default for AppStoreState {
    fn default() -> Self {
        AppStoreState {
            sidebar: SideBarState::default(),
        }
    }
}

pub fn ui() -> impl Widget<AppStoreState> {
    let mut row = Flex::row();

    let bar = LensWrap::new(sidebar(), lens!(AppStoreState, sidebar));
    row.add_child(bar);
    row
}

pub fn ui2() -> impl Widget<AppStoreState> {
    use crate::debug::string_hash_box;
    let mut col = Flex::row();

    col.add_child(ui());
    col.add_child(string_hash_box());
    col
}
