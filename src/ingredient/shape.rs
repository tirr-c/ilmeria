use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug)]
pub struct RawShape([[bool; 3]; 3]);

macro_rules! gen_box_method {
    ($method_name:ident, $r:ident, $c:ident, $cmp:path, $target:ident) => {
        fn $method_name(&self) -> Option<usize> {
            let mut ret = 3 - $cmp(0, 3);
            for $r in 0..3 {
                for $c in 0..3 {
                    if self.0[$r][$c] {
                        ret = $cmp(ret, $target);
                    }
                }
            }
            if ret == 3 { None } else { Some(ret) }
        }
    }
}

impl RawShape {
    gen_box_method!(box_left, r, c, ::std::cmp::min, c);
    gen_box_method!(box_right, r, c, ::std::cmp::max, c);
    gen_box_method!(box_top, r, c, ::std::cmp::min, r);
    gen_box_method!(box_bottom, r, c, ::std::cmp::max, r);
    pub fn corner(&self) -> Option<(usize, usize)> {
        self.box_top().and_then(|t| {
            self.box_left().map(|l| (t, l))
        })
    }
    pub fn size(&self) -> (usize, usize) {
        let width = self.box_right().and_then(|r| {
            self.box_left().map(|l| r - l + 1)
        }).unwrap_or(0);
        let height = self.box_bottom().and_then(|b| {
            self.box_top().map(|t| b - t + 1)
        }).unwrap_or(0);
        (width, height)
    }

    fn move_to_corner(&self) -> Self {
        let mut ret = [[false; 3]; 3];
        let (corner_r, corner_c) = match self.corner() {
            Some(x) => x,
            None => return *self,
        };
        for r in corner_r..3 {
            for c in corner_c..3 {
                ret[r - corner_r][c - corner_c] = self.0[r][c];
            }
        }
        RawShape(ret)
    }

    fn rotate_right(&self) -> Self {
        let mut ret = [[false; 3]; 3];
        for r in 0..3 {
            for c in 0..3 {
                ret[r][c] = self.0[2 - c][r];
            }
        }
        RawShape(ret)
    }

    pub fn available_shapes(self) -> HashSet<ProcessedShape> {
        let mut ret = HashSet::new();
        let mut shape = self;
        for _ in 0..4 {
            ret.insert(shape.into());
            shape = shape.rotate_right();
        }
        ret
    }
}

impl From<[[bool; 3]; 3]> for RawShape {
    fn from(val: [[bool; 3]; 3]) -> Self {
        RawShape(val)
    }
}
impl Into<[[bool; 3]; 3]> for RawShape {
    fn into(self) -> [[bool; 3]; 3] {
        self.0
    }
}
impl Into<ProcessedShape> for RawShape {
    fn into(self) -> ProcessedShape {
        let size = self.size();
        let shape = self.move_to_corner();
        ProcessedShape {
            size,
            shape,
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct ProcessedShape {
    size: (usize, usize),
    shape: RawShape,
}
impl Hash for ProcessedShape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (width, height) = self.size;
        for r in 0..height {
            self.shape.0[r][0..width].hash(state);
        }
    }
}
impl PartialEq for ProcessedShape {
    fn eq(&self, other: &Self) -> bool {
        self.shape.0 == other.shape.0
    }
}
impl Eq for ProcessedShape {}

impl ProcessedShape {
    pub fn size(&self) -> (usize, usize) {
        self.size
    }
}
impl Into<RawShape> for ProcessedShape {
    fn into(self) -> RawShape {
        self.shape
    }
}


#[cfg(test)]
mod tests {
    use super::{RawShape, ProcessedShape};

    macro_rules! construct_raw_shape {
        ($($t:tt,)*) => {
            RawShape::from(construct_raw_shape_arr!($($t),*))
        };
    }

    macro_rules! construct_raw_shape_arr {
        ($([$($s:ident)+]),*) => {
            [$([$(construct_raw_shape_arr!($s)),+]),*]
        };
        (O) => { true };
        (x) => { false };
    }

