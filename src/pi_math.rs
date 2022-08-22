pub mod pi_math {
    use std::ops::{Add, Div, Mul, Sub};

    use log::{debug, error, info, trace, warn};

    use rug::{Complete, Integer};
    use rug::ops::Pow;

    pub struct CalcPi {
        pub n_start: rug::Integer,
        pub n_end: rug::Integer,
        recursion_ready: bool,
        last_n: rug::Integer,
        last_l: rug::Integer,
        last_m: rug::Integer,
        last_x: rug::Integer,
        _k: rug::Integer,
    }

    impl CalcPi {
        pub fn new(n_start: rug::Integer, n_end: rug::Integer) -> Self {
            CalcPi {
                n_start,
                n_end,
                recursion_ready: false,
                last_n: rug::Integer::from(0),
                last_l: rug::Integer::from(0),
                last_m: rug::Integer::from(0),
                last_x: rug::Integer::from(0),
                _k: rug::Integer::from(0),
            }
        }

        fn calc_pi(&self) -> std::io::Result<()> {
            let mut pi: rug::Integer = rug::Integer::from(0);
            // for n in self.n_start..self.n_end {}
            Ok(())
        }


        fn calc_next_term(&self) -> std::io::Result<(Integer)> {
            let _num: Integer = Integer::mul(self.last_m.clone(), &self.last_l);
            let _qt: Integer = Integer::div(_num, self.last_x.clone());

            Ok((_qt))
        }


        fn calc_l_m_x(&mut self, n: rug::Integer) -> std::io::Result<()> {
            let _n: u32 = n.to_u32().unwrap();

            if self.recursion_ready {
                if self.last_n != Integer::sub(n.clone(), 1) {
                    println!("Recursion not ready at n={}, last_n={}", n, self.last_n);
                    self.recursion_ready = false;
                }
            }
            self.last_n = n;
            if !self.recursion_ready {

                // calc init m value

                let _q = rug::Integer::factorial(6 * &_n).complete();
                let _w = rug::Integer::factorial(3 * &_n).complete();
                let _e = rug::Integer::pow(Integer::factorial(*&_n).complete(), 3);
                self.last_m = _q / (_w * _e);

                let _kh: i128 = -6 + (12 * &_n) as i128;
                self._k = rug::Integer::from(_kh);

                // calc init l value
                let _a = rug::Integer::mul(Integer::from(545140134), &_n);
                self.last_l = rug::Integer::add(_a, 13591409);

                // calc init x value
                self.last_x = rug::Integer::pow(Integer::from(-262537412640768000 as i64), &_n);

                self.recursion_ready = true;
            } else {
                self.last_l = rug::Integer::from(&self.last_l + 545140134);
                self.last_x = rug::Integer::from(&self.last_x * -262537412640768000 as i128);
                self._k     = &self._k + Integer::from(12*&_n);

                let _q = rug::Integer::pow(Integer::from(&self._k), 3);
                let _w = Integer::mul(Integer::from(16), &self._k);
                let _e = Integer::pow(Integer::from(_n as i128), 3);

                let _num: Integer = Integer::sub(_q, _w);

                let _last_m_temp = self.last_m.clone();
                self.last_m = Integer::div(_num, _e);
                self.last_m = Integer::mul(Integer::from(&self.last_m), &_last_m_temp);
            }
            Ok(())
        }


        fn calc_final_after_sum(&mut self, n: rug::Integer) -> std::io::Result<(Integer)> {
            let _c : Integer = Integer::mul(Integer::sqrt(Integer::from(10005)), 426880);
            let _final_pi: Integer = Integer::mul(n.clone(), &_c);
            println!("_c: {}, n: {}, _final_pi: {}", _c, n, _final_pi);
            Ok((_final_pi))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_rug() {
            let a = Integer::from(1);
            println!("{}", a)
        }

        #[test]
        fn test_init_calc_l_m_x() {
            let n = Integer::from(0);
            let mut _c = CalcPi::new(Integer::from(0), Integer::from(1));
            _c.calc_l_m_x(rug::Integer::from(0));
            assert_eq!(_c.last_l, Integer::from(13591409));
            assert_eq!(_c.last_m, Integer::from(1));
            assert_eq!(_c.last_x, Integer::from(1));
            assert_eq!(_c.recursion_ready, true);
            println!("l: {}, x: {}, m: {}", _c.last_l, _c.last_x, _c.last_m);
        }

        #[test]
        fn test_recursive_calc_l_m_x() {
            let mut _c = CalcPi::new(Integer::from(0), Integer::from(1));
            _c.calc_l_m_x(rug::Integer::from(0));
            _c.calc_l_m_x(rug::Integer::from(1));
            // _c.calc_l_m_x(rug::Integer::from(1));
            assert_eq!(_c.last_l, Integer::from(558731543));
            assert_eq!(_c.last_m, Integer::from(120));
            assert_eq!(_c.last_x, Integer::from(-262537412640768000 as i128));
            print!("l: {}, x: {}, m: {}", _c.last_l, _c.last_x, _c.last_m);
        }

        #[test]
        fn test_recursion_ready() {
            let mut _c = CalcPi::new(Integer::from(0), Integer::from(1));
            _c.calc_l_m_x(rug::Integer::from(0));
            _c.calc_l_m_x(rug::Integer::from(1));
            assert_eq!(_c.recursion_ready, true);
            _c.calc_l_m_x(rug::Integer::from(3));
            assert_eq!(_c.recursion_ready, true);
            _c.calc_l_m_x(rug::Integer::from(4));
            assert_eq!(_c.recursion_ready, true);
        }

        #[test]
        fn test_calc_pi() {
            let mut _c = CalcPi::new(Integer::from(0), Integer::from(1));
            _c.calc_l_m_x(rug::Integer::from(0)).expect("TODO: panic message");
            let _next_term = _c.calc_next_term().unwrap();

            println!("{}", _c.calc_final_after_sum(_next_term).unwrap());
        }
    }
}
