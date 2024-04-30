use std::hash::{Hash, Hasher};

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

    pub(crate) fn circumcircle_contains_point(&self, point: &Point) -> bool {
        let ax = (self.a.x() - point.x()) as f32;
        let ay = (self.a.y() - point.y()) as f32;

        let bx = (self.b.x() - point.x()) as f32;
        let by = (self.b.y() - point.y()) as f32;

        let cx = (self.c.x() - point.x()) as f32;
        let cy = (self.c.y() - point.y()) as f32;

        let a_sq = ax * ax + ay * ay;
        let b_sq = bx * bx + by * by;
        let c_sq = cx * cx + cy * cy;

        let determinant = ax * (by * c_sq - cy * b_sq) -
            ay * (bx * c_sq - cx * b_sq) +
            a_sq * (bx * cy - cx * by);
        determinant <= 0.0
    }

    pub fn edges(&self) -> [Edge; 3] {
        [Edge::new(self.a, self.b), Edge::new(self.b, self.c), Edge::new(self.c, self.a)]
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

impl Hash for Triangle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
        self.c.hash(state);
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Copy, Clone, Debug, Eq)]
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

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
    }
}