use std::cmp::Ordering;

#[derive(PartialEq)]
pub enum Triangle {
    Equilateral,
    Scalene,
    Isosceles,
}

impl Triangle {
    pub fn build<T>(mut sides: [T; 3]) -> Option<Triangle>
    where
        T: PartialOrd + Copy + Default + std::ops::Add<Output = T>,
    {
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        if sides.iter().any(|&side| side <= T::default()) || sides[2] > sides[0] + sides[1] {
            return None;
        }

        match (
            sides[0] == sides[1],
            sides[1] == sides[2],
            sides[2] == sides[0],
        ) {
            (false, false, false) => Some(Triangle::Scalene),
            (true, true, true) => Some(Triangle::Equilateral),
            _ => Some(Triangle::Isosceles),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        *self == Triangle::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        *self == Triangle::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        *self == Triangle::Isosceles
    }
}
