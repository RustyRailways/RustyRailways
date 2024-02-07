# Rusty Railways
git repository for the embedded project

<img alt="Rusty Railways Logo" src="imgs/logo.png" width="200" height="200">

## Description

We designed a system to automate the motion of lego trains.
You can tell the system in which station you want a train to go (either using an HTTP request or using our GUI),
and the system will figure out the best way to achieve the goal, even moving trains that are on your
way if necessary. 

## Video

[![Watch the video](https://img.youtube.com/vi/AwafprRb6JQ/0.jpg)](https://youtu.be/AwafprRb6JQ)

## Components

The system is implemented using 5 different components:

 - `The Master`: Is the component that dose all the calculation to find the optimal path, and then controls the trains and the switches, you can learn more about the master [here](./master/readme.md)
 - `The Clint`: ([Godot](https://godotengine.org/) based client to create the train track and control/monitor the train system), you can learn more about the client [here](./client/README.md)
 - `The Map`: The map is a support data structure (a modified graph in particular) used by the master, you can learn more about the master [here](./map/readme.md)
 - `The Common Infrastructure`: The common infrastructure defines some basic messages that all components needs to communicate with each other, as well as the IPs and the hardware abstraction layer traits, you can learn more about the master [here](./common_infrastructure/readme.md)
 - `The Trains`: You can learn more about the code that manage the trains [here](./train/rust/readme.md)
 - `The Switches`: You can learn more about the code that manage the switches [here](./switch/readme.md)

## Documentation

Other that the readme available at the links above we have also a HTML documentation generated with 
doxigen and rust-doc available [here](https://mrlakige.github.io/rusty_railways/)

## Installation

Here you can find a quick list of the things you need to do to try our project yourself 

### Master Installation
The current implementation of the master runs on a linux system (e.g. Raspberry py with Raspberry Pi OS) so in order to run in you just need to install the [regular rust toolchain](https://www.rust-lang.org/tools/install)
Given that we used a Hardware abstraction layer to implement the master, with a bit more work, it can be executed on something smaller (like an eso32), if yow are interested in this you can check out the train toolchain installation down bellow. 

### Train Toolchain Installation
The trains code is built using embedded rust, with the [embedded-hal](https://github.com/rust-embedded/embedded-hal) and [embedded-svc](https://github.com/esp-rs/embedded-svc/tree/master) crates. To install the software for our target (The ESP-32) you can follow the tutorial [here](https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites)

### Switch Toolchain Installation
To compile the switch code you can follow [this](./switch/readme.md) tutorial we written

### Client Installation
See [Client README.md](https://github.com/MrLakige/rusty_railways/blob/main/client/README.md).

### 3D Printing
Files to print the switch mechanism are located [here](./switch/hardware/Switch_3D_print/)
You can find a 3D printing tutorial [here](https://all3dp.com/1/cura-tutorial-software-slicer-cura-3d/).  

### Electronic Components

To build the switch PCB you can follow [this](switch#pcb) tutorial we written
