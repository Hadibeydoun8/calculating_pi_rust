use std::ops::{Add, Div, Mul, Sub};

use rug::ops::Pow;
use rug::{Complete, Integer};
use tokio::sync::mpsc;

use crate::data_handler::DataWriter;

use crate::status_handler::PercentUpdate;

// enum CalculatingPiError {
//     RequiredValueNotSet(String),
// }

pub struct CalcPi {
    n_start: i128,
    n_end: i128,
    status_update_interval: Option<i128>,

    recursion_ready: bool,

    data_handler: DataWriter,

    last_n: Integer,
    last_l: Integer,
    last_m: Integer,
    last_x: Integer,
    _k: Integer,
}

impl CalcPi {
    pub fn new(n_start: i128, n_end: i128, base_output_path: Option<&str>) -> Self {
        CalcPi {
            n_start,
            n_end,
            status_update_interval: None,
            recursion_ready: false,
            data_handler: DataWriter::new("csv", base_output_path),
            last_n: Integer::from(0),
            last_l: Integer::from(0),
            last_m: Integer::from(0),
            last_x: Integer::from(0),
            _k: Integer::from(0),
        }
    }

    pub fn set_status_update_interval(&mut self, interval: i128) {
        self.status_update_interval = Some(interval);
    }

    pub fn calc_pi_terms(&mut self) -> std::io::Result<()> {
        self.init_data_handler();
        for n in self.n_start..self.n_end {
            self.calc_l_m_x(Integer::from(n));
            self.write_most_recent_l_m_x();
        }
        self.data_handler.close_and_compress_output().unwrap();
        Ok(())
    }

    pub async fn calc_pi_terms_with_status(&mut self, tx: mpsc::Sender<PercentUpdate>) {
        let range = self.n_end - self.n_start;
        if self.status_update_interval.is_none() {
            panic!("Status update interval not set");
        }
        self.init_data_handler();
        for n in self.n_start..self.n_end {
            if n % self.status_update_interval.unwrap() == 0 {
                let percent_complete = (n - self.n_start) as f32 / range as f32 * 100.0;
                println!("Percent complete: {} {}", percent_complete, n);
                tx.send(PercentUpdate::new(percent_complete)).await.unwrap();
                tokio::time::sleep(std::time::Duration::from_millis(0)).await;
            }
            self.calc_l_m_x(Integer::from(n));
            self.write_most_recent_l_m_x();
        }
        tx.send(PercentUpdate::new(100.0)).await.unwrap();
        self.data_handler.close_and_compress_output().unwrap();
    }

    pub fn set_data_handler_archive_id(&mut self, id: i32, batch_id: i32) {
        self.data_handler.set_archive_id(id, batch_id);
    }

    #[cfg(bench)]
    pub fn calc_pi_no_write(&mut self) -> std::io::Result<()> {
        self.init_data_handler();
        for n in self.n_start..self.n_end {
            self.calc_l_m_x(Integer::from(n));
        }

        Ok(())
    }

    fn calc_l_m_x(&mut self, n: Integer) {
        let _n: u32 = n.to_u32().unwrap();

        if self.recursion_ready && self.last_n != Integer::sub(n.clone(), 1) {
            println!("Recursion not ready at n={}, last_n={}", n, self.last_n);
            self.recursion_ready = false;
        }
        self.last_n = n;
        if !self.recursion_ready {
            // calc init m value

            let _q = Integer::factorial(6 * &_n).complete();
            let _w = Integer::factorial(3 * &_n).complete();
            let _e = Integer::pow(Integer::factorial(_n).complete(), 3);
            self.last_m = _q / (_w * _e);

            let _kh: i128 = -6 + (12 * &_n) as i128;
            self._k = Integer::from(_kh);

            // calc init l value
            let _a = Integer::mul(Integer::from(545140134), &_n);
            self.last_l = Integer::add(_a, 13591409);

            // calc init x value
            self.last_x = Integer::pow(Integer::from(-262537412640768000_i64), &_n);

            self.recursion_ready = true;
        } else {
            self.last_l = Integer::from(&self.last_l + 545140134);
            self.last_x = Integer::from(&self.last_x * -262537412640768000_i128);
            self._k = &self._k + Integer::from(12 * &_n);

            let _q = Integer::pow(Integer::from(&self._k), 3);
            let _w = Integer::mul(Integer::from(16), &self._k);
            let _e = Integer::pow(Integer::from(_n as i128), 3);

            let _num: Integer = Integer::sub(_q, _w);

            let _last_m_temp = self.last_m.clone();
            self.last_m = Integer::div(_num, _e);
            self.last_m = Integer::mul(Integer::from(&self.last_m), &_last_m_temp);
        }
    }

    fn write_most_recent_l_m_x(&mut self) {
        let data = vec![
            self.last_n.to_string(),
            self.last_l.to_string(),
            self.last_m.to_string(),
            self.last_x.to_string(),
        ];
        self.data_handler
            .write_data_using_array(data, Some(true))
            .unwrap();
    }

    fn init_data_handler(&mut self) {
        self.data_handler
            .assign_headers(vec![
                "n".to_string(),
                "l".to_string(),
                "m".to_string(),
                "x".to_string(),
            ])
            .unwrap();
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
        let test_path = Some("./testing");
        let mut _c = CalcPi::new(0_i128, 1_i128, test_path);
        _c.calc_l_m_x(Integer::from(0));
        assert_eq!(_c.last_l, Integer::from(13591409));
        assert_eq!(_c.last_m, Integer::from(1));
        assert_eq!(_c.last_x, Integer::from(1));
        assert!(_c.recursion_ready);
        println!("l: {}, x: {}, m: {}", _c.last_l, _c.last_x, _c.last_m);
    }

    #[test]
    fn test_recursive_calc_l_m_x() {
        let test_path = Some("./testing");
        let mut _c = CalcPi::new(0_i128, 1_i128, test_path);
        _c.calc_l_m_x(Integer::from(0));
        _c.calc_l_m_x(Integer::from(1));
        // _c.calc_l_m_x(Integer::from(1));
        assert_eq!(_c.last_l, Integer::from(558731543));
        assert_eq!(_c.last_m, Integer::from(120));
        assert_eq!(_c.last_x, Integer::from(-262537412640768000_i128));
        print!("l: {}, x: {}, m: {}", _c.last_l, _c.last_x, _c.last_m);
    }

    #[test]
    fn test_recursion_ready() {
        let test_path = Some("./testing");
        let mut _c = CalcPi::new(0_i128, 1_i128, test_path);
        _c.calc_l_m_x(Integer::from(0));
        _c.calc_l_m_x(Integer::from(1));
        assert!(_c.recursion_ready);
        _c.calc_l_m_x(Integer::from(3));
        assert!(_c.recursion_ready);
        _c.calc_l_m_x(Integer::from(4));
        assert!(_c.recursion_ready);
    }

    #[test]
    fn test_calc_pi() {
        let test_path = Some("./testing");
        let mut _c = CalcPi::new(0, 1000, test_path);
        _c.calc_pi_terms().unwrap();
    }
}
