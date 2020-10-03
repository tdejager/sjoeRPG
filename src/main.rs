use bracket_lib::prelude::*;
use legion::*;

struct State {
    world: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Sjoerd!");
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("SjoeRPG - Play your life!")
        .build()?;

    main_loop(context, State{ world: World::default()})
}
