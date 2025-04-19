//! # SpaceSensor Temperature Converter
//! 
//! This module provides a temperature conversion utility for space sensor data. It supports conversions between Celsius, Fahrenheit, and Kelvin scales, ensuring high precision and reliability for space applications.
//! 
//! ## Features
//! 
//! - Convert temperatures between Celsius, Fahrenheit, and Kelvin.
//! - Validate temperatures to ensure they are above absolute zero.
//! - Maintain a history of conversions for reference.
//! - User-friendly interface for input and output.
//! 
//! ## Structs
//! 
//! - `Celsius`: Represents a temperature in Celsius.
//! - `Fahrenheit`: Represents a temperature in Fahrenheit.
//! - `Kelvin`: Represents a temperature in Kelvin.
//! - `ConversionHistory`: Stores a history of temperature conversions.
//! 
//! ## Enums
//! 
//! - `Scale`: Represents the temperature scale (Celsius, Fahrenheit, Kelvin).
//! - `Temperature`: Represents a temperature in any of the supported scales.
//! 
//! ## Functions
//! 
//! - `parse_temperature(input: &str) -> Result<(f64, Scale), String>`: Parses a temperature string (e.g., "32F") into a value and scale.
//! - `create_temperature(value: f64, scale: Scale) -> Result<Temperature, String>`: Creates a `Temperature` instance, validating the input.
//! - `convert_temperature(temp: &Temperature, target_scale: Scale) -> Temperature`: Converts a temperature to the specified scale.
//! 
//! ## Methods
//! 
//! ### `Celsius`
//! - `to_fahrenheit(&self) -> Fahrenheit`: Converts Celsius to Fahrenheit.
//! - `to_kelvin(&self) -> Kelvin`: Converts Celsius to Kelvin.
//! - `is_valid(&self) -> bool`: Checks if the temperature is above absolute zero.
//! 
//! ### `Fahrenheit`
//! - `to_celsius(&self) -> Celsius`: Converts Fahrenheit to Celsius.
//! - `to_kelvin(&self) -> Kelvin`: Converts Fahrenheit to Kelvin.
//! - `is_valid(&self) -> bool`: Checks if the temperature is above absolute zero.
//! 
//! ### `Kelvin`
//! - `to_celsius(&self) -> Celsius`: Converts Kelvin to Celsius.
//! - `to_fahrenheit(&self) -> Fahrenheit`: Converts Kelvin to Fahrenheit.
//! - `is_valid(&self) -> bool`: Checks if the temperature is above absolute zero.
//! 
//! ### `Temperature`
//! - `display(&self) -> String`: Returns a formatted string representation of the temperature.
//! 
//! ### `ConversionHistory`
//! - `new() -> Self`: Creates a new, empty conversion history.
//! - `add(&mut self, from: Temperature, to: Temperature)`: Adds a conversion entry to the history.
//! - `display(&self)`: Displays the conversion history.
//! 
//! ## Usage
//! 
//! The main function provides an interactive interface for users to input temperatures, convert them, and view the conversion history. Example usage:
//! 
//! ```bash
//! Enter temperatures with scale (e.g., 32F, 0C, 273.15K)
//! Enter 'history' to view conversion history
//! Enter 'quit' to exit
//! ```
//! 
//! ## Example
//! 
//! ```rust
//! let celsius = Celsius(25.0);
//! let fahrenheit = celsius.to_fahrenheit();
//! let kelvin = celsius.to_kelvin();
//! 
//! println!("{} is {} or {}", celsius, fahrenheit, kelvin);
//! ```
//! 
//! ## Notes
//! 
//! - Temperatures below absolute zero are considered invalid and will result in an error.
//! - The conversion history is stored in memory and is cleared when the program exits.
# SpaceSensor Temperature Converter

Welcome to the **SpaceSensor Temperature Converter** project! This project is part of the "Mastering Space Robotics with Rust" course, specifically in **Week 1: Core Rust Foundation**. It demonstrates the use of Rust for building a temperature conversion utility for space sensor data.

## Features

- Convert temperatures between Celsius, Fahrenheit, and Kelvin.
- Simple and efficient implementation using Rust.
- Designed for high precision and reliability in space applications.

## Getting Started

### Prerequisites

- Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/).

### Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/yuvakali/Mastering-Space-Robotics-with-Rust.git
    cd Mastering-Space-Robotics-with-Rust/Week1_Core_Rust_Foundation/spacesensor_temperature_converter
    ```

2. Build the project:
    ```bash
    cargo build
    ```

3. Run the tests:
    ```bash
    cargo test
    ```

### Usage

Run the program with:
```bash
cargo run
```

Follow the on-screen instructions to input temperature values and select the desired conversion.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Acknowledgments

Thanks to the "Mastering Space Robotics with Rust" course for inspiring this project.