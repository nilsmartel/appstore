pub struct StringHashBox {}
use crate::{app_store, sidebar};

use druid::{
    kurbo::Size, piet::RenderContext, BoxConstraints, Env, Event, EventCtx, LayoutCtx, LensWrap,
    PaintCtx, UpdateCtx, Widget,
};

impl Widget<String> for StringHashBox {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &String, env: &Env) {
        // Rectangles: the path for practical people
        let rect = druid::kurbo::Rect::from_origin_size((0., 0.), (64., 64.));

        let sum = hash(data);

        let fill_color = druid::piet::Color::Rgba32(sum | 0xFF);
        paint_ctx.fill(rect, &fill_color);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &String,
        env: &Env,
    ) -> Size {
        Size::new(64.0, 64.0)
    }
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        println!("\n[DEBUG]");
        println!("\n[event] {:?}", event);
    }
    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &String,
        env: &Env,
    ) {
    }
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &String, data: &String, env: &Env) {}
}

pub fn string_hash_box() -> impl Widget<app_store::AppStoreState> {
    LensWrap::new(
        LensWrap::new(
            StringHashBox {},
            druid::lens!(sidebar::SideBarState, search_term),
        ),
        druid::lens!(app_store::AppStoreState, sidebar),
    )
}

fn hash(s: &str) -> u32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    (h.finish() & std::u32::MAX as u64) as u32
}
