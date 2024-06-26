<p align="center">
    <img alt="Rusty Railways Logo" src="imgs/logo.png" style="width: 20%;">
</p>
<h3 align="center">Rust and Godot Based Autonomous Lego Train System</h3>
<br/>
<p align="center">
    <img alt="Rusty Railways Railtrack 2" src="https://github.com/RustyRailways/RustyRailways/assets/116217951/8ebff71d-9b64-48a1-86f5-50a38c5b9bbc" style="width: 71%;">
    <img alt="Rusty Railways Server and Client Station 2" src="https://github.com/RustyRailways/RustyRailways/assets/116217951/352528a2-75bf-400b-add8-0c4c2b1afb42" style="width: 27%;;">
</p>

## Description

Group Project by [**lucaSartore** (Luca Sartore)](https://github.com/lucaSartore), [**LorenzoCattai** (Lorenzo Cattai)](https://github.com/LorenzoCattai), [**MrLakige** (Michele Callegari)](https://github.com/MrLakige), [**Technorico** (Federico Peruzzo)](https://github.com/Technorico)
for [Embedded Software for The Internet of Things - 145996](https://unitn.coursecatalogue.cineca.it/insegnamenti/2025/50459_641197_95013/2019/50459/10712?coorte=2024&schemaid=8535) taught by professor [Kasim Sinan Yildirim](https://webapps.unitn.it/du/it/Persona/PER0212812/Didattica) at [University of Trento, DISI Department](https://www.disi.unitn.it/).

This is a system that automates the motion of Lego trains and rail switches.
You can tell the system in which station you want a train to go (either using an HTTP request or using our GUI),
and the system will figure out the best way to achieve the goal, even moving trains that are on your
way if necessary.

## Presentation Video

[![Presentation Video](https://img.youtube.com/vi/J4SjKY6N7vM/0.jpg)](https://youtu.be/J4SjKY6N7vM)

## Components

The system is implemented using 5 different components:

 - `The Master`: This is the component that does all the calculations to find the optimal path, and then controls the trains and the switches, you can learn more about the master [here](./master/readme.md)
 - `The Client`: ([Godot](https://godotengine.org/) based client to create the train track and control/monitor the train system), you can learn more about the client [here](./client/README.md)
 - `The Map`: The map is a support data structure (a modified graph in particular) used by the master, you can learn more about the master [here](./map/readme.md)
 - `The Common Infrastructure`: The common infrastructure defines some basic messages that all components need to communicate with each other, as well as the IPs and the hardware abstraction layer traits, you can learn more about the master [here](./common_infrastructure/readme.md)
 - `The Trains`: You can learn more about the code that manages the trains [here](./train/rust/readme.md)
 - `The Switches`: You can learn more about the code that manages the switches [here](./switch/readme.md)

## Documentation

- [Common infrastructure documentation](https://mrlakige.github.io/rusty_railways/common_infrastructure/doc/common_infrastructure/index.html)
- [Map documentation](https://mrlakige.github.io/rusty_railways/map/doc/map/index.html)
- [Master documentation](https://mrlakige.github.io/rusty_railways/master/doc/master/index.html)
- [Railway sim map documentation](https://mrlakige.github.io/rusty_railways/railway_sim_map/doc/railway_sim_map/index.html)<br>
- [Switch documentation](https://mrlakige.github.io/rusty_railways/switch/html/index.html)<br>
- [Train documentation](https://mrlakige.github.io/rusty_railways/train/doc/locomotive/index.html)<br>

## Installation

Here you can find a quick list of the things you need to do, to try our project yourself.

### Master Installation
The current implementation of the master runs on a Linux system (e.g. Raspberry Pi with Raspberry Pi OS) so to run it, you just need to install the [regular rust toolchain](https://www.rust-lang.org/tools/install).
Given that we used a Hardware Abstraction Layer (HAL) to implement the master, with a bit more work, it can be executed on something smaller (like an ESP-32), if you are interested in this you can check out the train toolchain installation down below. 

### Train Toolchain Installation
The train code is written in embedded rust, with the [embedded-hal](https://github.com/rust-embedded/embedded-hal) and [embedded-svc](https://github.com/esp-rs/embedded-svc/tree/master) crates. To install the software needed to build for our target (The ESP-32) you can follow the tutorial [here](https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites).

### Switch Toolchain Installation
To compile the switch code you can follow [this](./switch/readme.md) tutorial we have written.

### Client Installation
See [Client README.md](https://github.com/MrLakige/rusty_railways/blob/main/client/README.md).

### 3D Printing
Files to print the switch mechanism are located [here](./switch/hardware/Switch_3D_print/)
You can find a 3D printing tutorial [here](https://all3dp.com/1/cura-tutorial-software-slicer-cura-3d/).  

### Electronic Components

To build the switch PCB you can follow [this](switch#pcb) tutorial we have written

## Additional Videos

https://github.com/RustyRailways/RustyRailways/assets/116217951/42091e8f-f878-42e6-8666-56fedade8acc

https://github.com/RustyRailways/RustyRailways/assets/116217951/6e29771f-3cac-484c-83f4-8b54dd48968c

https://github.com/RustyRailways/RustyRailways/assets/116217951/05d6a2de-c693-4c84-9944-1e6514dbb60c

https://github.com/RustyRailways/RustyRailways/assets/116217951/6d81c1b0-d530-42d4-b9ea-9f0555a467a0

https://github.com/RustyRailways/RustyRailways/assets/116217951/e64ef0ef-5b17-43a4-998a-6358e84ad2e4

### Special Thanks

[FabLab UniTn](https://fablab.unitn.it/) for the space, tools and part of the materials used
