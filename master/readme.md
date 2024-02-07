# Master

The master is the component that gets a request (e.g. Send train T1 to position P4) and figures out the way to execute it.

The master is divided into 3 different components:

 - High-level controller
 - Path finder and scheduler
 - Low-level controller

 ## High level controller

 This component is a web server that allows for a GUI to send moving requests and to get information about the status of the system.

 ## Path finder and scheduler

 This component receives a request from the high-level controller and finds the ideal path to satisfy it.
 It also avoids collision between trains and can decide autonomously what to do if a train is in the middle of the path.

 ## Low level controller

 This component receives from the path-finder and scheduler a list of positions and sends the messages to the trains and switches to achieve the desired movement
