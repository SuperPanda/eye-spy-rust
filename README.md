# Eye Spy - An Rust OpenCV Camera Game of Eye Spy

This project aims to develop a game of eye spy using OpenCV and Machine Learning with the goal of learning how write program in Rust and overcome technical limitations through iterative improvements and software engineering principles.

The initial commut release uses Rust and OpenCV to detect if a camera is available and display it to a video feed.

## Prerequisites

- Rust and Cargo (https://www.rust-lang.org/tools/install)
- OpenCV development libraries (https://opencv.org/releases/)
- CMake and pkg-config (https://cmake.org/download/ and https://www.freedesktop.org/wiki/Software/pkg-config/)

## Setup

1. Clone this repository:

```
git clone https://github.com/SuperPanda/eye-spy-rust.git
cd eye-spy-rust
```

2. Build the project:

```
cargo build
```

This command will download and compile the necessary dependencies, as well as build the project itself.

## Running the Project

1. Run the project:

```
cargo run
```

This will display the video feed from the default camera. Press any key to exit the program.

## Troubleshooting

If you encounter any issues with the camera or dependencies, please refer to the respective documentation:

- Rust: https://www.rust-lang.org/learn/get-started
- OpenCV: https://docs.opencv.org/master/
- opencv-rust: https://docs.rs/opencv/
