mod camera;
mod maaike;
mod map;
mod map_builder;
mod sjoerd;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::maaike::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::sjoerd::*;
    pub use bracket_lib::prelude::*;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use crate::prelude::*;

// State of the World
struct State {
    world: World,
    systems: Schedule,
    resources: Resources,
}

impl State {
    fn new() -> Self {
        let mut world = World::default();
        let map_builder = MapBuilder::build(&mut RandomNumberGenerator::new());
        // Set in center of screen
        let player_position = map_builder.player_start;
        // Spawn our hero: Sjoerd
        sjoerd::spawn_hero(&mut world, player_position.clone());
        // Spawn his partner Maaike
        maaike::spawn_maaike(&mut world, player_position.clone() + Point::new(1, 1));
        let mut resources = Resources::default();
        resources.insert(Camera::new(player_position));
        resources.insert(map_builder.map);

        Self {
            world,
            systems: Schedule::builder()
                .add_system(entity_render_system())
                .add_system(sjoerd_input_system())
                .add_system(map_render_system())
                .build(),
            resources,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    color: ColorPair,
    glyph: FontCharType,
}

#[system]
#[read_component(Point)]
#[read_component(Render)]
/// Render entities that are currently in the game
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(camera.world_to_screen(pos), render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear both consoles
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        // Insert a key as a resource
        self.resources.insert(ctx.key);
        // Execute the current systems
        self.systems.execute(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("SjoeRPG - Play your life!")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        // .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("cheepicus8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "cheepicus8x8.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "cheepicus8x8.png")
        .build()?;

    // Run the main loop
    main_loop(context, State::new())
}
