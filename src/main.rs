use calculating_pi_rust::pi_math::CalcPi;


fn main() {
    let mut calc_pi =  CalcPi::new(0, 10000000, Some("./testing/main"));
    calc_pi.calc_pi_terms().unwrap();
}

