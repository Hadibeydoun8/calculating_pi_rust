use std::collections::HashMap;


pub struct StatusHandler {
    pub job_id: i8,
    pub batch_id: i8,
    pub job_status: i32,

    pub cores_available: i32,
    pub current_memory: f32,
    pub max_memory: f32,
}

impl StatusHandler {
    pub fn new(api_url: String) -> StatusHandler {
        use sysinfo::SystemExt;
        let s = sysinfo::System::new_all();
        StatusHandler {
            job_id: 0,
            batch_id: 0,
            job_status: 0,
            cores_available: num_cpus::get() as i32,
            current_memory: (s.total_memory()/1024/1024) as f32,
            max_memory: (s.total_memory()/1024/1024) as f32,
        }
    }

    // pub async fn get_job_info(api_url: String) -> Result<String, E> {
    //     let body = reqwest::get("https://www.rust-lang.org")
    //         .await?
    //         .text()
    //         .await?;
    //     println!("body = {:?}", body);
    //     return body;
    // }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_cpus() {
        println!("{}", num_cpus::get());
        assert_eq!(num_cpus::get(), 8);
    }
    #[test]
    fn test_ram() {
        use sysinfo::SystemExt;
        let s = sysinfo::System::new_all();
        println!("{}", (s.total_memory()/1024/1024) as f32);
    }

}