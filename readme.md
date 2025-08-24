# RecolorImages

A small graphical application to recolor images, built with **Rust**, **egui**, and **eframe**.

## Features
- Open images (PNG, JPEG)
- Recolor / adjust colors
- Simple UI powered by **egui**
- Cross-platform (Windows, Linux, macOS)

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/) (edition 2024 or newer)

### Build
Clone the repository and build in release mode:
```bash
git clone https://github.com/Glubus/recolor_image_app.git
cd recolor_image_app
cargo build --release
```

The executable will be available at:

```
target/release/RecolorImages
```

## Usage

Run the application:

```bash
./target/release/RecolorImages
```

A window will open where you can load an image and apply color transformations.

## License

This project is licensed under the MIT License.
You are free to use, modify, and share it.
