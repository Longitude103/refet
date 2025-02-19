# Refet Rust Library

The `refet` library is a Rust-based module designed to handle reference evapotranspiration calculations. It is part of a
larger project that includes climate and crop modules, providing comprehensive tools for environmental and agricultural
data analysis.
The `refet` library uses the ASCE Standardized method to caluclate the ET rates and returns both the Short and Tall
Reference Crop values.

## Features

- Calculate reference evapotranspiration using ASCE Standardized Method and Heargraves if lacking data.
- Integrate seamlessly with other modules in the project.
- Designed for high performance and accuracy.

## Installation

To use the `refet` library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
refet = { path = "../refet" }
```

```rust
use refet::calculate_evapotranspiration;

fn main() {
    let result = calculate_evapotranspiration(/* parameters */);
    println!("Reference Evapotranspiration: {}", result);
}
```

## Testing

Tests embedded in the code have been created using edge cases or examples from the ASCE Standardized publication and are
within an acceptable range for calculations with floating point numbers but may not match perfectly.
The ASCE Standardized publication uses different values and decimal places depending on the text or appendix. These can
be slightly different from the results from this application.

## Contributing

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes and commit them with clear messages.
4. Push your changes to your fork.
5. Submit a pull request with a description of your changes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Contact

For questions or feedback, please contact the project maintainers.