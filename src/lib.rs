mod delaunay;

#[cfg(test)]
mod tests {
    use raylib::prelude::*;
    use crate::delaunay::bowyer_watson;

    #[test]
    fn test() {
        let points = [(103, 340), (134, 200), (301, 400), (243, 231), (143, 133)];

        const WINDOW_WIDTH: i32 = 450;
        const WINDOW_HEIGHT: i32 = 450;
        const RADIUS: f32 = 5.0;

        let (mut rl, thread) = init().size(WINDOW_WIDTH, WINDOW_HEIGHT).title("Delaunay Test").build();

        rl.set_target_fps(60);
        let points = bowyer_watson(points.to_vec(), WINDOW_WIDTH, WINDOW_HEIGHT);

        while !rl.window_should_close() {
            let mut drawing_handle = rl.begin_drawing(&thread);
            drawing_handle.clear_background(Color::DARKGRAY);

            for point in points {
                drawing_handle.draw_circle(point.0, point.1, RADIUS, Color::RED);
            }
        }
    }
}
