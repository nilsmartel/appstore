use crate::sidebar::sidebar;
use crate::sidebar::SideBarState;
use druid::widget::Column;
use druid::widget::Row;
use druid::Data;
use druid::Lens;
use druid::LensWrap;
use druid::Widget;

#[derive(Clone, Debug, Data, Lens)]
pub struct AppStoreState {
    sidebar: SideBarState,
}

impl Default for AppStoreState {
    fn default() -> Self {
        AppStoreState {
            sidebar: SideBarState::default(),
        }
    }
}

pub fn ui() -> impl Widget<AppStoreState> {
    let mut row = Row::new();

    let bar = LensWrap::new(sidebar(), lenses::app_store_state::sidebar);
    row.add_child(bar, 1.0);
    row
}

pub fn ui2() -> impl Widget<AppStoreState> {
    use crate::debug::string_hash_box;
    let mut col = Row::new();

    col.add_child(ui(), 1.0);
    col.add_child(string_hash_box(), 0.2);
    col
}
