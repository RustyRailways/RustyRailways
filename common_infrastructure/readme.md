# Common infrastructure

This crate defines some common components that are in use in all the other creates, in particular:

 - `Position` is an enum that represents all of the possible positions the train can be on, in reality, the position is identified by an RFID tag that can be read from an NFC reader, so the enum can be created from an array of 4 bytes.

 - `Train` is an enum that represents all the trains
 - `Switch` is an enum that represents all the switches in the map
- `urls` is a module that defines all the IPs and URLs of the components involved in the system
- `hals` is a module that defines some traits for the components involved in the system, these traits are used to abstract the hardware, to make it possible to run the same code on different hardware configurations. There are 3 main _HAL_s defined:
  - `GenericHal` is the _HAL_ that defines the basic functionalities, and all of the other HALs inherit from this.
  - `MasterHal` is the _HAL_ for the master/brain of the operation
  - `TrainHal` is the _HAL_ for the train
  - `HeartBeatManagerHal` is a _HAL_ for an optional component that can be used for additional safety that allows the system to detect if a component is disconnected. This is not in use in the current implementation of the master and the train
  
Note: the switch does not have a _HAL_ because our switches run on `ESP-8266` that are not supported in embedded rust.
