use crate::prelude::*;

mod entity_renderer;
mod map_renderer;
mod mouse_systems;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(mouse_systems::update_system())    
        .add_system(map_renderer::render_system())
        .add_system(entity_renderer::render_system())
        .add_system(mouse_systems::render_system())
        .build()
}
