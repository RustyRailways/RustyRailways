# Map

The map is basically a wrapper structure for a graph with some specific features that are used for our implementation

In the graph a node represent a position, (aka an NFC tag) in the real world.

The tags are used for:
 - Tel the train when to stop for a station
 - Track the position of the train and stop if it takes the wrong path
 - Regulate the train speed for tracks with different slope/curvature radius

A link in a map contains the length of the link, and the max speed the train is allowed to go in that track. The speed can be different depending on the direction the train is going (this is useful for example in a slope)

## Implementation

### The Initialized-Uninitialized States

The map is written using a generic-structure, where the state are `Initialized` or `Uninitialized`.
Inside the graph some `References` are used point to `Nodes`, `Trains` and `Switches`.
The [references](references/mod.rs) can also have two states (`Initialized` or `Uninitialized`).  
To understand this distinction let's make an example with a reference to the position `P1`  

In the case of am Uninitialized map a reference will just contains an enum
`Position::P1` This is good for simplicity, as it avoid having to deal with pointers, it also us to make the map structure serializable and deserializable to json using `serde` and `serde/json`.
The only problem is that the performances are not as good compared with 
having a pointer/reference.  

In the case of an Initialized map the reference is replaced with a raw pointer, that allows for quick access to the item referenced.

#### The use of unsafe

We waned to derive the serialize/deserialize trait, and this can only be done with the uninitialized state of the map, as we can't deserialize an address in memory.  

Unfortunately as of today (02/2024) is impossible to derive the serialize/deserialize for Map\<Uninitialized> and not for Map\<Initialized>. To avoid this we have implemented manually the serialize/deserialize for the initialized reference, with just a panic that explain that the function should never be used.

Then we had to use raw pointers instead of references inside the initialized pointer, because using a lifetime would prevent us for implementing the deserialize trait

## Map views

The map has many functionalities, to simplify the life of the user some `Views` has been created, in particular:
 - [Map creation view:](src/views/map_creation_view.rs) is a view used to quickly and easily build a map
 - [Map controller view:](src/views/map_controller_view.rs) is a view designed to control the map (e.g. move the train/the switches...)
 - [Map visualization view:](src/views/map_visualization_view.rs) is a view designed to give information about the map, like having the speed and status of trains
 - [Map navigation view:](src/views/map_visualization_view.rs) exposes some high level APIs to navigate the map as a graph, and find the best path
 - [Map factory:](src/views/map_factory.rs) is a factory that allow to create different views.

