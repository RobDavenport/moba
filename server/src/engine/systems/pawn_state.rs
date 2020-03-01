use crate::engine::components::all::*;
use legion::prelude::*;

pub fn pawn_state_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("pawn_state_system")
        .with_query(<Write<Pawn>>::query().filter(changed::<Pawn>()))
        .build(|command_buffer, mut world, _, query| {
            for mut pawn in query.iter(&mut world) {
                if let Some(next_state) = pawn.next_state {
                    pawn.current_state = next_state;
                    pawn.next_state = None;
                }
            }
        })
}
