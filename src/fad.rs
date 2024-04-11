use num::Num;

#[derive(Copy, Clone)]
pub struct ForwardADNode<C: Num> {
    order0: C,
    order1: C,
}

impl<C: Num> std::ops::Add<ForwardADNode<C>> for ForwardADNode<C> {
    type Output = ForwardADNode<C>;

    fn add(self, rhs: ForwardADNode<C>) -> ForwardADNode<C> {
        ForwardADNode {
            order0: self.order0 + rhs.order0,
            order1: self.order1 + rhs.order1,
        }
    }
}
impl<C: Num> std::ops::Sub<ForwardADNode<C>> for ForwardADNode<C> {
    type Output = ForwardADNode<C>;

    fn sub(self, rhs: ForwardADNode<C>) -> ForwardADNode<C> {
        ForwardADNode {
            order0: self.order0 - rhs.order0,
            order1: self.order1 - rhs.order1,
        }
    }
}

impl<C: Num> std::ops::Add<C> for ForwardADNode<C> {
    type Output = ForwardADNode<C>;

    fn add(self, rhs: C) -> ForwardADNode<C> {
        ForwardADNode {
            order0: self.order0 + rhs,
            order1: self.order1,
        }
    }
}
impl<C: Num> std::ops::Sub<C> for ForwardADNode<C> {
    type Output = ForwardADNode<C>;

    fn sub(self, rhs: C) -> ForwardADNode<C> {
        ForwardADNode {
            order0: self.order0 - rhs,
            order1: self.order1,
        }
    }
}
impl<C: Num + Copy> std::ops::Mul<C> for ForwardADNode<C> {
    type Output = ForwardADNode<C>;

    fn mul(self, rhs: C) -> ForwardADNode<C> {
        ForwardADNode {
            order0: self.order0 * rhs,
            order1: self.order1 * rhs,
        }
    }
}
impl<C: Num + Copy> std::ops::Div<C> for ForwardADNode<C> {
    type Output = ForwardADNode<C>;

    fn div(self, rhs: C) -> ForwardADNode<C> {
        ForwardADNode {
            order0: self.order0 / rhs,
            order1: self.order1 / rhs,
        }
    }
}

impl<C: Num + Copy> std::ops::Mul<ForwardADNode<C>> for ForwardADNode<C> {
    type Output = ForwardADNode<C>;

    fn mul(self, rhs: ForwardADNode<C>) -> ForwardADNode<C> {
        ForwardADNode {
            order0: self.order0 * rhs.order0,
            order1: self.order1 * rhs.order0 + rhs.order1 * self.order0,
        }
    }
}
impl<C: Num + Copy> std::ops::Div<ForwardADNode<C>> for ForwardADNode<C> {
    type Output = ForwardADNode<C>;

    fn div(self, rhs: ForwardADNode<C>) -> ForwardADNode<C> {
        ForwardADNode {
            order0: self.order0 / rhs.order0,
            order1: (self.order1 * rhs.order0 - rhs.order1 * self.order0) / rhs.order0 / rhs.order0,
        }
    }
}

//impl<C: Num> std::ops::Sub<ForwardADNode<C>> for C {
//    type Output = ForwardADNode<C>;
//
//    fn sub(self, rhs: C) -> ForwardADNode<C> {
//        ForwardADNode {
//            order0: self.order0 - rhs,
//            order1: self.order1,
//        }
//    }
//}

impl std::ops::Sub<ForwardADNode<f32>> for f32 {
    type Output = ForwardADNode<f32>;

    fn sub(self, rhs: ForwardADNode<f32>) -> ForwardADNode<f32> {
        ForwardADNode {
            order0: self - rhs.order0,
            order1: -rhs.order1,
        }
    }
}
impl std::ops::Sub<ForwardADNode<f64>> for f64 {
    type Output = ForwardADNode<f64>;

    fn sub(self, rhs: ForwardADNode<f64>) -> ForwardADNode<f64> {
        ForwardADNode {
            order0: self - rhs.order0,
            order1: -rhs.order1,
        }
    }
}
impl std::ops::Add<ForwardADNode<f32>> for f32 {
    type Output = ForwardADNode<f32>;

    fn add(self, rhs: ForwardADNode<f32>) -> ForwardADNode<f32> {
        ForwardADNode {
            order0: self + rhs.order0,
            order1: rhs.order1,
        }
    }
}
impl std::ops::Add<ForwardADNode<f64>> for f64 {
    type Output = ForwardADNode<f64>;

