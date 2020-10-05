use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq)]
pub enum TupleType {
    Point,
    Vector,
}

#[derive(Debug, PartialEq)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub tuple_type: TupleType,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
            tuple_type: match w {
                _ if w == 1.0 => TupleType::Point,
                _ => TupleType::Vector,
            },
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
            tuple_type: self.tuple_type,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
            tuple_type: match self.tuple_type {
                TupleType::Point => match other.tuple_type {
                    TupleType::Vector => TupleType::Point,
                    _ => TupleType::Vector,
                },
                TupleType::Vector => match other.tuple_type {
                    TupleType::Vector => TupleType::Vector,
                    _ => TupleType::Point,
                },
            },
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
            tuple_type: self.tuple_type,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
            tuple_type: self.tuple_type,
        }
    }
}

pub fn point(x: i32, y: i32, z: i32) -> Tuple {
    Tuple::new(x as f32, y as f32, z as f32, 1.0)
}

pub fn vector(x: i32, y: i32, z: i32) -> Tuple {
    Tuple::new(x as f32, y as f32, z as f32, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_one_width_is_a_point() {
        let point = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
        assert_eq!(point.tuple_type, TupleType::Point);
    }

    #[test]
    fn a_tuple_with_zero_width_is_a_vector() {
        let vector = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
        assert_eq!(vector.tuple_type, TupleType::Vector);
    }

    #[test]
    fn point_creates_a_tuple_with_w_as_one() {
        let p = point(4, -4, 3);

        assert_eq!(p.tuple_type, TupleType::Point);
    }

    #[test]
    fn vector_creates_a_tuple_with_w_as_zero() {
        let v = vector(4, -4, 3);

        assert_eq!(v.tuple_type, TupleType::Vector);
    }

    #[test]
    fn adding_two_tuples() {
        let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let result = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_eq!(a + b, result);
    }

    #[test]
    fn subtracting_two_points() {
        let a = point(3, 2, 1);
        let b = point(5, 6, 7);
        let result = vector(-2, -4, -6);

        assert_eq!(a - b, result);
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let a = point(3, 2, 1);
        let b = vector(5, 6, 7);
        let expected_result = point(-2, -4, -6);
        let actual_result = a - b;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn subtracting_two_vectors() {
        let a = vector(3, 2, 1);
        let b = vector(5, 6, 7);
        let expected_result = vector(-2, -4, -6);

        assert_eq!(a - b, expected_result);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = vector(0, 0, 0);
        let v = vector(1, -2, 3);
        let expected_result = vector(-1, 2, -3);

        assert_eq!(zero - v, expected_result);
    }

    #[test]
    fn negating_a_tuple() {
        let v = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let expected_result = Tuple::new(-1.0, 2.0, -3.0, 4.0);

        assert_eq!(-v, expected_result);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let expected_result = Tuple::new(3.5, -7.0, 10.5, -14.0);

        assert_eq!(a * 3.5, expected_result);
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let expected_result = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(a * 0.5, expected_result);
    }
}
