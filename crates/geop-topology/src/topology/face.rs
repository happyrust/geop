use std::rc::Rc;

use geop_geometry::{surfaces::{plane::Plane, sphere::Sphere, surface::Surface}, points::point::Point, curves::line::Line};

use crate::{PROJECTION_THRESHOLD, topology::edge::{Direction, EdgeCurve, EdgeIntersection}};

use super::{{edge_loop::EdgeLoop, edge::Edge}, vertex::Vertex};


#[derive(PartialEq, Clone, Debug)]
pub enum FaceSurface {
    Plane(Plane),
    Sphere(Sphere),
}
impl FaceSurface {
    pub fn surface(&self) -> &dyn Surface {
        match self {
            FaceSurface::Plane(plane) => plane,
            FaceSurface::Sphere(sphere) => sphere,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Face {
    pub boundaries: Vec<EdgeLoop>,
    // pub inner_loops: Vec<EdgeLoop>,
    pub surface: Rc<FaceSurface>,
    // convex_boundary: EdgeLoop, // TODO: Probably not needed
    // center_point: Point, // TODO: Probably not needed
}

pub enum FaceIntersection {
    Face(Face),
    EdgeLoop(EdgeLoop),
    Edge(Edge),
    Vertex(Vertex)
}

// Every face is homeomorphic to a disk or a square, hence we can use a parametrization of the form (u, v) \in [0, 1]^2.
// We will assert that the Face is shaped such that there is a midpoint, and each line from the midpoint to the boundary is within the face.
// The centerpoint cannot be on the boundary, and the boundary cannot intersect itself.
impl Face {
    pub fn new(boundaries: Vec<EdgeLoop>, surface: Rc<FaceSurface>) -> Face {
        Face {
            boundaries,
            surface,
        }
    }

    // pub fn point_at(&self, u: f64, v: f64) -> Point {
    //     let anchor_point_1 = self.outer_loop.point_at(u);
    //     let anchor_point_2 = self.center_point;
    //     match &*self.surface {
    //         FaceSurface::Plane(plane) => {
    //             anchor_point_1 + (anchor_point_2 - anchor_point_1) * v
    //         },
    //         FaceSurface::Sphere(sphere) => {
    //             let axis = (anchor_point_1 - sphere.basis).cross(anchor_point_2 - sphere.basis).normalize();
    //             let angle = (anchor_point_1 - sphere.basis).angle(anchor_point_2 - sphere.basis);
    //             sphere.basis + axis.rotate(anchor_point_1 - sphere.basis, angle * v)
    //         },
    //     }
    // }

    // pub fn project(&self, p: &Point) -> (f64, f64) {
    //     match &*self.surface {
    //         FaceSurface::Plane(plane) => {
    //             let direction = *p - self.center_point;
    //             let anchor_point = self.outer_loop.intersect(Line::new(self.center_point, direction));
    //             let u = self.outer_loop.project(&anchor_point).expect("Point not on boundary");
    //             let v = direction.norm() / (anchor_point - self.center_point).norm();
    //             (u, v)
    //         },
    //         FaceSurface::Sphere(sphere) => {
    //             todo!("Implement projection for sphere")
    //         },
    //     }
    // }

    pub fn intersect(&self, other: &Face) -> Vec<FaceIntersection> {
        todo!("Implement intersect");
    }

    pub fn contains(&self, other: &Point) -> bool {
        todo!("Implement contains");
    }

    // Splits this with an EdgeLoop if there are no Edges already. The EdgeLoop must be on the same surface as this Face.
    pub fn split_if_necessary(&self, other: &EdgeLoop) -> Option<Vec<Rc<Face>>> {
        let split_vertices = Vec::<Vertex>::new();
        for edge_other in other.edges {
            for edge_loop in &self.boundaries {
                for edge_self in &edge_loop.edges {
                    let intersections = edge_other.intersections(&edge_self);
                    for intersection in intersections {
                        match intersection {
                            EdgeIntersection::Edge(edge) => {
                                split_vertices.push(edge.start);
                                split_vertices.push(edge.end);
                            },
                            EdgeIntersection::Vertex(vertex) => {
                                split_vertices.push(vertex);
                            },
                        }
                    }
                }
            }
        }
        if split_vertices.len() == 0 {
            return None;
        }
        let split_edges = Vec::<Edge>::new();
        todo!("Implement split");
    }    
}

//     pub fn intersect(&self, other: &Face) {
//         if (self.surface.equals(&other.surface)) { // Results in a Face
//             // let outer_bounds = self.outer_loop.edges[0].split(other.outer_loop.edges[0]);
//             // for (edge1, edge2) in outer_bounds {
//             //     let inner_dir = cross_product(self.surface.normal(edge1.vertices[0]), edge1.tangent(edge1.vertices[1]));
//             //     let edge1_prod = dot_product(inner_dir, edge1.tangent(edge1.vertices[0]));
//             //     let edge2_prod = dot_product(inner_dir, edge2.tangent(edge2.vertices[0]));
//             //     if edge1_prod < edge2_prod {
//             //         // Keep edge1
//             //     } else {
//             //         // Keep edge2
//             //     }
//             // }
//         }
//         // Results in a line
//         let intersection_curve = self.surface.intersect(&other.surface);

//         let outer_bounds = intersection_curve.intersections(self.outer_loop);

//         let inner_bounds = self.inner_loops[0].edges[0].intersections(intersection_curve);
//     }

//     pub fn split(&self, other: &Face) {
//         let intersection_curve = self.surface.intersect(&other.surface);
//         let outer_bounds = intersection_curve.intersections(self.outer_loop);
//         let inner_bounds = self.inner_loops[0].edges[0].intersections(intersection_curve);
//     }

//     pub fn union(&self, other: &Face) {
//         assert!(self.surface.equals(&other.surface));
//     }
//     pub fn difference(&self, other: &Face) {
//         assert!(self.surface.equals(&other.surface));
//     }
//     pub fn intersection(&self, other: &Face) {
//         assert!(self.surface.equals(&other.surface));
//     }
// }