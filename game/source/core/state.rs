type EntityId = string;

pub struct Entity {
    id: EntityId,
}

pub struct Component {
    entity_id: EntityId,
}

pub struct System {
    components: Vec<Component>,
}

pub struct World {
    entities: Vec<Entity>,
    systems: Vec<System>,
}

impl World {
    fn init() {
        let world = World {
            entities: vec![],
            systems: vec![],
        };

        return world;
    }
}
