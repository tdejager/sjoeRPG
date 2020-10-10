use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sjoerd;

#[system]
#[write_component(Point)]
#[read_component(Sjoerd)]
/// Handle input for the player
pub fn sjoerd_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    #[resource] map: &mut Map,
) {
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
            // Get all our sjens
            let mut sjens = <&mut Point>::query().filter(component::<Sjoerd>());
            // Move them
            sjens.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(&destination) {
                    *pos = destination;
                    camera.recenter(destination);
                }
            });
        }
    }
}

// Spawns sjoerd in the map
pub fn spawn_hero(ecs: &mut World, pos: Point) {
    ecs.push((
        Sjoerd,
        pos,
        crate::Render {
            color: ColorPair::new(RGB::named(GREEN), RGB::named(BLACK)),
            glyph: to_cp437('S'),
        },
    ));
}
