use crate::{points::point::Point, transforms::Transform, EQ_THRESHOLD};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Ellipsis {
    pub basis: Point,
    pub normal: Point,
    pub major_radius: Point,
    pub minor_radius: Point,
}

impl Ellipsis {
    pub fn new(basis: Point, normal: Point, major_radius: Point, minor_radius: Point) -> Ellipsis {
        let normal = normal.normalize();
        assert!(
            normal.dot(major_radius).abs() < EQ_THRESHOLD,
            "Major radius and normal must be orthogonal"
        );
        assert!(
            normal.dot(minor_radius).abs() < EQ_THRESHOLD,
            "Minor radius and normal must be orthogonal"
        );
        assert!(
            major_radius.dot(minor_radius).abs() < EQ_THRESHOLD,
            "Major and minor radii must be orthogonal"
        );
        Ellipsis {
            basis,
            normal,
            major_radius,
            minor_radius,
        }
    }

    fn transform(&self, transform: Transform) -> Ellipsis {
        let basis = transform * self.basis;
        let normal = transform * (self.normal + self.basis) - basis;
        let major_radius = transform * (self.major_radius + self.basis) - basis;
        let minor_radius = transform * (self.minor_radius + self.basis) - basis;
        Ellipsis::new(basis, normal, major_radius, minor_radius)
    }

    pub fn neg(&self) -> Ellipsis {
        Ellipsis::new(
            self.basis,
            -self.normal,
            self.major_radius,
            self.minor_radius,
        )
    }
}

impl CurveLike for Ellipsis {
    fn transform(&self, transform: Transform) -> Curve {
        Curve::Ellipsis(self.transform(transform))
    }

    fn neg(&self) -> Curve {
        Curve::Ellipsis(self.neg())
    }

    fn tangent(&self, p: Point) -> Point {
        assert!(self.on_curve(p));
        let p = p - self.basis;
        let x = self.major_radius.dot(p) / self.major_radius.norm();
        let y = self.minor_radius.dot(p) / self.minor_radius.norm();
        let tangent = y * self.major_radius - x * self.minor_radius;
        tangent.normalize()
    }

    fn on_curve(&self, p: Point) -> bool {
        let p = p - self.basis;
        let x = self.major_radius.dot(p) / self.major_radius.norm();
        let y = self.minor_radius.dot(p) / self.minor_radius.norm();
        (p.dot(self.normal).abs() < EQ_THRESHOLD)
            && ((x.powi(2) + y.powi(2) - 1.0).abs() < EQ_THRESHOLD)
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let angle = (x - self.basis).angle(y - self.basis);
        self.major_radius.norm() * angle
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start = start - self.basis;
                let end = end - self.basis;
                let x_start = self.major_radius.dot(start);
                let x_end = self.major_radius.dot(end);
                let y_start = self.minor_radius.dot(start);
                let y_end = self.minor_radius.dot(end);
                let angle1 = y_start.atan2(x_start);
                let mut angle2 = y_end.atan2(x_end);
                if angle2 < angle1 {
                    angle2 += 2.0 * std::f64::consts::PI;
                }
                let angle = angle1 + t * (angle2 - angle1);
                angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis
            }
            (Some(start), None) => {
                let start = start - self.basis;
                let x_start = self.major_radius.dot(start);
                let y_start = self.minor_radius.dot(start);
                let angle1 = y_start.atan2(x_start);
                let angle = angle1 + t * std::f64::consts::PI * 2.0;
                angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis
            }
            (None, Some(end)) => {
                let end = end - self.basis;
                let x_end = self.major_radius.dot(end);
                let y_end = self.minor_radius.dot(end);
                let angle2 = y_end.atan2(x_end);
                let angle = angle2 + t * std::f64::consts::PI * 2.0;
                angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis
            }
            (None, None) => {
                let angle = t * std::f64::consts::PI * 2.0;
                angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis
            }
        }
    }

    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool {
        assert!(self.on_curve(m));
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start = start - self.basis;
                let end = end - self.basis;
                let m = m - self.basis;
                let angle_start = self
                    .minor_radius
                    .dot(start)
                    .atan2(self.major_radius.dot(start));
                let mut angle_end = self.minor_radius.dot(end).atan2(self.major_radius.dot(end));
                let mut angle_m = self.minor_radius.dot(m).atan2(self.major_radius.dot(m));
                if angle_end < angle_start {
                    angle_end += 2.0 * std::f64::consts::PI;
                }
                if angle_m < angle_start {
                    angle_m += 2.0 * std::f64::consts::PI;
                }
                angle_start <= angle_m && angle_m <= angle_end
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                true
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                true
            }
            (None, None) => true,
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start_rel = start - self.basis;
                let end_rel = end - self.basis;
                let mid = (start_rel + end_rel) / 2.0;
                let mid = mid.normalize() * self.major_radius.norm();
                let p1 = mid + self.basis;
                if self.between(p1, Some(start), Some(end)) {
                    return p1;
                } else {
                    return -mid + self.basis;
                }
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                return self.basis - (start - self.basis);
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                return self.basis - (end - self.basis);
            }
            (None, None) => {
                return self.basis + self.major_radius;
            }
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = v - self.normal * (v.dot(self.normal));
        v.normalize() * self.major_radius.norm() + self.basis
    }

    fn get_bounding_box(
        &self,
        interval_self: Option<Point>,
        midpoint_self: Option<Point>,
    ) -> crate::bounding_box::BoundingBox {
        todo!()
    }
}

// Implement partial equality for Ellipsis
impl PartialEq for Ellipsis {
    fn eq(&self, other: &Ellipsis) -> bool {
        self.basis == other.basis
            && self.normal == other.normal
            && self.major_radius == other.major_radius
            && self.minor_radius == other.minor_radius
    }
}