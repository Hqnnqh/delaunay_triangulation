pub mod delaunay;
pub mod geometry;
pub mod voronoi;

#[derive(Clone, Debug)]
pub struct TriangulationError;

impl std::fmt::Display for TriangulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DelaunayError occurred")
    }
}
