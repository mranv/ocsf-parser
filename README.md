# OCSF Parser

OCSF Parser is a Rust implementation for parsing log files according to the Open Cybersecurity Schema Framework (OCSF).

## Overview

This project provides a flexible and efficient parser for converting various log formats into the standardized OCSF schema. It aims to simplify log analysis and improve interoperability between different security tools.

## Features

- Parse log files into OCSF-compliant JSON format
- Support for multiple log formats (extensible)
- Efficient processing of large log files
- Command-line interface for easy integration into existing workflows

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/ocsf-parser.git
   cd ocsf-parser
   ```

2. Build the project:
   ```
   cargo build --release
   ```

### Usage

To parse a log file:

```
cargo run --bin ocsf-parser -- path/to/your/logfile.log
```

For the sample implementation:

```
cargo run --bin sample-parser -- path/to/your/logfile.log
```

## Project Structure

- `src/main.rs`: Main entry point for the OCSF parser
- `src/parser.rs`: Contains the log parsing logic
- `src/schema.rs`: Defines the OCSF event structure
- `src/sample.rs`: Alternative implementation (for demonstration or testing)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Open Cybersecurity Schema Framework (OCSF)](https://github.com/ocsf)
- All contributors to this project
