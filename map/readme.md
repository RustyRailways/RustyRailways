# Map

Basically, the map is a wrapper structure for a graph with some specific features that are used for our implementation.

In the graph, a node represents a position, (aka an NFC tag) in the real world.

The tags are used for:
 - Tel the train when to stop for a station
 - Track the position of the train and stop if it takes the wrong path
 - Regulate the train speed for tracks with different slope/curvature radius

A link in a map contains the length of the link and the maximum speed the train is allowed to reach on that track. The speed can be different depending on the direction the train is going (this is useful for example in a slope).

## Implementation

### The Initialized-Uninitialized States

The map is written using a generic-structure, where the generic parameters _S_ represent the states (that can be `Initialized` or `Uninitialized`).
Inside the graph, some `References` are used to point to `Nodes`, `Trains`, and `Switches`.
In the map, a reference is a structure that points to an object (e.g. a train has inside a reference to the station it is currently on).
The [references](references/mod.rs) can also have two states (`Initialized` or `Uninitialized`).  
To understand this distinction let's make an example with a reference to the position `P1`  

In the case of an Uninitialized map, a reference will just contain an enum
`Position::P1`. This is good for simplicity, as it avoids having to deal with pointers, it also allows us to make the map structure serializable and deserializable to JSON using `serde` and `serde/json`.
The only problem is that the performances are not as good compared with 
having a pointer/reference.

In the case of an Initialized map, the reference is replaced with a raw pointer, that allows for quick access to the item referenced.

#### The use of unsafe

We wanted to derive the serialize/deserialize trait, and this can only be done with the uninitialized state of the map, as we cannot deserialize a reference/pointer to a variable.  

Unfortunately as of today (02/2024), due to Rust characteristics and implementation, is impossible to decide to derive the serialize/deserialize (or any other Trait) only for a version with type _T_ of a generic structure (e.g. talking about our project we cannot derive _serialize_/_deserialize_ for Map\<_Uninitialized_> without also deriving it for Map\<_Initialized_>). To avoid this we have implemented manually the serialize/deserialize for the initialized reference, with just a panic that explains that the function should never be used.
Then we had to use raw pointers instead of references inside the initialized Map reference because using a lifetime would prevent us from implementing the deserialize trait.

## Map views

The map has many functionalities, to simplify the life of the user some `Views` have been created, in particular:
 - [Map creation view:](src/views/map_creation_view.rs) is a view used to quickly and easily build a map
 - [Map controller view:](src/views/map_controller_view.rs) is a view designed to control the map (e.g. move the train/the switches...)
 - [Map visualization view:](src/views/map_visualization_view.rs) is a view designed to give information about the map, like having the speed and status of trains
 - [Map navigation view:](src/views/map_visualization_view.rs) exposes some high-level APIs to navigate the map as a graph and find the best path
 - [Map factory:](src/views/map_factory.rs) is a factory that allow to create different views.

