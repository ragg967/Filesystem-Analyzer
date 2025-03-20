# Filesystem Analyzer

A filesystem analyzer made in Rust.

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction
Filesystem Analyzer is a tool designed to analyze and provide insights into the filesystem. It is built using Rust, ensuring safety and performance.

## Features
- Analyze disk usage
- Identify large files and directories
- Generate reports on filesystem statistics
- Cross-platform support

## Installation
To install Filesystem Analyzer, you need to have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

```sh
# Clone the repository
git clone https://github.com/ragg967/Filesystem-Analyzer.git

# Navigate to the project directory
cd Filesystem-Analyzer

# Build the project
cargo build --release
```

## Usage
After building the project, you can run the filesystem analyzer using the following command:

```sh
./target/release/filesystem-analyzer [OPTIONS] <PATH>
```

Options:
- `-h, --help`: Print help information
- `-V, --version`: Print version information

Example:
```sh
./target/release/filesystem-analyzer /path/to/analyze
```

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request.

1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature-branch`)
5. Create a new Pull Request

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
