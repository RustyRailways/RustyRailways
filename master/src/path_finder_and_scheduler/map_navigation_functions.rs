use common_infrastructure::Position;

pub mod path_to_position;
pub use path_to_position::get_path_to_position;
pub mod path_to_move_out_of_the_way;
pub use path_to_move_out_of_the_way::find_path_to_move_out_of_the_way;

//pub mod path_to_intersection;

pub mod path_finder;
#[derive(PartialEq, Eq, Debug)]
struct OrderedPosition{
    priority: u32,
    position: Position,
}

impl OrderedPosition{
    fn new(priority: u32, position: Position) -> Self{
        Self{
            priority,
            position,
        }
    }
}

impl PartialOrd for OrderedPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmp(other).into()
    }
}

impl Ord for OrderedPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
