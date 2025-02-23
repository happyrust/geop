#[cfg(test)]
mod tests {
    use geop_geometry::{efloat::EFloat64, point::Point, surfaces::SurfaceLike};
    use geop_topology::{
        primitive_objects::{
            edges::line::primitive_line,
            faces::{cylinder::primitive_cylinder, sphere::primitive_sphere},
        },
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_surface_log_operation_unit_x(#[future] renderer: Box<HeadlessRenderer>) {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);
        let face = primitive_sphere(Point::zero(), EFloat64::from(1.0));
        scene.faces.push((face.clone(), Color::light_gray()));

        let anchor = Point::from_f64(-0.5, -1.3, 0.5).normalize().unwrap();
        scene.points.push((anchor, Color::blue()));

        for p in face.surface.point_grid(2.0) {
            assert!(face.surface.on_surface(p));
            if face.edge_from_to(anchor, p).length().unwrap() < 1.0 {
                scene.points.push((p, Color::green()));
                let log = face.surface.log(anchor, p).unwrap() + anchor;
                scene
                    .edges
                    .push((primitive_line(log, p).unwrap(), Color::white()));
                scene.points.push((log, Color::red()));
            }
        }

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/log_exp_map.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_surface_log_operation_2(#[future] renderer: Box<HeadlessRenderer>) {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);
        let face = primitive_cylinder(Point::zero(), Point::unit_z(), EFloat64::from(1.0));
        scene.faces.push((face.clone(), Color::light_gray()));

        let mut anchor = Point::from_f64(-0.5, -1.3, 0.0).normalize().unwrap();
        anchor.z = EFloat64::from(-0.5);
        scene.points.push((anchor, Color::blue()));

        for p in face.surface.point_grid(2.0) {
            assert!(face.surface.on_surface(p));
            if (anchor - p).norm() < 1.5 && (anchor - p).norm() > 0.0001 {
                scene.points.push((p, Color::green()));
                let log = face.surface.log(anchor, p).unwrap() + anchor;
                if (log - p).norm() >= 0.0 {
                    scene
                        .edges
                        .push((primitive_line(log, p).unwrap(), Color::white()));
                }
                scene.points.push((log, Color::red()));
            }
        }

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/log_exp_map2.png"),
            )
            .await;
    }
}
