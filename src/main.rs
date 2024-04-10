use std::io;
use rand::Rng;

trait WaterLevelSensor {
    fn get_current_water_level(&self) -> i32;
    fn release_water(&self, water_level: i32);
}

struct RandomWaterLevelSensor;

impl WaterLevelSensor for RandomWaterLevelSensor {
    fn get_current_water_level(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..101)
    }

    fn release_water(&self, water_level: i32) {
        if water_level > 30 {
            println!("Water released.");
        } else {
            println!("Water level is too low to release water.");
        }
    }
}

struct OverflowSensor {
    water_level: i32,
}

impl OverflowSensor {
    fn new() -> Self {
        OverflowSensor { water_level: 0 }
    }

    fn add_water(&mut self, amount: i32) {
        self.water_level += amount;
        self.check_overflow();
    }

    fn check_overflow(&self) {
        if self.water_level > 100 {
            println!("Overflow detected in the cistern. Signaling to stop water supply!");
            self.stop_water_supply();
        }
    }

    fn stop_water_supply(&self) {
        println!("Stopping water supply from rainwater collector to prevent overflow in the cistern!");
    }
}

impl WaterLevelSensor for OverflowSensor {
    fn get_current_water_level(&self) -> i32 {
        self.water_level
    }

    fn release_water(&self, water_level: i32) {
        if water_level < 30 {
            println!("Cistern water level is too low. Stopping water outlet.");
        } else {
            println!("Cistern water level is sufficient. Proceeding with water outlet.");
        }
    }
}

struct FountainBasinSensor {
    water_level: i32,
}

impl FountainBasinSensor {
    fn new() -> Self {
        FountainBasinSensor { water_level: 0 }
    }

    fn add_water(&mut self, amount: i32) {
        self.water_level += amount;
        self.check_alarm();
    }

    fn check_alarm(&self) {
        if self.water_level > 2000 {
            println!("Alarm: Water level in the fountain basin is too high!");
        } else if self.water_level < 20 {
            println!("Alarm: Water level in the fountain basin is too low! Using the Fountain is not possible.");
        }
    }
}

impl WaterLevelSensor for FountainBasinSensor {
    fn get_current_water_level(&self) -> i32 {
        self.water_level
    }

    fn release_water(&self, _water_level: i32) {
        // FountainBasinSensor does not release water, so this method is empty
    }
}

fn main() {
    let sensor1 = RandomWaterLevelSensor;
    let sensor2 = RandomWaterLevelSensor;
    let mut sensor3 = OverflowSensor::new();
    let mut sensor4 = FountainBasinSensor::new();

    let mut input = String::new();
    loop {
        let water_level1 = sensor1.get_current_water_level();
        let water_level2 = sensor2.get_current_water_level();
        let water_level3 = sensor3.get_current_water_level();
        let water_level4 = sensor4.get_current_water_level();

        println!("Current water level - Sensor 1: {}", water_level1);
        println!("Current water level - Sensor 2: {}", water_level2);
        println!("Current water level - Sensor 3 (Cistern): {}", water_level3);
        println!("Current water level - Sensor 4 (Fountain Basin): {}", water_level4);

        println!("Please select an action (1: Add water to cistern, 2: Release water from cistern, 3: Quit):");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        match input.trim() {
            "1" => {
                let mut rng = rand::thread_rng();
                let water_amount: i32 = rng.gen_range(1..101);
                println!("Random water amount added to cistern: {}", water_amount);
                sensor3.add_water(water_amount);
                sensor4.add_water(water_amount);
            }
            "2" => {
                sensor3.release_water(sensor3.get_current_water_level());
            }
            "3" => break,
            _ => println!("Invalid input. Please use 1, 2, or 3 to select an action."),
        }
    }
}