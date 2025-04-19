// Define errors that might occur in calculations
#[derive(Debug)]
enum CalculationError {
    InvalidInput,
    DivisionByZero,
    NegativeValue,
    OutOfRange,
}
enum SpaceCalculation {
    OrbitalVelocity { radius: f64,},
    EscapeVelocity { radius: f64 },
    OrbitalPeriod { radius: f64},
    Hohmann { r1: f64, r2: f64, central_mass: f64 },
    TsiolkovskyRocket { exhaust_velocity: f64, init_mass: f64, final_mass: f64 },
    GravitationalForce {  msat: f64, radius: f64 },
    GeostationaryOrbitRadius {  },
    OrbitalEnergy { msat: f64, radius: f64 },
    EscapeEnergy { msat: f64, radius: f64 },
    LinBudgetEquation{
        pt: f64,
        gt: f64,
        gr: f64,
        lp: f64,
        ls: f64,
        lm: f64,
    },
    PathLoss { radius: f64, frequency: f64 },
    InclineAngle { possatx: f64, possaty: f64, possatz: f64, velsatx: f64, velsaty: f64, velsatz: f64 },
    SlantRange { altitude: f64, elevation: f64 },
}
fn calculate(calc: SpaceCalculation) -> Result<f64, CalculationError> {
    const G: f64 = 6.67430e-11; // Gravitational constant
    const mearth: f64 = 5.972e24; // Mass of Earth in kg
    const rearth:f64=6371.0; // Radius of Earth in km
    
    match calc {
        SpaceCalculation::OrbitalVelocity { radius } => {
            if radius <= 0.0 || mearth <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }
            //convert radius to meters
            let radius = radius * 1000.0; // Convert km to m
            
            let orbital_velocity = (G * mearth / radius).sqrt();
            //ok function mention the  unit of the result  in terminal 
            println!("Orbital velocity: {} m/s", orbital_velocity);
            Ok(orbital_velocity) // Result in meters per second (m/s)
             
        },
        
        SpaceCalculation::EscapeVelocity { radius} => {
            if radius <= 0.0 || mearth <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }

            let radius = radius * 1000.0;
            //print the unit of the result in terminal
            println!("Escape velocity: {} m/s", (2.0 * G * mearth / radius).sqrt());
            Ok((2.0 * G * mearth / radius).sqrt())
        },
        
        SpaceCalculation::OrbitalPeriod { radius } => {
            if radius <= 0.0 || mearth <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }
            //convert radius to meters
            let radius = radius * 1000.0; // Convert km to m
            let orbital_period = 2.0 * std::f64::consts::PI * (radius.powi(3) / (G * mearth)).sqrt();
            println!("Orbital period: {} seconds", orbital_period);
            Ok(orbital_period)
        },
        
        SpaceCalculation::Hohmann { r1, r2, central_mass } => {
            if r1 <= 0.0 || r2 <= 0.0 || central_mass <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }
            if r1 == r2 {
                return Err(CalculationError::InvalidInput);
            }
            //convert r1 and r2 to meters
            let r1 = r1 * 1000.0; // Convert km to m
            let r2 = r2 * 1000.0; // Convert km to m
            // Delta-v for Hohmann transfer
            let delta_v1 = ((G * central_mass / r1).sqrt()) * 
                          ((2.0 * r2 / (r1 + r2)).sqrt() - 1.0);
            let delta_v2 = ((G * central_mass / r2).sqrt()) * 
                          (1.0 - (2.0 * r1 / (r1 + r2)).sqrt());
            println!("Delta-v for Hohmann transfer: {} m/s", delta_v1 + delta_v2);
                          
            Ok(delta_v1 + delta_v2) // Total delta-v
        },
        
        SpaceCalculation::TsiolkovskyRocket { exhaust_velocity, init_mass, final_mass } => {
            if exhaust_velocity <= 0.0 || init_mass <= 0.0 || final_mass <= 0.0 || init_mass <= final_mass {
                return Err(CalculationError::InvalidInput);
            }
            
            // Tsiolkovsky rocket equation: delta-v = v_e * ln(m_0 / m_f)
            let delta_v = exhaust_velocity * (init_mass / final_mass).ln();
            println!("Rocket delta-v: {} m/s", delta_v);
            Ok(delta_v)
        }
        SpaceCalculation::GravitationalForce {  msat, radius } => {
            if radius <= 0.0 || mearth <= 0.0 || msat <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }
            //convert radius to meters
            let radius = radius * 1000.0; // Convert km to m
            println!("Gravitational force: {} N", (G * mearth * msat) / radius.powi(2));
            Ok((G * mearth * msat) / radius.powi(2))
        }
        SpaceCalculation::GeostationaryOrbitRadius {  } => {
            let geostationary_radius = (G * mearth * 24.0 * 3600.0_f64.powi(2) / (4.0 * std::f64::consts::PI.powi(2))).cbrt();
            println!("Geostationary orbit radius: {} km", geostationary_radius / 1000.0);
            Ok(geostationary_radius)
        }
        SpaceCalculation::OrbitalEnergy { msat, radius } => {
            if radius <= 0.0 || mearth <= 0.0 || msat <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }
            //convert radius to meters
            let radius = radius * 1000.0; // Convert km to m
            println!("Orbital Energy :{} julies", -G * mearth * msat / (2.0 * radius));
            Ok(-G * mearth * msat / (2.0 * radius))
        }
        SpaceCalculation::EscapeEnergy { msat, radius } => {
            if radius <= 0.0 || mearth <= 0.0 || msat <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }
            //convert radius to meters
            let radius = radius * 1000.0; // Convert km to m
            println!("Escape energy: {} joules", G * mearth * msat / radius);
            Ok(G * mearth * msat / radius)
        }
        SpaceCalculation::LinBudgetEquation{pt, gt, gr, lp, ls, lm} => {
            if pt <= 0.0 || gt <= 0.0 || gr <= 0.0 || lp <= 0.0 || ls <= 0.0 || lm <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }
            //convert pt, gt, gr, lp, ls, lm to dB
            let pt = 10.0_f64.powf(pt / 10.0);      
            let gt = 10.0_f64.powf(gt / 10.0);
            let gr = 10.0_f64.powf(gr / 10.0);
            let lp = 10.0_f64.powf(lp / 10.0);
            let ls = 10.0_f64.powf(ls / 10.0);
            let lm = 10.0_f64.powf(lm / 10.0);
            //calculate the lin budget equation
            
            println!("Lin budget equation: {} dB", pt + gt + gr - lp - ls - lm);
            Ok(pt + gt + gr - lp - ls - lm)
        }
        SpaceCalculation::PathLoss{radius, frequency} => {
            if radius <= 0.0 || frequency <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }
            //convert radius to meters
            let radius = radius * 1000.0; // Convert km to m
            //convert frequency to Hz
            let wavelength = 3e8 / frequency;
            //calculate the path loss
            println!("Path loss: {} dB", 20.0 * radius.ln() + 20.0 * wavelength.ln() + 32.44);
            Ok(20.0 * radius.ln() + 20.0 * wavelength.ln() + 32.44)
        }
        SpaceCalculation::InclineAngle { possatx, possaty, possatz, velsatx, velsaty, velsatz } => {
            let h_x = possaty * velsatz - possatz * velsaty;
            let h_y = possatz * velsatx - possatx * velsatz;
            let h_z = possatx * velsaty - possaty * velsatx;
        
            let h_mag = (h_x.powi(2) + h_y.powi(2) + h_z.powi(2)).sqrt();
        
            if h_mag == 0.0 {
                return Err(CalculationError::InvalidInput);
            }
        
            let cos_i = h_z / h_mag;
            if cos_i.abs() > 1.0 {
                return Err(CalculationError::InvalidInput); // avoid NaN from acos
            }
        
            let inclination_rad = cos_i.acos();
            let inclination_deg = inclination_rad.to_degrees();
            println!("Incline angle: {} degrees", inclination_deg);
            Ok(inclination_deg)
        }
        
        SpaceCalculation::SlantRange{altitude, elevation} => {
            if altitude <= 0.0 || elevation <= 0.0 {
                return Err(CalculationError::InvalidInput);
            }
            //convert altitude to meters
            let altitude = altitude * 1000.0; // Convert km to m
            //convert elevation to radians
            let elevation = elevation.to_radians();
            //calculate the slant range
            let reart = rearth * 1000.0; // Convert km to m
            println!("Slant range: {} m", ((reart + altitude).powi(2) + reart.powi(2) - 2.0 * reart * (reart + altitude) * (elevation.sin())).sqrt());
            Ok(((rearth + altitude).powi(2) + rearth.powi(2) - 2.0 * rearth * (rearth + altitude) * (elevation.to_radians().sin())).sqrt())
        }
    }
}
use std::io::{self, Write};

