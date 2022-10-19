use calculating_pi_rust::status_handler::StatusHandler;
use std::env;


fn main() {
    let mut sh = StatusHandler::new("https://piapi.oscorp.ml".to_string());
    if env::args().len() > 1 {
        sh.set_node_info(env::args().nth(1).unwrap().parse::<i32>().unwrap(), env::args().nth(2).unwrap().parse::<i32>().unwrap());
    }
    sh.get_job().unwrap();
    sh.dispatch_job();
}
