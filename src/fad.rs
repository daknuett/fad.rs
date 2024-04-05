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
}