fn main() {
    println!("=== Space Mission Calculator ===");
    
    loop {
        println!("\nSelect calculation type:");
        println!("1. Orbital Velocity");
        println!("2. Escape Velocity");
        println!("3. Orbital Period");
        println!("4. Hohmann Transfer Delta-V");
        println!("5. Rocket Delta-V (Tsiolkovsky)");
        println!("6. Slant Range");
        println!("7. Gravitational Force");
        println!("8. Geostationary Orbit Radius");
        println!("9. Orbital Energy");
        println!("10. Escape Energy");
        println!("11. Lin Budget Equation");
        println!("12. Path Loss");
        println!("13. Incline Angle");
        println!("14. Slant Range");
        println!("0. Exit");
        
        print!("Enter choice: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        // Parse the choice
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        if choice == 0 {
            println!("Exiting calculator. Goodbye!");
            break;
        }
        
        // Process the calculation type
        let calculation_result = match choice {
            1 => process_orbital_velocity(),
            2 => process_escape_velocity(),
            3 => process_orbital_period(),
            4 => process_hohmann_transfer(),
            5 => process_rocket_delta_v(),
            6 => process_slant_range(),
            7 => process_gravitational_force(),
            8 => process_geostationary_orbit_radius(),
            9 => process_orbital_energy(),
            10 => process_escape_energy(),
            11 => process_lin_budget_equation(),
            12 => process_path_loss(),
            13 => process_incline_angle(),
            14 => process_slant_range(),
            _ => {
                println!("Invalid choice. Please try again.");
                continue;
            }
        };
        
        // Display the result
        match calculation_result {
            Ok(value) => println!("Result: {}", value),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

// Helper function to read a positive f64 value
fn read_positive_f64(prompt: &str) -> Result<f64, CalculationError> {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let value: f64 = match input.trim().parse() {
        Ok(num) if num > 0.0 => num,
        Ok(_) => return Err(CalculationError::NegativeValue),
        Err(_) => return Err(CalculationError::InvalidInput),
    };
    
    Ok(value)
}

// Process functions for each calculation type
fn process_orbital_velocity() -> Result<f64, CalculationError> {
    let radius = read_positive_f64("Enter orbital radius (km)")?;
    
    calculate(SpaceCalculation::OrbitalVelocity { radius})
}

fn process_escape_velocity() -> Result<f64, CalculationError> {
    let radius = read_positive_f64("Enter distance from center of mass (km)")?;
    
    calculate(SpaceCalculation::EscapeVelocity { radius })
}

fn process_orbital_period() -> Result<f64, CalculationError> {
    let radius = read_positive_f64("Enter orbital radius (km)")?;
    
    calculate(SpaceCalculation::OrbitalPeriod { radius })
}

fn process_hohmann_transfer() -> Result<f64, CalculationError> {
    let r1 = read_positive_f64("Enter initial orbital radius (km)")?;
    let r2 = read_positive_f64("Enter final orbital radius (km)")?;
    let central_mass = read_positive_f64("Enter central body mass (kg)")?;
    
    calculate(SpaceCalculation::Hohmann { r1, r2, central_mass })
}

fn process_rocket_delta_v() -> Result<f64, CalculationError> {
    let exhaust_velocity = read_positive_f64("Enter exhaust velocity (m/s)")?;
    let init_mass = read_positive_f64("Enter initial mass (kg)")?;
    let final_mass = read_positive_f64("Enter final mass (kg)")?;
    
    if final_mass >= init_mass {
        return Err(CalculationError::InvalidInput);
    }
    calculate(SpaceCalculation::TsiolkovskyRocket { exhaust_velocity, init_mass, final_mass })
}
fn process_gravitational_force() -> Result<f64, CalculationError> {
    let msat = read_positive_f64("Enter satellite mass (kg)")?;
    let radius = read_positive_f64("Enter distance from center of mass (m)")?;
    
    calculate(SpaceCalculation::GravitationalForce { msat, radius })
}
fn process_geostationary_orbit_radius() -> Result<f64, CalculationError> {
    calculate(SpaceCalculation::GeostationaryOrbitRadius {})
}
fn process_orbital_energy() -> Result<f64, CalculationError> {
    let msat = read_positive_f64("Enter satellite mass (kg)")?;
    let radius = read_positive_f64("Enter distance from center of mass (m)")?;
    
    calculate(SpaceCalculation::OrbitalEnergy { msat, radius })
}
fn process_escape_energy() -> Result<f64, CalculationError> {
    let msat = read_positive_f64("Enter satellite mass (kg)")?;
    let radius = read_positive_f64("Enter distance from center of mass (m)")?;
    
    calculate(SpaceCalculation::EscapeEnergy { msat, radius })
}
fn process_lin_budget_equation() -> Result<f64, CalculationError> {
    let pt = read_positive_f64("Enter Pt (dBm)")?;
    let gt = read_positive_f64("Enter Gt (dBi)")?;
    let gr = read_positive_f64("Enter Gr (dBi)")?;
    let lp = read_positive_f64("Enter Lp (dB)")?;
    let ls = read_positive_f64("Enter Ls (dB)")?;
    let lm = read_positive_f64("Enter Lm (dB)")?;
    
    calculate(SpaceCalculation::LinBudgetEquation { pt, gt, gr, lp, ls, lm })
}
fn process_path_loss() -> Result<f64, CalculationError> {
    let radius = read_positive_f64("Enter distance (m)")?;
    let frequency = read_positive_f64("Enter frequency (Hz)")?;
    
    calculate(SpaceCalculation::PathLoss { radius, frequency })
}
fn process_incline_angle() -> Result<f64, CalculationError> {
    let possatx = read_positive_f64("Enter satellite position x (m)")?;
    let possaty = read_positive_f64("Enter satellite position y (m)")?;
    let possatz = read_positive_f64("Enter satellite position z (m)")?;
    let velsatx = read_positive_f64("Enter satellite velocity x (m/s)")?;
    let velsaty = read_positive_f64("Enter satellite velocity y (m/s)")?;
    let velsatz = read_positive_f64("Enter satellite velocity z (m/s)")?;
    
    calculate(SpaceCalculation::InclineAngle { possatx, possaty, possatz, velsatx, velsaty, velsatz })
}
fn process_slant_range() -> Result<f64, CalculationError> {
    let altitude = read_positive_f64("Enter altitude (km)")?;
    let elevation = read_positive_f64("Enter elevation angle (degrees)")?;
    
    calculate(SpaceCalculation::SlantRange { altitude, elevation })
}