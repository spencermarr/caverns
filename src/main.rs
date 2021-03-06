mod systems;
mod components;
mod factories;
mod resources;
mod colors;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::systems::*;
    pub use crate::components::*;
    pub use crate::factories::*;
    pub use crate::resources::*;
    pub use crate::colors::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();

        UnitFactory::spawn_dwarf(&mut ecs, Point::new(5, 3));
        UnitFactory::spawn_dwarf(&mut ecs, Point::new(7, 3));
        UnitFactory::spawn_dwarf(&mut ecs, Point::new(8, 3));

        resources.insert(MapFactory::build_map());
        resources.insert(Mouse::new(ColorPair::new(WHITE, BLACK), to_cp437('░')));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Caverns")
        .with_fps_cap(60.0)
        .build()?;

    main_loop(context, State::new())
}
