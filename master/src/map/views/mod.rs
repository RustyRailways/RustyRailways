/// the controller is use to set the switch, the train speed, and move the trains arround
pub mod map_controller_view;
/// the navigation view is use to navigate the graph
pub mod map_navigation_view;
/// the visualization view is use get information about the status of the components
pub mod map_visualization_view;
/// the creation view is use to create the graph
pub mod map_creation_view;
/// allows to build the views
pub mod map_factory;

pub use map_controller_view::MapControllerView;
pub use map_navigation_view::MapNavigationView;
pub use map_visualization_view::MapVisualizationView;
pub use map_creation_view::MapCreationView;
pub use map_factory::MapFactory;

