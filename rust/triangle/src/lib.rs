pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|&side| side <= 0) {
            return None;
        }

        if 2 * sides.iter().max().unwrap() >= sides.iter().sum() {
            return None;
        }

        Some(Triangle {
            a: sides[0],
            b: sides[1],
            c: sides[2],
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c && self.c == self.a
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.c != self.a
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.c == self.a
    }
}
