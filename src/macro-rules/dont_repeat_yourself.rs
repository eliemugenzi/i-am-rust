use std::ops::{Add, Mul, Sub};
use std::process::Output;

macro_rules! assert_equal_len {

    // The `tt` (token tree) designator is used for
    // operators and tokens
    ($a: expr, $b: expr, $func: ident, $op: tt) => {
     assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    };
}

macro_rules! op {
    ($func: ident, $bound: ident, $op: tt, $method: ident) => {
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func: ident, $x: expr, $y: expr, $z: expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let mut y: Vec<_> = iter::repeat($y).take(size).collect();
                    let mut z: Vec<_> = iter::repeat($z).take(size).collect();
                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        };
    }

    test!(add_assign, 1, 2, 3);
    test!(mul_assign, 1, 2, 2);
    test!(sub_assign, 1, 2, -1);
}