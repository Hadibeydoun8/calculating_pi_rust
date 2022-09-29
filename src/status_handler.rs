use serde::{Serialize, Deserialize};
use reqwest::{Client};
use reqwest;
use std::collections::HashMap;
use serde_json::json;


pub struct StatusHandler {
    pub job_id: i8,
    pub batch_id: i8,
    pub job_status: i32,

    pub api_url: String,

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

            api_url,

            cores_available: num_cpus::get() as i32,
            current_memory: (s.total_memory()/1024/1024) as f32,
            max_memory: (s.total_memory()/1024/1024) as f32,
        }
    }

    #[tokio::main]
    async fn get_job_types() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(" http://127.0.0.1:8000/job-types/")
            .await?
            .json::<serde_json::Value>()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use crate::status_handler;

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
    #[test]
    fn test_http() {
        status_handler::StatusHandler::get_job_types();
    }

}