use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug, Clone, Copy)]
struct Rect<T> {
    // (l.x , l.y)
    //      +--------------+
    //      |              |
    //      |              |
    //      |              |
    //      +--------------+
    //                (r.x , r.y)
    l: Point<T>,
    r: Point<T>,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct GraphPath {}

impl GraphPath {
    pub fn hollow_circle(width: usize, height: usize, radius: usize) -> Vec<(usize, usize)> {
        // TODO:

        let w = width as f32;
        let h = height as f32;
        let r = radius as f32;

        let cx = w / 2.0;
        let cy = h / 2.0;

        let mut x = 0.0;
        let mut y = r - 0.5;

        let mut res = Vec::with_capacity(y as usize * 2);

        while x <= y {
            let px = x + cx;
            let py = y + cy;

            if (0.0..w).contains(&px) && (0.0..h).contains(&py) {
                let dx = px as usize;
                let dy = py as usize;

                res.push((
                    dy * width + dx,
                    dx * width + dy,
                    // (height - dy) * width + dx,
                    // dx * width + (height - dy),
                    // dy * width + (width - dx),
                    // (width - dx) * width + dy,
                    // (height - dy) * width + (width - dx),
                    // (width - dx) * width + (height - dy),
                ));
            }

            x += 1.0;

            // r^2 < x^2+y^2
            if r.powi(2) < x.powi(2) + y.powi(2) {
                y -= 1.0;
            }
        }

        res
    }
}

mod test {
    use super::GraphPath;

    #[test]
    fn _draw_hollow_circle() {
        let _ = GraphPath::hollow_circle(32, 32, 32 / 3);
    }
}
