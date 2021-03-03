use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const RIGHT: Vec3 = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };

    pub const UP: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    pub const FORWARD: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn new<T, U, V>(x: T, y: U, z: V) -> Self
    where
        T: Into<f64> + Copy,
        U: Into<f64> + Copy,
        V: Into<f64> + Copy,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn write_color(self) {
        let ir = (255.999 * self.x) as u8;
        let ig = (255.999 * self.y) as u8;
        let ib = (255.999 * self.z) as u8;

        println!("{} {} {}", ir, ig, ib);
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();

        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// This macro helps us implement math operators on Vector3
// in such a way that it handles binary operators on any
// combination of Vec3, &Vec3 and f64.
macro_rules! impl_binary_operations {
    // $Operation is something like `Add`
    // $op_fn is something like `add`
    // $op_symbol is something like `+`
    ($Operation:ident $op_fn:ident $op_symbol:tt) => {
      // Implement a + b where a and b are both of type &Vec3.
      impl<'a, 'b> $Operation<&'a Vec3> for &'b Vec3 {
        type Output = Vec3;
        fn $op_fn(self, other: &'a Vec3) -> Vec3 {
          Vec3 {
            x: self.x $op_symbol other.x,
            y: self.y $op_symbol other.y,
            z: self.z $op_symbol other.z,
          }
        }
      }

      // Implement a op b for the cases...
      //
      //   a: Vec3,  b: &Vec3
      //   a: &Vec3, b: Vec3
      //   a: Vec3, b: Vec3
      //
      // In each case we forward through to the implementation above.
      impl $Operation<Vec3> for Vec3 {
        type Output = Vec3;

        #[inline]
        fn $op_fn(self, other: Vec3) -> Vec3 {
          &self $op_symbol &other
        }
      }

      impl<'a> $Operation<&'a Vec3> for Vec3 {
        type Output = Vec3;

        #[inline]
        fn $op_fn(self, other: &'a Vec3) -> Vec3 {
          &self $op_symbol other
        }
      }

      impl<'a> $Operation<Vec3> for &'a Vec3 {
        type Output = Vec3;

        #[inline]
        fn $op_fn(self, other: Vec3) -> Vec3 {
          self $op_symbol &other
        }
      }

      // Implement a + b where a is type &Vec3 and b is type Into<f64>
      impl<'a, T> $Operation<T> for &'a Vec3
      where T: Into<f64> + Copy {
        type Output = Vec3;

        fn $op_fn(self, other: T) -> Vec3 {
          Vec3 {
            x: self.x $op_symbol other.into(),
            y: self.y $op_symbol other.into(),
            z: self.z $op_symbol other.into(),
          }
        }
      }

      // Implement a + b where...
      //
      // a is Vec3 and b is f64
      // a is f64 and b is Vec3
      // a is f64 and b is &Vec3
      //
      // In each case we forward the logic to the implementation
      // above.
      impl<T> $Operation<T> for Vec3
      where T: Into<f64> + Copy {
        type Output = Vec3;

        #[inline]
        fn $op_fn(self, other: T) -> Vec3 {
          &self $op_symbol other
        }
      }

      impl $Operation<Vec3> for f64 {
        type Output = Vec3;

        #[inline]
        fn $op_fn(self, other: Vec3) -> Vec3 {
          &other $op_symbol self
        }
      }

      impl<'a> $Operation<&'a Vec3> for f64 {
        type Output = Vec3;

        #[inline]
        fn $op_fn(self, other: &'a Vec3) -> Vec3 {
          other $op_symbol self
        }
      }
    };
  }

macro_rules! impl_unary_operators {
    // $Operation is something like `Add`
    // $op_fn is something like `add`
    // $op_symbol is something like `+`
    ($Operation:ident $op_fn:ident $op_symbol:tt) => {
        impl<'a> $Operation for &'a Vec3 {
            type Output = Vec3;
            fn $op_fn(self) -> Vec3 {
              Vec3 {
                x: $op_symbol self.x,
                y: $op_symbol self.y,
                z: $op_symbol self.z,
              }
            }
          }

          impl $Operation for Vec3 {
              type Output = Vec3;
              fn $op_fn(self) -> Vec3 {
                  $op_symbol &self
              }
          }
    };
}

impl_binary_operations!(Add add +);
impl_binary_operations!(Sub sub -);
impl_binary_operations!(Mul mul *);
impl_binary_operations!(Div div /);

impl_unary_operators!(Neg neg -);
