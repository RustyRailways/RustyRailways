# Master

The master is the component that gets a request (e.g. Send train 1 to position 4) and figure out the way to execute it.

the master is divided into 3 different components:

 - High level controller
 - Path finder and scheduler
 - Low level controller

 ## High level controller

 This component is a web server that allows for a GUI to send the moving requests and to get informations about the status of the system

 ## Path finder and scheduler

 This component receive a request from the high level controller and find the ideal path tho satisfy it.
 It also avoid collision between trains and can decide autonomously what to do if a train is in the middle of the path.

 ## Low level controller

 This component receive from the path finder and scheduler a list of positions, and send the messages to the trains and switches in order to achieve the desired movement