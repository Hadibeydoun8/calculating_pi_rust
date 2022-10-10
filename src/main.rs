use calculating_pi_rust::status_handler::StatusHandler;


fn main() {
    let mut sh = StatusHandler::new("https://piapi.oscorp.ml".to_string());
    sh.get_job();
    sh.dispatch_job();
}

