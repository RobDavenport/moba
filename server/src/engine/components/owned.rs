use legion::prelude::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Owned {
    pub owner: Entity,
}
