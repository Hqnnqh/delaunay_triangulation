mod delaunay;
mod geometry;

#[cfg(test)]
mod tests {
    use raylib::prelude::*;

    use crate::delaunay;
    use crate::geometry::{Point};

    #[test]
    fn test() {
        let mut points = Vec::from([Point::new(500, 340), Point::new(410, 200), Point::new(301, 400), Point::new(130, 300), Point::new(900, 100), ]);

        const WINDOW_WIDTH: i32 = 1200;
        const WINDOW_HEIGHT: i32 = 900;
        const RADIUS: f32 = 5.0;

        let (mut rl, thread) = init().size(WINDOW_WIDTH, WINDOW_HEIGHT).title("Delaunay Test").build();

        rl.set_target_fps(60);



        while !rl.window_should_close() {
            let super_triangle = delaunay::super_triangle(points.as_slice()).unwrap();
            let delaunay_triangulation = delaunay::bowyer_watson(&points).unwrap();



            let mut drawing_handle = rl.begin_drawing(&thread);
            drawing_handle.clear_background(Color::DARKGRAY);

            if drawing_handle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                points.push(Point::new(drawing_handle.get_mouse_position().x as i32, drawing_handle.get_mouse_position().y as i32));
            }

            for point in &points {
                drawing_handle.draw_circle(point.x(), point.y(), RADIUS, Color::RED);
            }

            drawing_handle.draw_triangle_lines(Vector2::from((super_triangle.a().x() as f32, super_triangle.a().y() as f32)), Vector2::from((super_triangle.b().x() as f32, super_triangle.b().y() as f32)), Vector2::from((super_triangle.c().x() as f32, super_triangle.c().y() as f32)), Color::BLUE);


            for triangle in &delaunay_triangulation {
                drawing_handle.draw_triangle_lines(Vector2::from((triangle.a().x() as f32, triangle.a().y() as f32)), Vector2::from((triangle.b().x() as f32, triangle.b().y() as f32)), Vector2::from((triangle.c().x() as f32, triangle.c().y() as f32)), Color::GREEN);
            }
        }
    }


}
