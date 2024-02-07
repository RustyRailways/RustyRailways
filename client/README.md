# Rusty Railways Client

This is a [Godot](https://godotengine.org/) based client to create the train track and control/see the status of the train system.

It lets you **Create** and **Edit** the **Train Track**, place the **NFC Tags**, **Trains** and **upload** (currently broken) everything to the Controller,
**Control** and **Monitor** the **Trains** and **Switches**.

### Screenshots
![example-train-track](./screenshots/example-train-track.png)

https://github.com/MrLakige/rusty_railways/assets/116217951/06c1ccc1-122a-48b4-8d96-e06f8fb9ee58

https://github.com/MrLakige/rusty_railways/assets/116217951/4c1d8071-14fc-486d-a815-6086c24b4450

![available-train-track-pieces](./screenshots/available-train-track-pieces.png)

### Main Components
- Main (UI) ([Main.gd](./Main.gd) and [Main.tscn](./Main.tscn))
  - Create and Edit the Train Track, set IDs and Master IP

- HTTPIO ([HTTPIO.gd](./HTTPIO.gd))
  - Send move requests to Master (using HTTP with [this standard](./../master/src/high_level_controller#requests))

- Updaters ([TrainUpdatePosition.gd](./trains/TrainUpdatePosition.gd), [TrainUpdateStatus.gd](./trains/TrainUpdateStatus.gd), [TrainUpdateSpeed.gd](./trains/TrainUpdateSpeed.gd), [SwitchStateUpdater.gd](./trains/SwitchStateUpdater.gd))
  - Pools Train Positions, States, Speeds and Switch States data from Master (using HTTP with [this standard](./../master/src/high_level_controller#requests))

### Installation (from pre-made builds)
Download the latest master build for your OS from the [latest release page](https://github.com/MrLakige/rusty_railways/releases/latest) under `Assets` and launch it, the following builds are currently available:

- Linux x86_64
- Windows x86_64
- MacOS x86_64

https://github.com/MrLakige/rusty_railways/releases/download/V0.1/linux-x86_64-rusty-railways-client.x86_64

If you need a build for a different system and/or architecture please follow the [Installation (from source)](#Installation (from source)) section.

### Installation (from source)
Install the latest Godot 4 Version ([Steam App](https://store.steampowered.com/app/404790/Godot_Engine/), [Standalone Installer](https://godotengine.org/download/windows/), [Flatpak (Linux only)](https://flathub.org/apps/org.godotengine.Godot) or [WinGet (Windows only)](https://winstall.app/apps/GodotEngine.GodotEngine))

Clone the repository with 
```bash
git clone https://github.com/MrLakige/rusty_railways.git
```

Open 
```bash
./rusty_railways/client/project.godot
```

Go to `Project` -> `Export...` and choose one of the pre-made `Presets` or use `Add...` to add a new custom preset with a different system and/or architecture of your choice.
