use std::fmt;
use std::ops::{Add, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn zero() -> Self { Self { x: 0f32, y: 0f32 } }
    pub fn one() -> Self { Self { x: 1f32, y: 1f32 } }
    pub fn up() -> Self { Self { x: 0f32, y: 1f32 } }
    pub fn down() -> Self { Self { x: 0f32, y: -1f32 } }
    pub fn left() -> Self { Self { x: -1f32, y: 0f32 } }
    pub fn right() -> Self { Self { x: 1f32, y: 0f32 } }
    pub fn positive_infinity() -> Self { Self { x: f32::INFINITY, y: f32::INFINITY } }
    pub fn negative_infinity() -> Self { Self { x: f32::NEG_INFINITY, y: f32::NEG_INFINITY } }
    pub fn magnitude(&self) -> f32 { self.sqr_magnitude().sqrt() }
    pub fn sqr_magnitude(&self) -> f32 { self.x * self.x + self.y * self.y }
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    pub fn lerp(a: Self, b: Self, mut t: f32) -> Self {
        t = t.clamp(0f32, 1f32);
        Self::new(a.x + (b.x - a.x) * t,
                  a.y + (b.y - a.y) * t)
    }
    pub fn lerp_unclamped(a: Self, b: Self, t: f32) -> Self {
        Self::new(a.x + (b.x - a.x) * t,
                  a.y + (b.y - a.y) * t)
    }
    pub fn move_towards(current: Self, target: Self, max_distance_delta: f32) -> Self {
        let vector = target - current;
        let num = vector.magnitude();
        return if num <= max_distance_delta || num == 0f32 {
            target
        } else {
            current + vector / num * max_distance_delta
        }
    }

    pub fn scale(&mut self, scale: Self) {
        *self *= scale
    }

    pub fn normalize(&mut self) {
        let num = self.magnitude();
        if num > 1E-05f32 {
            *self /= num;
        } else {
            *self = Self::zero();
        }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self::new(self.x * other, self.y * other)
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, other: Vector2) -> Vector2 {
        Vector2::new(self * other.x, self * other.y)
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32)  {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, scale: Self)  {
        self.x *= scale.x;
        self.y *= scale.y;
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
    }
}

impl Div<f32> for Vector2 {
        type Output = Self;

    fn div(self, other: f32) -> Self {
        Self::new(self.x / other, self.y / other)
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32)  {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2_creation() {
        let v = Vector2::new(1f32, 2f32);
        assert_eq!(v.x, 1f32);
        assert_eq!(v.y, 2f32);
        let v = Vector2::zero();
        assert_eq!(v.x, 0f32);
        assert_eq!(v.y, 0f32);
        let v = Vector2::one();
        assert_eq!(v.x, 1f32);
        assert_eq!(v.y, 1f32);
        let v = Vector2::up();
        assert_eq!(v.x, 0f32);
        assert_eq!(v.y, 1f32);
        let v = Vector2::down();
        assert_eq!(v.x, 0f32);
        assert_eq!(v.y, -1f32);
        let v = Vector2::left();
        assert_eq!(v.x, -1f32);
        assert_eq!(v.y, 0f32);
        let v = Vector2::right();
        assert_eq!(v.x, 1f32);
        assert_eq!(v.y, 0f32);
        let v = Vector2::positive_infinity();
        assert_eq!(v.x, f32::INFINITY);
        assert_eq!(v.y, f32::INFINITY);
        let v = Vector2::negative_infinity();
        assert_eq!(v.x, f32::NEG_INFINITY);
        assert_eq!(v.y, f32::NEG_INFINITY);
    }

    #[test]
    fn test_set() {
        let mut v = Vector2::new(1f32, 2f32);
        assert_eq!(v.x, 1f32);
        assert_eq!(v.y, 2f32);
        v.set(0f32, 0f32);
        assert_eq!(v.x, 0f32);
        assert_eq!(v.y, 0f32);
    }

    #[test]
    fn test_magnitude() {
        let v = Vector2::new(2f32, 2f32);
        assert_eq!(v.magnitude(), 8f32.sqrt());
        assert_eq!(v.sqr_magnitude(), 8f32);
    }

    #[test]
    fn test_with_index() {
        let v = Vector2::new(1f32, 2f32);
        assert_eq!(v[0], 1f32);
        assert_eq!(v[1], 2f32);
    }

    #[test]
    fn test_fields_mutable() {
        let mut v = Vector2::new(1f32, 2f32);
        v.x = 3f32;
        v.y = 4f32;
        assert_eq!(v.x, 3f32);
        assert_eq!(v.y, 4f32);
    }

    #[test]
    fn test_fields_mutable_with_index() {
        let mut v = Vector2::new(1f32, 2f32);
        v[0] = 3f32;
        v[1] = 4f32;
        assert_eq!(v.x, 3f32);
        assert_eq!(v.y, 4f32);
    }

    #[test]
    fn test_lerp() {
        let a = Vector2::new(1f32, 1f32);
        let b = Vector2::new(2f32, 2f32);
        let v1 = Vector2::lerp(a, b, 1f32);
        assert_eq!(v1.x, 2f32);
        assert_eq!(v1.y, 2f32);
        let v2 = Vector2::lerp_unclamped(a, b, 2f32);
        assert_eq!(v2.x, 3f32);
        assert_eq!(v2.y, 3f32);
        let v3 = Vector2::lerp(a, b, 3f32);
        assert_eq!(v3.x, 2f32);
        assert_eq!(v3.y, 2f32);
    }

    #[test]
    fn test_operators() {
        let a = Vector2::new(2f32, 2f32);
        let b = Vector2::new(2f32, 2f32);
        assert_eq!(a + b, Vector2::new(4f32,4f32));
        assert_eq!(a - b, Vector2::new(0f32,0f32));
        assert_eq!(a * b, Vector2::new(4f32,4f32));
        assert_eq!(a / b, Vector2::new(1f32,1f32));
        assert_eq!(a * 2f32, Vector2::new(4f32,4f32));
        assert_eq!(2f32 * a, Vector2::new(4f32,4f32));
        assert_eq!(a / 2f32, Vector2::new(1f32,1f32));
    }

    #[test]
    fn test_move_towards() {
        let current = Vector2::new(0f32, 0f32);
        let target = Vector2::new(1f32, 1f32);

        let result = Vector2::move_towards(current, target, 0.5f32);
        assert_eq!(result.x, 0.5f32 / 2f32.sqrt());
        assert_eq!(result.y, 0.5f32 / 2f32.sqrt());

        let result = Vector2::move_towards(current, target, 2f32);
        assert_eq!(result.x, 1f32);
        assert_eq!(result.y, 1f32);

        let result = Vector2::move_towards(current, target, 0f32);
        assert_eq!(result.x, 0f32);
        assert_eq!(result.y, 0f32);
    }

    #[test]
    fn test_scale() {
        let mut v = Vector2::new(2f32, 2f32);
        let scale = Vector2::new(1f32, 2f32);
        v.scale(scale);
        assert_eq!(v, Vector2::new(2f32,4f32));
    }

    #[test]
    fn test_normalize() {
        let mut v1 = Vector2::new(1f32, 1f32);
        v1.normalize();
        assert_eq!(v1, Vector2::new(0.5f32.sqrt(),0.5f32.sqrt()));
        let mut v2 = Vector2::new(0f32, 0.00001f32);
        v2.normalize();
        assert_eq!(v2, Vector2::new(0f32.sqrt(),0f32.sqrt()));
    }
}
