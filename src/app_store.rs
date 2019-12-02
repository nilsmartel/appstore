use crate::sidebar::sidebar;
use crate::sidebar::SideBarState;
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
