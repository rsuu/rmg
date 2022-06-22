use std::convert::TryInto;

pub struct ArrMatrix<'a, T> {
    pub arr: &'a [T],
    pub width: T,
    pub height: T,
}

pub trait Affine<'a> {
    type T;

    fn new(arr: &'a [Self::T], width: Self::T, height: Self::T) -> Self;
    fn translate_x(&self, step: usize, right: bool) -> Option<Vec<Self::T>>;
    fn translate_y(&self, step: usize, up: bool) -> Option<Vec<Self::T>>;
    // fn other(&self, _: usize, _: bool) -> Option<Vec<Self::T>>;
}

// Translate/Scale/Rotate（平移/缩放/旋转）
impl<'a> Affine<'a> for ArrMatrix<'a, u32> {
    type T = u32;

    fn new(arr: &'a [Self::T], width: Self::T, height: Self::T) -> Self {
        ArrMatrix { arr, width, height }
    }

    fn translate_x(&self, step: usize, right: bool) -> Option<Vec<Self::T>> {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let zero: Self::T = 0;
        let width: usize = self.width.try_into().unwrap();
        let height: usize = self.height.try_into().unwrap();
        let img_size: usize = width * height;

        if self.arr.len() == 0 {
            return None;
        }

        if step > 0 && step < width {
            if right {
                let mut res: Vec<u32> = vec![zero; img_size];

                for line in self.arr.chunks(width) {
                    for _ in line.iter() {
                        // have to use < x
                        // not <= x
                        if ((x + step) < width) {
                            res[x + step + (width * y)] = self.arr[x + (width * y)];
                        } else {
                        }

                        x += 1;
                    }

                    x = 0;
                    y += 1;
                }

                return Some(res);
            } else if right == false {
                let mut res: Vec<u32> = vec![zero; width * height];

                for line in self.arr.chunks(width) {
                    for _ in line.iter() {
                        //if (x.checked_sub(step)).is_some() {
                        if x >= step && x - step >= 0 {
                            res[x - step + (width * y)] = self.arr[x + (width * y)];
                        } else {
                        }

                        x += 1;
                    }

                    x = 0;
                    y += 1;
                }

                return Some(res);
            } else {
                unreachable!()
            }
        } else if step == width {
            return Some(vec![zero; img_size]);
        } else {
            return None;
        }
    }

    fn translate_y(&self, step: usize, up: bool) -> Option<Vec<Self::T>> {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let zero: Self::T = 0;
        let width: usize = self.width.try_into().unwrap();
        let height: usize = self.height.try_into().unwrap();
        let img_size: usize = width * height;

        if self.arr.len() == 0 {
            return None;
        }

        if step > 0 && step < width {
            if up {
                let mut res: Vec<u32> = vec![zero; img_size];

                for line in self.arr.chunks(width) {
                    for _ in line.iter() {
                        if y >= step && y - step >= 0 {
                            res[x + width * (y - step)] = self.arr[x + (width * y)];
                        } else {
                        }

                        x += 1;
                    }

                    x = 0;
                    y += 1;
                }

                return Some(res);
            } else if up == false {
                let mut res: Vec<u32> = vec![zero; width * height];

                for line in self.arr.chunks(width) {
                    for _ in line.iter() {
                        if y + step < height {
                            res[x + width * (y + step)] = self.arr[x + width * y];
                        } else {
                        }

                        x += 1;
                    }

                    x = 0;
                    y += 1;
                }

                return Some(res);
            } else {
                unreachable!()
            }
        } else if step == width {
            return Some(vec![zero; img_size]);
        } else {
            return None;
        }
    }
}

mod test {
    use super::*;

    #[test]
    pub fn test_translate_x() {
        // 3x2
        let mut step = 0;
        let matrix = ArrMatrix {
            arr: [1, 2, 3, 4, 5, 6].as_slice(),
            width: 3,
            height: 2,
        };

        let res = matrix.translate_x(step, true);
        assert_eq!(res, None);

        step = 1;
        let res = matrix.translate_x(step, true).unwrap();
        assert_eq!(res.as_slice(), [0, 1, 2, 0, 4, 5],);

        step = 2;
        let res = matrix.translate_x(step, true).unwrap();
        assert_eq!(res.as_slice(), [0, 0, 1, 0, 0, 4,]);

        step = 3;
        let res = matrix.translate_x(step, true).unwrap();
        assert_eq!(res.as_slice(), [0, 0, 0, 0, 0, 0,]);

        step = 4;
        let res = matrix.translate_x(step, true);
        assert_eq!(res, None,);

        step = 1;
        let res = matrix.translate_x(step, false).unwrap();
        assert_eq!(res.as_slice(), [2, 3, 0, 5, 6, 0,]);
    }

    pub fn test_translate_y() {
        // 3x2
        let mut step = 0;
        let matrix = ArrMatrix {
            arr: [1, 2, 3, 4, 5, 6].as_slice(),
            width: 3,
            height: 2,
        };

        let res = matrix.translate_y(step, true);
        assert_eq!(res, None);

        step = 1;
        let res = matrix.translate_y(step, true).unwrap();
        assert_eq!(res.as_slice(), [4, 5, 6, 0, 0, 0],);

        step = 2;
        let res = matrix.translate_y(step, true).unwrap();
        assert_eq!(res.as_slice(), [0, 0, 0, 0, 0, 0,]);

        step = 3;
        let res = matrix.translate_y(step, true);
        assert_eq!(res, None);

        step = 1;
        let res = matrix.translate_y(step, false).unwrap();
        assert_eq!(res.as_slice(), [0, 0, 0, 1, 2, 3]);
    }
}
