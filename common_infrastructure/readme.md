# Common infrastructure

This crate define some common components that are in use in all the other creates, in particular:

 - `Position` is an enum that represent all of the possible position the train can be in, in reality the position is an RFID tag that can be read from an NFC reader, so the enum can be created from an array of 4 digits.

 - `Train` is an enum that represent all the trains
 - `Switch` is an enum that represent all possible switch that there can be in the map
- `urls` is a module that defines all the IPs and URLs of the components involved in the system
- `hals` is a module that defines some trait for the components involved in the system, this trait are used to abstract the hardware, to make it possible to run the same main code on different hardware configurations. There are 3 main HALs defined:
  - `GenericHal` is the hal that defines the basic functionalities, and all of the other HALs inherit from this.
  - `MasterHal` is the hal for the master/brain of the operation
  - `TrainHal` is the hal for the train
  - `HeartBeatManagerHal` is an hal for an optional component that can be used for additional safety that allow the system to detect if a component is disconnected. This is in use in the current implementation of the master and the train
  
Note: the switch dose not have an HAL because our switches run on  `ESP-01s` that are not supported in embedded rust.