use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Maaike;

// Spawns Maaik in the map
pub fn spawn_maaike(ecs: &mut World, pos: Point) {
    ecs.push((
        Maaike,
        pos,
        crate::Render {
            color: ColorPair::new(RGB::named(PINK), RGB::named(BLACK)),
            glyph: to_cp437('M'),
        },
    ));
}