    fn add(self, rhs: ForwardADNode<f64>) -> ForwardADNode<f64> {
        ForwardADNode {
            order0: self + rhs.order0,
            order1: rhs.order1,
        }
    }
}

impl std::ops::Div<ForwardADNode<f32>> for f32 {
    type Output = ForwardADNode<f32>;

    fn div(self, rhs: ForwardADNode<f32>) -> ForwardADNode<f32> {
        ForwardADNode {
            order0: self / rhs.order0,
            order1: - self * rhs.order1 / rhs.order0.powi(2),
        }
    }
}
impl std::ops::Div<ForwardADNode<f64>> for f64 {
    type Output = ForwardADNode<f64>;

    fn div(self, rhs: ForwardADNode<f64>) -> ForwardADNode<f64> {
        ForwardADNode {
            order0: self / rhs.order0,
            order1: - self * rhs.order1 / rhs.order0.powi(2),
        }
    }
}
impl std::ops::Mul<ForwardADNode<f32>> for f32 {
    type Output = ForwardADNode<f32>;

    fn mul(self, rhs: ForwardADNode<f32>) -> ForwardADNode<f32> {
        ForwardADNode {
            order0: self * rhs.order0,
            order1: self * rhs.order1,
        }
    }
}
impl std::ops::Mul<ForwardADNode<f64>> for f64 {
    type Output = ForwardADNode<f64>;

    fn mul(self, rhs: ForwardADNode<f64>) -> ForwardADNode<f64> {
        ForwardADNode {
            order0: self * rhs.order0,
            order1: self * rhs.order1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! assert_releq {
        ($x:expr, $y:expr, $tol:expr) => {
            let mean = ($x + $y) / 2.0;
            if mean.abs() < $tol * 1e-2 
            {
                if $x.abs() > $tol || $y.abs() > $tol
                {
                    panic!();
                }
            }
            let reldif = ($x - $y) / mean;
            if reldif.abs() > $tol
            {
                panic!();
            }

        }
    }

    #[test]
    fn test_add_sub() {
        let a = ForwardADNode{order0: 1.2, order1: 2.3};
        let b = ForwardADNode{order0: 1.4, order1: 4.3};
        {
            let c = a + b;
            assert_releq!(c.order0, 2.6_f32, 1e-5);
            assert_releq!(c.order1, 6.6_f32, 1e-5);
        }
        {
            let d = a - b;
            assert_releq!(d.order0, -0.2_f32, 1e-5);
            assert_releq!(d.order1, -2.0_f32, 1e-5);
        }
        {
            let c = a - 1.1f32;
            assert_releq!(c.order0, 0.1_f32, 1e-5);
            assert_releq!(c.order1, 2.3_f32, 1e-5);
        }
        {
            let c = a - 1.1f32;
            assert_releq!(c.order0, 0.1_f32, 1e-5);
            assert_releq!(c.order1, 2.3_f32, 1e-5);
            
        }
        {
            let c = 1.1f32 - a;
            assert_releq!(c.order0, -0.1_f32, 1e-5);
            assert_releq!(c.order1, -2.3_f32, 1e-5);
        }
    }


    #[test]
    fn diff_mul()
    {
        let a = ForwardADNode{order0: 1.2, order1: 2.3};
        let b = ForwardADNode{order0: 1.4, order1: 4.3};

        {
            let factor = 3f32;
            let c = a / factor;
            assert_releq!(c.order0, a.order0 / factor, 1e-5);
            assert_releq!(c.order1, a.order1 / factor, 1e-5);
        }
        {
            let factor = 3f32;
            let c = a * factor;
            assert_releq!(c.order0, a.order0 * factor, 1e-5);
            assert_releq!(c.order1, a.order1 * factor, 1e-5);
        }
        {
            let factor = 3f32;
            let c = factor / a;
            assert_releq!(c.order0, factor  / a.order0, 1e-5);
            assert_releq!(c.order1, -4.791666666666666_f32, 1e-5);
        }

        {
            let c = a * b;
            assert_releq!(c.order0, 1.68_f32, 1e-5);
            assert_releq!(c.order1, 8.38_f32, 1e-5);
        }
        {
            let c = a / b;
            assert_releq!(c.order0, 0.8571428571428572_f32, 1e-5);
            assert_releq!(c.order1, -0.9897959183673468_f32, 1e-5);
        }

    }
}
