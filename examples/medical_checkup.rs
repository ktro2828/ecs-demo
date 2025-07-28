use ecs_demo::{
    app::{App, Schedule},
    world::World,
};

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
struct HyperTensionRisk(HyperTensionRiskLevel);

#[derive(Debug)]
enum HyperTensionRiskLevel {
    Low,
    Moderate,
    High,
}

// === Systems ===
// === startup systems ===
fn setup(world: &mut World) {
    // add entity1
    {
        let e = world.spawn();
        world.insert(e, Age(30));
        world.insert(e, Height(170.0));
        world.insert(e, Weight(70.0));
        world.insert(e, BMI(22.0));
        world.insert(
            e,
            BloodPressure {
                systolic: 120,
                diastolic: 80,
            },
        );
    }

    // add entity2
    {
        let e = world.spawn();
        world.insert(e, Age(45));
        world.insert(e, Height(180.0));
        world.insert(e, Weight(90.0));
        world.insert(e, BMI(27.0));
        world.insert(
            e,
            BloodPressure {
                systolic: 130,
                diastolic: 90,
            },
        );
    }
}

// === update systems ===
fn bmi_system(world: &mut World) {
    for id in world.entities() {
        if let (Some(height), Some(weight)) = (world.get::<Height>(id), world.get::<Weight>(id)) {
            let h_m = height.0 / 100.0;
            let bmi = weight.0 / (h_m * h_m);
            world.insert(id, BMI(bmi));
        }
    }
}

fn hypertension_risk_system(world: &mut World) {
    for id in world.entities() {
        if let (Some(bp), Some(age)) = (world.get::<BloodPressure>(id), world.get::<Age>(id)) {
            let risk = if bp.systolic >= 140 || bp.diastolic >= 90 {
                if age.0 >= 60 {
                    HyperTensionRiskLevel::High
                } else {
                    HyperTensionRiskLevel::Moderate
                }
            } else {
                HyperTensionRiskLevel::Low
            };
            world.insert(id, HyperTensionRisk(risk));
        }
    }
}

fn gain_weight_system(world: &mut World) {
    for id in world.entities() {
        if let Some(weight) = world.get_mut::<Weight>(id) {
            weight.0 += 1.0;
        }
    }
}

fn display_system(world: &mut World) {
    println!("=== Displaying result ===");
    for id in world.entities() {
        println!("Entity ID: {}", id);
        if let Some(bmi) = world.get::<BMI>(id) {
            println!("BMI: {}", bmi.0);
        }
        if let Some(risk) = world.get::<HyperTensionRisk>(id) {
            println!("Hypertension Risk: {:?}", risk.0);
        }
    }
}

fn main() {
    println!("Let's start the ECS demo with medical checkup!");

    App::new()
        .add_system(Schedule::Startup, setup)
        .add_system(Schedule::Update, bmi_system)
        .add_system(Schedule::Update, hypertension_risk_system)
        .add_system(Schedule::Update, gain_weight_system)
        .add_system(Schedule::Update, display_system)
        .run(5);
}
