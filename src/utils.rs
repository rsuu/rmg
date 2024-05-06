pub mod filter;
pub mod size;
pub mod vec2;

// REFS: https://isocpp.org/wiki/faq/newbie#floating-point-arith
pub fn is_similar(a: f32, b: f32) -> bool {
    (a - b).abs() <= a.abs() * f32::EPSILON
}
