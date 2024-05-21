use crate::*;

pub fn bezier(points: &[Vec2], t: f32) -> Vec2 {
    if points.len() == 1 {
        return points[0];
    }

    let mut new_points = Vec::new();
    for i in 0..points.len() - 1 {
        let (x1, y1) = (points[i].x, points[i].y);
        let (x2, y2) = (points[i + 1].x, points[i + 1].y);
        let x = (1.0 - t) * x1 + t * x2;
        let y = (1.0 - t) * y1 + t * y2;

        new_points.push(Vec2::new(x, y));
    }

    bezier(&new_points, t)
}

// fn main() {
//     let points = vec![(50.0, 100.0), (100.0, 200.0), (200.0, 50.0), (250.0, 150.0)];
//
//     for i in 0..=100 {
//         let t = i as f32 / 100.0;
//         let (x, y) = bezier(&points, t);
//         println!("Point at t={:.2}: ({:.2}, {:.2})", t, x, y);
//     }
// }
