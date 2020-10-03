use bracket_lib::prelude::*;
use legion::*;
use legion::world::SubWorld;

struct State {
    world: World,
    systems: Schedule,
    resources: Resources,
}

impl State {
    fn new() -> Self {
        let mut world = World::default();

        // Spawn our hero
        spawn_hero(&mut world, Point{x: 5, y: 5});

        Self {
            world,
            systems: Schedule::builder().add_system(entity_render_system()).build(),
            resources: Resources::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Render {
    color: ColorPair,
    glyph: FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sjoerd;

pub fn spawn_hero(ecs : &mut World, pos : Point) {
    ecs.push(
        (Sjoerd,
         pos,
         Render{
             color: ColorPair::new(RGB::named(GREEN), RGB::named(BLACK)),
             glyph : to_cp437('S')
         }
        )
    );
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
            draw_batch.set(
                *pos,
                render.color,
                render.glyph
            );
        }
        );
    draw_batch.submit(5000).expect("Batch error");
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // Insert a key as a resource
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
        ctx.print(0, 0, "Hello Sjoerd!");
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("SjoeRPG - Play your life!")
        .build()?;

    main_loop(context, State::new())
}
