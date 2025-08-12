use std::collections::HashMap;

use crate::world::World;

pub(crate) type SystemFn = fn(&mut World);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Schedule {
    /// System invoked once at the beginning
    Startup,
    /// System invoked every frame before updates
    PreUpdate,
    /// System invoked every frame
    Update,
    /// System invoked every frame after updates
    PostUpdate,
}

#[derive(Debug, Default)]
pub struct App {
    systems: HashMap<Schedule, Vec<SystemFn>>,
    world: World,
}

impl App {
    /// Create a new application.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a system to the application.
    pub fn add_system(&mut self, schedule: Schedule, system: SystemFn) -> &mut Self {
        self.systems.entry(schedule).or_default().push(system);
        self
    }

    /// Run the application for a specified number of loops.
    pub fn run(&mut self, n_loop: u32) {
        // call startup systems first
        if let Some(startup_systems) = self.systems.get(&Schedule::Startup) {
            for system in startup_systems {
                system(&mut self.world)
            }
        }

        // NOTE: in game engine, event loop is infinite
        for _ in 0..n_loop {
            // call pre-update systems
            if let Some(pre_systems) = self.systems.get(&Schedule::PreUpdate) {
                for system in pre_systems {
                    system(&mut self.world)
                }
            }

            // call update systems
            if let Some(update_systems) = self.systems.get(&Schedule::Update) {
                for system in update_systems {
                    system(&mut self.world)
                }
            }

            // call post-update systems
            if let Some(post_systems) = self.systems.get(&Schedule::PostUpdate) {
                for system in post_systems {
                    system(&mut self.world)
                }
            }
        }
    }
}
