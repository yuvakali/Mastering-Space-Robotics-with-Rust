// src/main.rs
use std::io;
use std::fmt;

// Temperature scale structs
#[derive(Debug, Copy, Clone)]
struct Celsius(f64);

#[derive(Debug, Copy, Clone)]
struct Fahrenheit(f64);

#[derive(Debug, Copy, Clone)]
struct Kelvin(f64);

// Implement Display for each temperature type
impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}°C", self.0)
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}°F", self.0)
    }
}

impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}K", self.0)
    }
}

// Conversion implementations
impl Celsius {
    fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
    
    fn to_kelvin(&self) -> Kelvin {
        Kelvin(self.0 + 273.15)
    }
    
    fn is_valid(&self) -> bool {
        self.0 >= -273.15 // Not below absolute zero
    }
}

impl Fahrenheit {
    fn to_celsius(&self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }
    
    fn to_kelvin(&self) -> Kelvin {
        self.to_celsius().to_kelvin()
    }
    
    fn is_valid(&self) -> bool {
        self.0 >= -459.67 // Not below absolute zero
    }
}

impl Kelvin {
    fn to_celsius(&self) -> Celsius {
        Celsius(self.0 - 273.15)
    }
    
    fn to_fahrenheit(&self) -> Fahrenheit {
        self.to_celsius().to_fahrenheit()
    }
    
    fn is_valid(&self) -> bool {
        self.0 >= 0.0 // Not below absolute zero
    }
}

// Enum to represent temperature scale
#[derive(Debug, Copy, Clone)]
enum Scale {
    C,
    F,
    K,
}

// Enum to store any temperature type
#[derive(Debug, Copy, Clone)]
enum Temperature {
    C(Celsius),
    F(Fahrenheit),
    K(Kelvin),
}

impl Temperature {
    fn display(&self) -> String {
        match self {
            Temperature::C(c) => format!("{}", c),
            Temperature::F(f) => format!("{}", f),
            Temperature::K(k) => format!("{}", k),
        }
    }
}

// Conversion history
struct ConversionHistory {
    entries: Vec<(Temperature, Temperature)>,
}

impl ConversionHistory {
    fn new() -> Self {
        ConversionHistory {
            entries: Vec::new(),
        }
    }
    
    fn add(&mut self, from: Temperature, to: Temperature) {
        self.entries.push((from, to));
    }
    
    fn display(&self) {
        if self.entries.is_empty() {
            println!("No conversion history yet.");
            return;
        }
        
        println!("\nConversion History:");
        println!("------------------");
        for (i, (from, to)) in self.entries.iter().enumerate() {
            println!("{}. {} → {}", i+1, from.display(), to.display());
        }
    }
}

fn parse_temperature(input: &str) -> Result<(f64, Scale), String> {
    // Check if input is empty
    if input.trim().is_empty() {
        return Err("Input cannot be empty".to_string());
    }
    
    // Get the last character for scale
    let input = input.trim();
    let last_char = input.chars().last().unwrap().to_ascii_uppercase();
    
    // Determine scale
    let scale = match last_char {
        'C' => Scale::C,
        'F' => Scale::F,
        'K' => Scale::K,
        _ => return Err("Unknown temperature scale. Use C, F, or K.".to_string()),
    };
    
    // Parse value
    let value_str = &input[..input.len() - 1];
    let value = match value_str.parse::<f64>() {
        Ok(val) => val,
        Err(_) => return Err("Invalid temperature value.".to_string()),
    };
    
    Ok((value, scale))
}

fn create_temperature(value: f64, scale: Scale) -> Result<Temperature, String> {
    match scale {
        Scale::C => {
            let celsius = Celsius(value);
            if !celsius.is_valid() {
                return Err("Temperature below absolute zero".to_string());
            }
            Ok(Temperature::C(celsius))
        },
        Scale::F => {
            let fahrenheit = Fahrenheit(value);
            if !fahrenheit.is_valid() {
                return Err("Temperature below absolute zero".to_string());
            }
            Ok(Temperature::F(fahrenheit))
        },
        Scale::K => {
            let kelvin = Kelvin(value);
            if !kelvin.is_valid() {
                return Err("Temperature below absolute zero".to_string());
            }
            Ok(Temperature::K(kelvin))
        },
    }
}

fn convert_temperature(temp: &Temperature, target_scale: Scale) -> Temperature {
    match (temp, target_scale) {
        // Celsius conversions
        (Temperature::C(c), Scale::F) => Temperature::F(c.to_fahrenheit()),
        (Temperature::C(c), Scale::K) => Temperature::K(c.to_kelvin()),
        (Temperature::C(c), Scale::C) => Temperature::C(*c),
        
        // Fahrenheit conversions
        (Temperature::F(f), Scale::C) => Temperature::C(f.to_celsius()),
        (Temperature::F(f), Scale::K) => Temperature::K(f.to_kelvin()),
        (Temperature::F(f), Scale::F) => Temperature::F(*f),
        
        // Kelvin conversions
        (Temperature::K(k), Scale::C) => Temperature::C(k.to_celsius()),
        (Temperature::K(k), Scale::F) => Temperature::F(k.to_fahrenheit()),
        (Temperature::K(k), Scale::K) => Temperature::K(*k),
    }
}

fn main() {
    println!("SpaceSensor Temperature Converter");
    println!("=================================");
    println!("Enter temperatures with scale (e.g. 32F, 0C, 273.15K)");
    println!("Enter 'history' to view conversion history");
    println!("Enter 'quit' to exit");
    
    let mut history = ConversionHistory::new();
    
    loop {
        println!("\nEnter temperature to convert:");
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("quit") {
            break;
        }
        
        if input.eq_ignore_ascii_case("history") {
            history.display();
            continue;
        }
        
        // Parse input temperature
        let temp_result = match parse_temperature(input) {
            Ok((value, scale)) => create_temperature(value, scale),
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };
        
        let temp = match temp_result {
            Ok(t) => t,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };
        
        // Convert to all other scales
        println!("\nConversion results:");
        for target_scale in [Scale::C, Scale::F, Scale::K].iter() {
            let converted = convert_temperature(&temp, *target_scale);
            
            // Skip showing conversion to same scale
            match (&temp, &converted) {
                (Temperature::C(_), Temperature::C(_)) => continue,
                (Temperature::F(_), Temperature::F(_)) => continue,
                (Temperature::K(_), Temperature::K(_)) => continue,
                _ => {}
            }
            
            println!("{} → {}", temp.display(), converted.display());
            
            // Add to history
            history.add(temp, converted);
        }
    }
    
    println!("Thank you for using SpaceSensor Temperature Converter!");
}