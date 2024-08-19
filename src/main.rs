struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    fn monthly_insurance() -> u32 {
        let insurance: u32 = 123;
        insurance
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }

    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year: year,
            fuel_level: 0.0,
            price: 0,
        }
    }

    fn display_car_info(&self) {
        println!(
            "Owner : {}, Year : {}, Price : {}, Fuel : {}",
            self.owner, self.year, self.price, self.fuel_level
        )
    }

    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    fn sell(self) -> Self {
        self
    }
}

fn main() {
    let my_car = Car {
        owner: String::from("Utkarsh Gaikwad"),
        year: 2020,
        fuel_level: 43.5,
        price: 17_00_000,
    };
    my_car.display_car_info();

    let mut another_car = Car {
        owner: String::from("Kalyani"),
        ..my_car
    };

    another_car.display_car_info();
    another_car.refuel(25.3);
    another_car.display_car_info();
    let new_car = another_car.sell();

    new_car.display_car_info();

    let mut car2 = Car::new("XYZ".to_string(), 2023);
    car2.price = 5000;
    car2.display_car_info();

    let sp: u32 = car2.selling_price();
    println!("Insurance : {}", Car::monthly_insurance());
    println!("Selling Price : {}", sp);

    #[derive(Debug)]
    struct point_2D(i32, i32);

    #[derive(Debug)]
    struct point_3D(i32, i32, i32);

    let point1 = point_2D(2, 3);
    let point2 = point_3D(1, 3, 4);

    println!("{:?}", point1);
    println!("{:?}", point2);

    //unit structs
    struct ABC;
}
