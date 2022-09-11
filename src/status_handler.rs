
pub struct StatusHandler {
    pub job_id: i8,
    pub batch_id: i8,
    pub job_status: i32,
    pub cores_used: i32,
    pub cores_available: i32,
    pub current_memory: i32,
    pub max_memory: i32,
}

impl StatusHandler {
    pub fn new(job_id: i8, job_status: i32, cores_used: i32, current_memory: i32, max_memory: i32) -> StatusHandler {
        StatusHandler {
            job_id,
            batch_id: 0,
            job_status,
            cores_used,
            cores_available: num_cpus::get() as i32,
            current_memory,
            max_memory,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cpus() {
        println!("{}", num_cpus::get());
        assert_eq!(num_cpus::get(), 8);
    }
}