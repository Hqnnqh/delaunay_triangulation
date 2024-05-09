use crate::geometry::{Edge, Point, Triangle};
use crate::TriangulationError;

pub fn voronoi(delaunay: &Vec<Triangle>) -> Result<Vec<Edge>, TriangulationError> {
    if delaunay.is_empty() {
        return Err(TriangulationError);
    }

    let mut diagram: Vec<Edge> = Vec::default();

    for triangle in delaunay {
        let circumcenter = triangle.circumcircle()?;

        // get neighbors of triangle
        let mut neighbors: Vec<&Triangle> = Vec::default();
        for edege in triangle.edges() {
            delaunay.iter().for_each(|other| if other.contains_edge(&edege) && other != triangle && !neighbors.contains(&other) { neighbors.push(other); })
        }

        // connect circumecenters of neighboring triangles
        for neighbor in neighbors {
            let neighbor_circumcenter = neighbor.circumcircle()?;
            diagram.push(Edge::new(Point::new(circumcenter.0.0 as i32, circumcenter.0.1 as i32), Point::new(neighbor_circumcenter.0.0 as i32, neighbor_circumcenter.0.1 as i32)));
        }
    }

    Ok(diagram)
}