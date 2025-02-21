use crate::geometry::{line::Line, points::Point};

pub struct Node<'a, P: Point> {
    id: usize, 
    north_face: Line<'a, P>,
    south_face: Line<'a, P>,
    east_face:  Line<'a, P>,
    west_face:  Line<'a, P>,
}