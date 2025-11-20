pub struct Triangle {
    x: u64,
    y: u64,
    z: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sides = sides;
        sides.sort();
        if sides[0] + sides[1] <= sides[2] || sides[2] == 0 {
            return None;
        }
        Some(Triangle {
            x: sides[0],
            y: sides[1],
            z: sides[2],
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.x == self.y && self.y == self.z
    }

    pub fn is_scalene(&self) -> bool {
        return self.x != self.y && self.x != self.z && self.y != self.z
    }

    pub fn is_isosceles(&self) -> bool {
        if self.is_equilateral() {
            return false
        }
        self.x == self.y || self.x == self.z || self.y == self.z
    }
}
