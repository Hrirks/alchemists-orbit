use crate::events::GameEvent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// The main physics world that manages the Bevy app
pub struct PhysicsWorld {
    app: App,
    event_channel: std::sync::mpsc::Sender<GameEvent>,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let (tx, _rx) = std::sync::mpsc::channel();

        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_systems(Startup, setup_gravity_well)
            .add_systems(
                Update,
                (update_gravity_field, detect_collisions, handle_merges),
            );

        Self {
            app,
            event_channel: tx,
        }
    }

    pub fn step(&mut self, delta_time: f32) {
        // Step the physics simulation
        self.app.update();
    }

    pub fn drop_orb(&mut self, x: f32, y: f32, tier: u8) {
        // Spawn an orb at the given position
        // This will be implemented to interact with the Bevy world
    }
}

fn setup_gravity_well(mut commands: Commands) {
    // Create the central gravity well
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GravityWell {
            strength: 100.0,
            radius: 500.0,
        },
    ));
}

fn update_gravity_field(
    gravity_wells: Query<(&Transform, &GravityWell)>,
    mut orbs: Query<(&Transform, &mut ExternalForce), With<crate::orb::Orb>>,
) {
    for (well_transform, well) in gravity_wells.iter() {
        for (orb_transform, mut force) in orbs.iter_mut() {
            let delta = well_transform.translation - orb_transform.translation;
            let distance = delta.length();

            if distance < well.radius && distance > 0.0 {
                let gravity_force = delta.normalize() * well.strength / (distance * distance);
                force.force = gravity_force.truncate();
            }
        }
    }
}

fn detect_collisions(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        // Handle collision detection
        println!("Collision detected: {:?}", collision_event);
    }
}

fn handle_merges() {
    // Handle orb merging logic
}

#[derive(Component)]
pub struct GravityWell {
    pub strength: f32,
    pub radius: f32,
}
