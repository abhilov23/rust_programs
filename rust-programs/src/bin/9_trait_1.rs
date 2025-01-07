
struct Sedan;
impl Sedan{
    fn drive(&self){
        println!("Sedan is driving");
    }
}

fn road_trip(vehicle: &Sedan){
    vehicle.drive(); // Calls the drive method from the Vehicle trait
}

fn main(){
    let car = Sedan;
    car.drive(); //prints: Sedan is driving
    road_trip(&car); //passing the car so now road_trip also have a drive method
}