use std::time;

use ecs_demo::{
    app::{App, Schedule},
    world::World,
};

// === Components ===
#[derive(Debug, Clone)]
struct Age(u32);

#[derive(Debug, Clone)]
struct Height(f32);

#[derive(Debug, Clone)]
struct Weight(f32);

#[derive(Debug, Clone)]
struct BMI(f32);

#[derive(Debug, Clone)]
struct BloodPressure {
    systolic: u32,
    diastolic: u32,
}

#[derive(Debug, Clone)]
struct HyperTensionRisk(HyperTensionRiskLevel);

impl Default for HyperTensionRisk {
    fn default() -> Self {
        Self(HyperTensionRiskLevel::Unknown)
    }
}

#[derive(Debug, Clone)]
enum HyperTensionRiskLevel {
    Low,
    Moderate,
    High,
    Unknown,
}

// === Systems ===
// === startup systems ===
fn setup(world: &mut World) {
    // add entity1
    {
        let e = world.spawn((
            Age(30),
            Height(170.0),
            Weight(70.0),
            BMI(22.0),
            BloodPressure {
                systolic: 120,
                diastolic: 80,
            },
            HyperTensionRisk::default(),
        ));
    }

    // add entity2
    {
        let e = world.spawn((
            Age(45),
            Height(180.0),
            Weight(90.0),
            BMI(27.0),
            BloodPressure {
                systolic: 130,
                diastolic: 90,
            },
            HyperTensionRisk::default(),
        ));
    }
}

// === update systems ===
fn bmi_system(world: &mut World) {
    for archetype in world.archetypes.values_mut() {
        let heights = archetype.get::<Height>().map(|v| v.clone());
        let weights = archetype.get::<Weight>().map(|v| v.clone());

        if let (Some(heights), Some(weights)) = (heights, weights) {
            if let Some(bmis) = archetype.get_mut::<BMI>() {
                for ((h, w), bmi) in heights.iter().zip(weights.iter()).zip(bmis.iter_mut()) {
                    let h_m = h.0 / 100.0;
                    bmi.0 = w.0 / (h_m * h_m);
                }
            }
        }
    }
}

fn hypertension_risk_system(world: &mut World) {
    for archetype in world.archetypes.values_mut() {
        let bps = archetype.get::<BloodPressure>().map(|v| v.clone());
        let ages = archetype.get::<Age>().map(|v| v.clone());
        if let (Some(bps), Some(ages)) = (bps, ages) {
            if let Some(risks) = archetype.get_mut::<HyperTensionRisk>() {
                for ((bp, age), risk) in bps.iter().zip(ages.iter()).zip(risks.iter_mut()) {
                    risk.0 = if bp.systolic >= 140 || bp.diastolic >= 90 {
                        if age.0 >= 60 {
                            HyperTensionRiskLevel::High
                        } else {
                            HyperTensionRiskLevel::Moderate
                        }
                    } else {
                        HyperTensionRiskLevel::Low
                    };
                }
            }
        }
    }
}

fn gain_weight_system(world: &mut World) {
    for archetype in world.archetypes.values_mut() {
        if let Some(weights) = archetype.get_mut::<Weight>() {
            for weight in weights.iter_mut() {
                weight.0 += 1.0;
            }
        }
    }
}

fn display_system(world: &mut World) {
    println!("=== Displaying result ===");
    for id in world.entities() {
        println!("Entity ID: {}", id);
        if let Some(bmi) = world.get_component::<BMI>(id) {
            println!("BMI: {}", bmi.0);
        }
        if let Some(risk) = world.get_component::<HyperTensionRisk>(id) {
            println!("Hypertension Risk: {:?}", risk.0);
        }
    }
}

fn main() {
    println!("Let's start the ECS demo with medical checkup!");

    let start = time::Instant::now();

    App::new()
        .add_system(Schedule::Startup, setup)
        .add_system(Schedule::Update, bmi_system)
        .add_system(Schedule::Update, hypertension_risk_system)
        .add_system(Schedule::Update, gain_weight_system)
        .add_system(Schedule::Update, display_system)
        .run(5);

    println!(">> Time elapsed: {:?}", start.elapsed());
}
