
// fn main() {
//     let mut calc_pi =  CalcPi::new(0, 10000000, Some("./testing/main"));
//     calc_pi.calc_pi_terms().unwrap();
// }

fn main() {
   let response_text = reqwest::get("https://www.rust-lang.org")
       .expect("Error getting response")
       .text()
       .expect("Error getting text");
}

