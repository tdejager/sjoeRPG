use bracket_lib::prelude::*;
use legion::world::SubWorld;
use legion::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

mod camera;

// State of the World
struct State {
    world: World,
    systems: Schedule,
    resources: Resources,
}

impl State {
    fn new() -> Self {
        let mut world = World::default();

        // Spawn our hero: Sjoerd
        spawn_hero(&mut world, Point { x: 5, y: 5 });

        Self {
            world,
            systems: Schedule::builder()
                .add_system(entity_render_system())
                .add_system(sjoerd_input_system())
                .build(),
            resources: Resources::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Render {
    color: ColorPair,
    glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sjoerd;

// Spawns sjoerd in the map
pub fn spawn_hero(ecs: &mut World, pos: Point) {
    ecs.push((
        Sjoerd,
        pos,
        Render {
            color: ColorPair::new(RGB::named(GREEN), RGB::named(BLACK)),
            glyph: to_cp437('S'),
        },
    ));
}

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}

#[system]
#[write_component(Point)]
#[read_component(Sjoerd)]
pub fn sjoerd_input(ecs: &mut SubWorld, #[resource] key: &Option<VirtualKeyCode>) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };


        // Move if something has actually moved
        if delta.x != 0 || delta.y != 0 {
            let mut sjens = <&mut Point>::query().filter(component::<Sjoerd>());
            sjens.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                *pos = destination;
            });
        }
    }
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
    let mut context = BTermBuilder::new()
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
