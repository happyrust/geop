use std::fmt::Debug;

use crate::{points::point::Point, transforms::Transform};

use super::{
    circle::{Circle, CircleTransform},
    line::Line,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Curve {
    Line(Line),
    Circle(Circle),
}

// This represents a curve, which can be a line or a circle.
impl Curve {
    // Transform
    pub fn transform(&self, transform: Transform) -> Curve {
        match self {
            Curve::Line(line) => Curve::Line(line.transform(transform)),
            Curve::Circle(circle) => match circle.transform(transform) {
                CircleTransform::Circle(circle) => Curve::Circle(circle),
                CircleTransform::Ellipse() => todo!("Implement this"),
            },
        }
    }

    // Change the direction of the curve
    pub fn neg(&self) -> Curve {
        match self {
            Curve::Line(line) => Curve::Line(line.neg()),
            Curve::Circle(circle) => Curve::Circle(circle.neg()),
        }
    }

    // Normalized Tangent / Direction of the curve at the given point.
    pub fn tangent(&self, p: Point) -> Point {
        match self {
            Curve::Line(line) => line.tangent(p),
            Curve::Circle(circle) => circle.tangent(p),
        }
    }

    // Checks if point is on the curve.
    pub fn on_curve(&self, p: Point) -> bool {
        match self {
            Curve::Line(line) => line.on_curve(p),
            Curve::Circle(circle) => circle.on_curve(p),
        }
    }

    // Interpolate between start and end at t. t is between 0 and 1.
    pub fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match self {
            Curve::Line(line) => line.interpolate(start, end, t),
            Curve::Circle(circle) => circle.interpolate(start, end, t),
        }
    }

    // Checks if m is between x and y. m==x and m==y are true.
    pub fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool {
        match self {
            Curve::Line(line) => line.between(m, start, end),
            Curve::Circle(circle) => circle.between(m, start, end),
        }
    }

    // Get the midpoint between start and end. Not that this is well defined even on a circle, because the midpoint is between start and end.
    // If start or end is None, the midpoint is a point that is a unit distance away from the other point.
    pub fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point {
        match self {
            Curve::Line(line) => line.get_midpoint(start, end),
            Curve::Circle(circle) => circle.get_midpoint(start, end),
        }
    }

    pub fn project(&self, p: Point) -> Point {
        match self {
            Curve::Line(line) => line.project(p),
            Curve::Circle(circle) => circle.project(p),
        }
    }
}