    #[test]
    fn test_raw_shape_corner() {
        let shape = construct_raw_shape!(
            [O x x],
            [x O x],
            [x O x],
        );
        assert_eq!(shape.corner(), Some((0, 0)));

        let shape = construct_raw_shape!(
            [x x x],
            [x O O],
            [x O x],
        );
        assert_eq!(shape.corner(), Some((1, 1)));

        let shape = construct_raw_shape!(
            [x x O],
            [x x O],
            [x O x],
        );
        assert_eq!(shape.corner(), Some((0, 1)));

        let shape = construct_raw_shape!(
            [x x x],
            [x x x],
            [x x x],
        );
        assert_eq!(shape.corner(), None);
    }

    #[test]
    fn test_raw_shape_size() {
        let shape = construct_raw_shape!(
            [O x x],
            [x x x],
            [x x x],
        );
        assert_eq!(shape.size(), (1, 1));

        let shape = construct_raw_shape!(
            [O x x],
            [x O x],
            [x O x],
        );
        assert_eq!(shape.size(), (2, 3));

        let shape = construct_raw_shape!(
            [x x x],
            [x O O],
            [x O x],
        );
        assert_eq!(shape.size(), (2, 2));

        let shape = construct_raw_shape!(
            [x x x],
            [O x O],
            [x O x],
        );
        assert_eq!(shape.size(), (3, 2));

        let shape = construct_raw_shape!(
            [x x x],
            [x x x],
            [x x x],
        );
        assert_eq!(shape.size(), (0, 0));
    }

    #[test]
    fn test_raw_shape_move_to_corner() {
        let given = construct_raw_shape!(
            [O x x],
            [x x x],
            [x x x],
        );
        let expected = construct_raw_shape!(
            [O x x],
            [x x x],
            [x x x],
        );
        let actual = given.move_to_corner();
        assert_eq!(expected.0, actual.0);

        let given = construct_raw_shape!(
            [x O x],
            [x x O],
            [x x x],
        );
        let expected = construct_raw_shape!(
            [O x x],
            [x O x],
            [x x x],
        );
        let actual = given.move_to_corner();
        assert_eq!(expected.0, actual.0);

        let given = construct_raw_shape!(
            [x x x],
            [x O O],
            [O O x],
        );
        let expected = construct_raw_shape!(
            [x O O],
            [O O x],
            [x x x],
        );
        let actual = given.move_to_corner();
        assert_eq!(expected.0, actual.0);

        let given = construct_raw_shape!(
            [x x x],
            [x x O],
            [x O O],
        );
        let expected = construct_raw_shape!(
            [x O x],
            [O O x],
            [x x x],
        );
        let actual = given.move_to_corner();
        assert_eq!(expected.0, actual.0);

        let given = construct_raw_shape!(
            [x x x],
            [x x x],
            [x x x],
        );
        let expected = construct_raw_shape!(
            [x x x],
            [x x x],
            [x x x],
        );
        let actual = given.move_to_corner();
        assert_eq!(expected.0, actual.0);
    }

    #[test]
    fn test_raw_shape_rotate_right() {
        let given = construct_raw_shape!(
            [O O x],
            [x x x],
            [x x x],
        );
        let expected = construct_raw_shape!(
            [x x O],
            [x x O],
            [x x x],
        );
        let actual = given.rotate_right();
        assert_eq!(expected.0, actual.0);

        let given = construct_raw_shape!(
            [x O x],
            [O x x],
            [x O x],
        );
        let expected = construct_raw_shape!(
            [x O x],
            [O x O],
            [x x x],
        );
        let actual = given.rotate_right();
        assert_eq!(expected.0, actual.0);

        let given = construct_raw_shape!(
            [x O O],
            [x O x],
            [x O x],
        );
        let expected = construct_raw_shape!(
            [x x x],
            [O O O],
            [x x O],
        );
        let actual = given.rotate_right();
        assert_eq!(expected.0, actual.0);

        let given = construct_raw_shape!(
            [x x x],
            [x x x],
            [x x x],
        );
        let expected = construct_raw_shape!(
            [x x x],
            [x x x],
            [x x x],
        );
        let actual = given.rotate_right();
        assert_eq!(expected.0, actual.0);
    }
}
