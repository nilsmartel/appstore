mod app_store;
mod debug;
mod sidebar;

use app_store::AppStoreState;
use druid::{AppLauncher, WindowDesc};

fn main() {
    let main_window = WindowDesc::new(app_store::ui2);

    AppLauncher::with_window(main_window)
        .launch(AppStoreState::default())
        .expect("launching app failed");
}
