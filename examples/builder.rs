#[derive(Debug)]
pub struct Car {
    pub model: String,
    pub seats: u32,
}

impl Car {
    fn builder() -> CarBuilder {
        CarBuilder::new()
    }
}

struct CarBuilder {
    model: Option<String>,
    seats: Option<u32>,
}

impl CarBuilder {
    fn new() -> Self {
        Self {
            model: None,
            seats: None,
        }
    }

    fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    fn with_seats(mut self, seats: u32) -> Self {
        self.seats = Some(seats);
        self
    }

    fn build(self) -> Car {
        Car {
            model: self.model.expect("model field should be set"),
            seats: self.seats.expect("seats field should be set"),
        }
    }
}

impl Default for CarBuilder {
    fn default() -> Self {
        CarBuilder::new()
    }
}

fn main() {
    println!("builder");
    let car = Car::builder()
        .with_model("Ford".into())
        .with_seats(4)
        .build();

    println!("{:?}", car);
}
