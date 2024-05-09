use crate::geometry::{Edge, Point, Triangle};
use crate::TriangulationError;

pub fn bowyer_watson(points: &Vec<Point>) -> Result<Vec<Triangle>, TriangulationError> {
    if points.len() < 3 {
        return Err(TriangulationError);
    }

    let super_triangle = super_triangle(points.as_slice())?;

    let mut triangulation: Vec<Triangle> = Vec::from([super_triangle]);

    for point in points {
        let mut bad_triangles: Vec<Triangle> = Vec::default();

        // find all triangles that are no longer valid due to the insertion
        for triangle in &triangulation {
            if triangle.point_in_circumcircle(point)? {
                bad_triangles.push(*triangle);
            }
        }

        let mut polygon: Vec<Edge> = Vec::default();
        // find the boundary of the polygonal hole
        for triangle in &bad_triangles {
            for edge in triangle.edges() {
                // edge not shared by any other triangles
                if bad_triangles.iter().filter(|&triangle| triangle.contains_edge(&edge)).count() == 1 {
                    polygon.push(edge);
                }
            }
        }

        // remove bad triangles from triangulation
        triangulation.retain(|triangle| !bad_triangles.contains(triangle));

        // re-triangulate the polygonal hole
        for edge in polygon {
            let new_triangle = Triangle::new([edge.a(), edge.b(), *point]);
            triangulation.push(new_triangle);
        }
    }

    triangulation.retain(|triangle| !triangle.points().iter().any(|point| super_triangle.points().contains(point)));

    Ok(triangulation)
}

pub fn super_triangle(points: &[Point]) -> Result<Triangle, TriangulationError> {
    if points.len() < 3 {
        return Err(TriangulationError);
    }

    let large_factor = f32::INFINITY;

    // (min, max)
    let bounding_box: (Point, Point) = (Point::new(points.iter().map(|point| point.x()).min().unwrap(), points.iter().map(|point| point.y()).min().unwrap()), Point::new(points.iter().map(|point| point.x()).max().unwrap(), points.iter().map(|point| point.y()).max().unwrap()));

    let a = Point::new((((bounding_box.0.x() + bounding_box.0.y()) as f32) / 2.0) as i32, (bounding_box.0.y() as f32 - large_factor * ((bounding_box.1.y() - bounding_box.0.y()) as f32)) as i32);
    let b = Point::new((bounding_box.0.x() as f32 - (large_factor * ((bounding_box.1.x() - bounding_box.0.x()) as f32))) as i32, ((bounding_box.1.y() as f32) + large_factor * (bounding_box.1.y() - bounding_box.0.y()) as f32) as i32);
    let c = Point::new((bounding_box.1.x() as f32 + (large_factor * ((bounding_box.1.x() - bounding_box.0.x()) as f32))) as i32, ((bounding_box.1.y() as f32) + large_factor * (bounding_box.1.y() - bounding_box.0.y()) as f32) as i32);

    Ok(Triangle::new([a, b, c]))
}

