use actix_web::{get, post, web, HttpResponse, Responder};
use rand::Rng;
use std::sync::{Arc, Mutex};
use serde_json::json;
use actix_web::App;
use actix_web::HttpServer;

trait WaterLevelSensor {
    fn get_current_water_level(&self) -> i32;
    fn release_water(&mut self, water_level: i32);
    fn add_water(&mut self);
}

struct RandomWaterLevelSensor;

impl WaterLevelSensor for RandomWaterLevelSensor {
    fn get_current_water_level(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..101)
    }

    fn release_water(&mut self, water_level: i32) {
        if water_level > 30 {
            println!("Water released.");
        } else {
            println!("Water level is too low to release water.");
        }
    }

    fn add_water(&mut self) {
        // Not implemented for RandomWaterLevelSensor
    }
}

struct OverflowSensor {
    water_level: i32,
}

impl OverflowSensor {
    fn new() -> Self {
        OverflowSensor { water_level: 0 }
    }
}

impl WaterLevelSensor for OverflowSensor {
    fn get_current_water_level(&self) -> i32 {
        self.water_level
    }

    fn release_water(&mut self, water_level: i32) {
        if water_level < 30 {
            println!("Cistern water level is too low. Stopping water outlet.");
        } else {
            println!("Cistern water level is sufficient. Proceeding with water outlet.");
        }
    }

    fn add_water(&mut self) {
        let mut rng = rand::thread_rng();
        let water_amount: i32 = rng.gen_range(1..101);
        self.water_level += water_amount;
    }
}

struct FountainBasinSensor {
    water_level: i32,
}

impl FountainBasinSensor {
    fn new() -> Self {
        FountainBasinSensor { water_level: 0 }
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

    fn release_water(&mut self, _water_level: i32) {
        // FountainBasinSensor does not release water, so this method is empty
    }

    fn add_water(&mut self) {
        let mut rng = rand::thread_rng();
        let water_amount: i32 = rng.gen_range(1..101);
        self.water_level += water_amount;
        self.check_alarm();
    }
}

// Define API handlers
#[get("/sensors")]
async fn get_sensor_state(data: web::Data<Arc<Mutex<(RandomWaterLevelSensor, RandomWaterLevelSensor, OverflowSensor, FountainBasinSensor)>>>) -> impl Responder {
    match data.lock() {
        Ok(sensors) => {
            let (sensor1, sensor2, sensor3, sensor4) = &*sensors;

            let water_level1 = sensor1.get_current_water_level();
            let water_level2 = sensor2.get_current_water_level();
            let water_level3 = sensor3.get_current_water_level();
            let water_level4 = sensor4.get_current_water_level();

            HttpResponse::Ok().json(json!({
                "sensor1": water_level1,
                "sensor2": water_level2,
                "sensor3": water_level3,
                "sensor4": water_level4,
            }))
        }
        Err(e) => {
            println!("Failed to lock sensors: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/add_water")]
async fn add_water(data: web::Data<Arc<Mutex<(RandomWaterLevelSensor, RandomWaterLevelSensor, OverflowSensor, FountainBasinSensor)>>>) -> impl Responder {
    match data.lock() {
        Ok(mut sensors) => {
            let (_, _, sensor3, sensor4) = &mut *sensors;
            sensor3.add_water();
            sensor4.add_water();

            HttpResponse::Ok().body("Random water added to cistern and fountain basin.")
        }
        Err(e) => {
            println!("Failed to lock sensors: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create shared state for sensors
    let sensors = Arc::new(Mutex::new((
        RandomWaterLevelSensor,
        RandomWaterLevelSensor,
        OverflowSensor::new(),
        FountainBasinSensor::new(),
    )));

    // Start Actix HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sensors.clone()))
            .service(get_sensor_state)
            .service(add_water)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
