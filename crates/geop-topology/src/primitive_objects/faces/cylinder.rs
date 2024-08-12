use std::rc::Rc;

use geop_geometry::{
    points::point::Point,
    surfaces::{cylinder::Cylinder, surface::Surface},
};

use crate::topology::face::Face;

pub fn primitive_cylinder(basis: Point, extend_dir: Point, radius: f64) -> Face {
    Face::new(
        None,
        vec![],
        Rc::new(Surface::Cylinder(Cylinder::new(
            basis, extend_dir, radius, true,
        ))),
    )
}