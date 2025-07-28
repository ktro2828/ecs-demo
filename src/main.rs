use std::collections::HashMap;

// === Components ===
#[derive(Debug)]
struct Age(u32);

#[derive(Debug)]
struct Height(f32);

#[derive(Debug)]
struct Weight(f32);

#[derive(Debug)]
struct BMI(f32);

#[derive(Debug)]
struct BloodPressure {
    systolic: u32,
    diastolic: u32,
}

#[derive(Debug)]
struct HyperTensionRisk(String);

// === Entity ===
type EntityId = u32;

#[derive(Debug, Default)]
struct World {
    next_entity: EntityId,
    ages: HashMap<EntityId, Age>,
    heights: HashMap<EntityId, Height>,
    weights: HashMap<EntityId, Weight>,
    bmis: HashMap<EntityId, BMI>,
    blood_pressures: HashMap<EntityId, BloodPressure>,
    hyper_tension_risks: HashMap<EntityId, HyperTensionRisk>,
}

impl World {
    fn spawn(&mut self) -> EntityId {
        let id = self.next_entity;
        self.next_entity += 1;
        id
    }

    fn add_age(&mut self, id: EntityId, age: Age) {
        self.ages.insert(id, age);
    }

    fn add_height(&mut self, id: EntityId, height: Height) {
        self.heights.insert(id, height);
    }

    fn add_weight(&mut self, id: EntityId, weight: Weight) {
        self.weights.insert(id, weight);
    }

    fn insert_bmi(&mut self, id: EntityId, bmi: BMI) {
        self.bmis.insert(id, bmi);
    }

    fn add_blood_pressure(&mut self, id: EntityId, blood_pressure: BloodPressure) {
        self.blood_pressures.insert(id, blood_pressure);
    }

    fn insert_hyper_tension_risk(&mut self, id: EntityId, hyper_tension_risk: HyperTensionRisk) {
        self.hyper_tension_risks.insert(id, hyper_tension_risk);
    }

    fn entities(&self) -> Vec<EntityId> {
        let mut all = self.ages.keys().cloned().collect::<Vec<_>>();
        all.sort();
        all.dedup();
        all
    }
}

// === Systems ===
// === startup systems ===
fn setup(world: &mut World) {
    // add entity1
    {
        let e1 = world.spawn();
        world.add_age(e1, Age(45));
        world.add_height(e1, Height(170.0));
        world.add_weight(e1, Weight(75.0));
        world.add_blood_pressure(
            e1,
            BloodPressure {
                systolic: 130,
                diastolic: 85,
            },
        );
    }

    // add entity2
    {
        let e2 = world.spawn();
        world.add_age(e2, Age(70));
        world.add_height(e2, Height(180.0));
        world.add_weight(e2, Weight(90.0));
        world.add_blood_pressure(
            e2,
            BloodPressure {
                systolic: 150,
                diastolic: 95,
            },
        );
    }
}

// === update systems ===
fn bmi_system(world: &mut World) {
    for id in world.entities() {
        if let (Some(height), Some(weight)) = (world.heights.get(&id), world.weights.get(&id)) {
            let h_m = height.0 / 100.0;
            let bmi = weight.0 / (h_m * h_m);
            world.insert_bmi(id, BMI(bmi));
        }
    }
}

fn hypertension_risk_system(world: &mut World) {
    for id in world.entities() {
        if let (Some(bp), Some(age)) = (world.blood_pressures.get(&id), world.ages.get(&id)) {
            let risk = if bp.systolic >= 140 || bp.diastolic >= 90 {
                if age.0 >= 60 {
                    "High"
                } else {
                    "Moderate"
                }
            } else {
                "Low"
            };
            world.insert_hyper_tension_risk(id, HyperTensionRisk(risk.to_string()));
        }
    }
}

fn gain_weight_system(world: &mut World) {
    for id in world.entities() {
        if let Some(weight) = world.weights.get_mut(&id) {
            weight.0 += 1.0;
        }
    }
}

fn display_system(world: &mut World) {
    println!("=== Displaying result ===");
    for id in world.entities() {
        println!("Entity ID: {}", id);
        if let Some(bmi) = world.bmis.get(&id) {
            println!("BMI: {}", bmi.0);
        }
        if let Some(risk) = world.hyper_tension_risks.get(&id) {
            println!("Hypertension Risk: {}", risk.0);
        }
    }
}

/// === Scheduler ===
type SystemFn = fn(&mut World);

#[derive(Debug, PartialEq, Eq, Hash)]
enum Schedule {
    Startup,
    Update,
}

#[derive(Debug, Default)]
struct App {
    systems: HashMap<Schedule, Vec<SystemFn>>,
    world: World,
}

impl App {
    fn new() -> Self {
        Self::default()
    }

    fn add_system(&mut self, schedule: Schedule, system: SystemFn) -> &mut Self {
        self.systems.entry(schedule).or_default().push(system);
        self
    }

    fn run(&mut self) {
        // call startup systems first
        if let Some(startup_systems) = self.systems.get(&Schedule::Startup) {
            for system in startup_systems {
                system(&mut self.world);
            }
        }

        // NOTE: in game engine, event loop is infinite.
        for _ in 0..5 {
            // call update systems at each iteration
            if let Some(update_systems) = self.systems.get(&Schedule::Update) {
                for system in update_systems {
                    system(&mut self.world);
                }
            }
        }
    }
}

fn main() {
    println!("Let's start the ECS Demo with medical checkup!");

    App::new()
        .add_system(Schedule::Startup, setup)
        .add_system(Schedule::Update, bmi_system)
        .add_system(Schedule::Update, hypertension_risk_system)
        .add_system(Schedule::Update, gain_weight_system)
        .add_system(Schedule::Update, display_system)
        .run();
}
