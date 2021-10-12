use crate::prelude::*;

mod map_renderer;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(map_renderer::render_system())
        .build()
}
