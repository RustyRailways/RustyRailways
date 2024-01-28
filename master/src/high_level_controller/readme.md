# High level controller

The high level controller is the component that communicate with the GUI and control the
path finder and scheduler.

## Requests

The high level controller spawn a web server that can receive requests from the GUI.  
All the requests will be done at at the ip `IP_MASTER` and port `MASTER_PORT_VISUALIZER` (see the [common infrastructure](../../../common_infrastructure/src/urls.rs)).

The handler supported are:

 - `POST /add_request`: add a new request to the scheduler  
   examples:
    - `{"MoveTrain": ["T1","P3"]}`
    - `{"LockTrain": ["T1"]}`
    - `{"UnlockTrain": ["T1"]}`
 - `GET /train_position/{id}` return the position of the train  
   example: 
   - `"P1"`
 - `GET /train_status/{id}` return the status of the train,  
    examples: 
   - `"Locked"`
   - `"Unlocked"`
   - `"Moving"`
 - `GET /train_speed/{id}` return the speed of the train, as a signed 8-bit integer  
    example: 
   - `10`
 - `GET /switch_position/{id}` return the position of the switch  
    examples: 
   - `"Straight"`
   - `"Diverted"`

Note 1: `id` is an integer, and is the id of the train or switch.
for example `id=1` is equal to the train `T1` or the switch `S1`.

Note 2: All the get methods can return null if the train or switch is not found. 

## Response
 All the response are in TEXT format, if an error occurs the code will be `500` and the body will be the error message.