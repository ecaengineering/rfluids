use rfluids::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello World!");

    let mut water_vapor = Fluid::from(Pure::Water)
        .in_state(FluidInput::pressure(101_325.0), FluidInput::quality(1.0))?;

    println!(
        "water_vapor -> T: {:?} K, D: {:?} kg/m3, Q: {:?}",
        water_vapor.temperature(),
        water_vapor.density(),
        water_vapor.quality(),
    );

    println!("Hello World!");

    Ok(())
}