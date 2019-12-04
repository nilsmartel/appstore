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
    let mut col = Column::new();

    col.add_child(ui(), 1.0);
    col.add_child(
        LensWrap::new(StringHashBox {}, lenses::app_store_state::sidebar),
        1.0,
    )
}

struct StringHashBox {}
use druid::kurbo::Size;
use druid::BaseState;
use druid::BoxConstraints;
use druid::Env;
use druid::Event;
use druid::EventCtx;
use druid::LayoutCtx;
use druid::PaintCtx;
use druid::UpdateCtx;

impl Widget<String> for StringHashBox {
    fn paint(
        &mut self,
        paint_ctx: &mut PaintCtx,
        base_state: &BaseState,
        data: &String,
        env: &Env,
    ) {
        unimplemented!()
    }
    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &String,
        env: &Env,
    ) -> Size {
        unimplemented!()
    }
    fn event(&mut self, event: &Event, ctx: &mut EventCtx, data: &mut String, env: &Env) {
        unimplemented!()
    }
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&String>, data: &String, env: &Env) {
        unimplemented!()
    }
}
