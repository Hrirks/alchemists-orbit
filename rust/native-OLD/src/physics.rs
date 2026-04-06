use crate::events::GameEvent;
use crate::orb::{Orb, OrbBundle, OrbTier};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::sync::mpsc::{Receiver, Sender};

/// Commands that can be sent to the physics world
#[derive(Debug)]
pub enum PhysicsCommand {
    SpawnOrb { x: f32, y: f32, tier: OrbTier },
}

/// The main physics world that manages the Bevy app
pub struct PhysicsWorld {
    app: App,
    event_channel: Sender<GameEvent>,
    command_receiver: Receiver<PhysicsCommand>,
    command_sender: Sender<PhysicsCommand>,
    next_orb_id: u32,
    orb_count: usize,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let (event_tx, _event_rx) = std::sync::mpsc::channel();
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel();

        let mut app = App::new();

        // Clone the event sender for the resource
        let event_sender_resource = EventSender {
            sender: event_tx.clone(),
        };

        app.add_plugins(MinimalPlugins)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .insert_resource(PendingMerges::default())
            .insert_resource(OrbIdCounter::default())
            .insert_resource(event_sender_resource)
            .add_systems(Startup, setup_gravity_well)
            .add_systems(
                Update,
                (update_gravity_field, detect_collisions, handle_merges).chain(),
            );

        Self {
            app,
            event_channel: event_tx,
            command_receiver: cmd_rx,
            command_sender: cmd_tx,
            next_orb_id: 1,
            orb_count: 0,
        }
    }

    pub fn step(&mut self, _delta_time: f32) {
        // Process any pending commands
        while let Ok(command) = self.command_receiver.try_recv() {
            match command {
                PhysicsCommand::SpawnOrb { x, y, tier } => {
                    if self.orb_count < 50 {
                        self.spawn_orb_internal(x, y, tier);
                    } else {
                        println!("Max orb limit (50) reached!");
                    }
                }
            }
        }

        // Step the physics simulation
        self.app.update();
    }

    pub fn drop_orb(&mut self, x: f32, y: f32, tier: u8) {
        if let Some(tier_enum) = OrbTier::from_u8(tier) {
            let _ = self.command_sender.send(PhysicsCommand::SpawnOrb {
                x,
                y,
                tier: tier_enum,
            });
        } else {
            println!("Invalid tier: {}", tier);
        }
    }

    fn spawn_orb_internal(&mut self, x: f32, y: f32, tier: OrbTier) {
        let orb_id = self.next_orb_id;
        self.next_orb_id += 1;
        self.orb_count += 1;

        println!(
            "Spawning orb #{} (tier {:?}) at ({}, {})",
            orb_id, tier, x, y
        );

        // Spawn the orb in the Bevy world
        self.app
            .world_mut()
            .spawn(OrbBundle::new(orb_id, tier, x, y));

        // Send event to Flutter
        let _ = self.event_channel.send(GameEvent::OrbSpawned {
            id: orb_id,
            tier: tier as u8,
            x,
            y,
        });
    }
}

fn setup_gravity_well(mut commands: Commands) {
    // Create the central gravity well
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GravityWell {
            strength: 5000.0, // Increased strength for better attraction
            radius: 1000.0,
            rotation_speed: 0.0, // Will increase with level
        },
    ));
}

fn update_gravity_field(
    gravity_wells: Query<(&Transform, &GravityWell)>,
    mut orbs: Query<(&Transform, &mut ExternalForce), With<Orb>>,
) {
    for (well_transform, well) in gravity_wells.iter() {
        for (orb_transform, mut force) in orbs.iter_mut() {
            let delta = well_transform.translation - orb_transform.translation;
            let distance = delta.length();

            if distance < well.radius && distance > 0.1 {
                // Apply gravity force (inverse square law)
                let gravity_magnitude = well.strength / (distance * distance + 1.0);
                let gravity_force = delta.normalize() * gravity_magnitude;
                force.force = gravity_force.truncate();
            }
        }
    }
}

/// Resource to track pending merges
#[derive(Resource, Default)]
struct PendingMerges {
    merges: Vec<(Entity, Entity, OrbTier)>,
}

/// Resource to track the next orb ID
#[derive(Resource)]
struct OrbIdCounter {
    next_id: u32,
}

impl Default for OrbIdCounter {
    fn default() -> Self {
        Self { next_id: 1 }
    }
}

/// Resource to send events to Flutter
#[derive(Resource)]
struct EventSender {
    sender: Sender<GameEvent>,
}

fn detect_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    orbs: Query<&Orb>,
    mut pending_merges: ResMut<PendingMerges>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Check if both entities are orbs
            if let (Ok(orb1), Ok(orb2)) = (orbs.get(*entity1), orbs.get(*entity2)) {
                // Check if they're the same tier
                if orb1.tier == orb2.tier {
                    println!(
                        "Collision detected between orbs #{} and #{} (tier {:?})",
                        orb1.id, orb2.id, orb1.tier
                    );

                    // Add to pending merges (avoid duplicates)
                    if !pending_merges.merges.iter().any(|(e1, e2, _)| {
                        (*e1 == *entity1 && *e2 == *entity2) || (*e1 == *entity2 && *e2 == *entity1)
                    }) {
                        pending_merges.merges.push((*entity1, *entity2, orb1.tier));
                    }
                }
            }
        }
    }
}

fn handle_merges(
    mut commands: Commands,
    mut pending_merges: ResMut<PendingMerges>,
    mut orb_id_counter: ResMut<OrbIdCounter>,
    event_sender: Res<EventSender>,
    orbs: Query<(&Orb, &Transform)>,
) {
    // Process all pending merges
    for (entity1, entity2, tier) in pending_merges.merges.drain(..) {
        // Check if both entities still exist
        if let (Ok((orb1, transform1)), Ok((orb2, transform2))) =
            (orbs.get(entity1), orbs.get(entity2))
        {
            if let Some(next_tier) = tier.next_tier() {
                println!(
                    "Merging orbs #{} and #{} from {:?} to {:?}",
                    orb1.id, orb2.id, tier, next_tier
                );

                // Calculate merge position (midpoint)
                let merge_x = (transform1.translation.x + transform2.translation.x) / 2.0;
                let merge_y = (transform1.translation.y + transform2.translation.y) / 2.0;

                // Get new ID
                let new_id = orb_id_counter.next_id;
                orb_id_counter.next_id += 1;

                // Despawn both orbs
                commands.entity(entity1).despawn();
                commands.entity(entity2).despawn();

                // Spawn new orb at merge position
                commands.spawn(OrbBundle::new(new_id, next_tier, merge_x, merge_y));

                // Send merge event to Flutter
                let _ = event_sender.sender.send(GameEvent::OrbMerged {
                    orb1_id: orb1.id,
                    orb2_id: orb2.id,
                    new_orb_id: new_id,
                    new_tier: next_tier as u8,
                    x: merge_x,
                    y: merge_y,
                });
            } else {
                println!("Orbs at max tier (Tier7), cannot merge further");
            }
        }
    }
}

#[derive(Component)]
pub struct GravityWell {
    pub strength: f32,
    pub radius: f32,
    pub rotation_speed: f32,
}
