use crate::TriangulationError;

#[derive(Copy, Clone, Debug, Eq)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(vertices: [Point; 3]) -> Self {
        Triangle { a: vertices[0], b: vertices[1], c: vertices[2] }
    }

    pub fn points(&self) -> [Point; 3] {
        [self.a, self.b, self.c]
    }

    pub fn a(&self) -> Point { self.a }
    pub fn b(&self) -> Point { self.b }
    pub fn c(&self) -> Point { self.c }
    pub fn edges(&self) -> [Edge; 3] {
        [Edge::new(self.a, self.b), Edge::new(self.b, self.c), Edge::new(self.c, self.a)]
    }
    pub(crate) fn point_in_circumcircle(&self, point: &Point) -> Result<bool, TriangulationError> {
        if let Ok(((ux, uy), r)) = self.circumcircle() {
            let dx = point.x() as f64 - ux;
            let dy = point.y() as f64 - uy;

            let distance = (dx * dx + dy * dy).sqrt();

            Ok(distance <= r)
        } else {
            Err(TriangulationError)
        }
    }

    pub(crate) fn circumcircle(&self) -> Result<((f64, f64), f64), TriangulationError> {
        let ax = self.a.x() as f64;
        let ay = self.a.y() as f64;
        let bx = self.b.x() as f64;
        let by = self.b.y() as f64;
        let cx = self.c.x() as f64;
        let cy = self.c.y() as f64;

        let d = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));

        if d == 0.0 {
            return Err(TriangulationError);
        }

        let ux = ((ax * ax + ay * ay) * (by - cy) + (bx * bx + by * by) * (cy - ay) + (cx * cx + cy * cy) * (ay - by)) / d;
        let uy = ((ax * ax + ay * ay) * (cx - bx) + (bx * bx + by * by) * (ax - cx) + (cx * cx + cy * cy) * (bx - ax)) / d;
        let r = ((ax - ux).powi(2) + (ay - uy).powi(2)).sqrt();

        Ok(((ux, uy), r))
    }

    pub(crate) fn contains_edge(&self, other_edge: &Edge) -> bool {
        for edge in self.edges() {
            if &edge == other_edge {
                return true;
            }
        }

        false
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        let mut self_sides = [self.a, self.b, self.c];
        let mut other_sides = [other.a, other.b, other.c];

        // Sort the sides
        self_sides.sort_unstable();
        other_sides.sort_unstable();

        // Compare the sorted sides
        self_sides == other_sides
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn x(&self) -> i32 { self.x }

    pub fn y(&self) -> i32 { self.y }
}

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    a: Point,
    b: Point,
}


impl Edge {
    pub fn new(a: Point, b: Point) -> Self {
        Edge { a, b }
    }

    pub fn a(&self) -> Point { self.a }

    pub fn b(&self) -> Point { self.b }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b || self.a == other.b && self.b == other.a
    }
}
