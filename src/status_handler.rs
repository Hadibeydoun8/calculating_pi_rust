use reqwest;
use serde::{Deserialize, Serialize};



pub struct StatusHandler {
    pub job_id: i8,
    pub batch_id: i8,
    pub job_status: i32,

    pub api_url: String,

    pub cores_available: i32,
    pub current_memory: f32,
    pub max_memory: f32,

    job_info: Option<JobInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JobBatch {
    cpu_needed: f32,
    ram_needed: f32,
    id: i8,
}

#[derive(Serialize, Deserialize, Debug)]
struct JobArgs {
    start_n: f32,
    end_n: f32
}


#[derive(Serialize, Deserialize, Debug)]
struct JobInfo {
    id: i8,
    job_batch: JobBatch,
    job_args: JobArgs
}

#[derive(Serialize, Deserialize)]
struct SetStatus {
    id: i8,
    status: i8
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
            current_memory: (s.total_memory() / 1024 / 1024) as f32,
            max_memory: (s.total_memory() / 1024 / 1024) as f32,

            job_info: None,
        }
    }


    #[tokio::main]
    async fn get_job(&mut self) {
        let x_ids: Vec<i8> = vec![];
        let mut job_selected = false;
        while !&job_selected {

            let resp = reqwest::get(self.api_url.clone() + "/worker-nodes/get-job")
                .await
                .unwrap()
                .json::<JobInfo>()
                .await;

            match resp {
                Ok(job) => {
                    println!("Job selected: {:?}", job);
                    self.job_info = Some(job);
                },
                Err(e) => {
                    panic!("Error: {:?}", e);
                }
            }


            // let resp: JobInfo = serde_json::from_str(resp).unwrap();
            println!("{:?}", self.job_info);
            println!("{:#?}", self.job_info.as_ref().unwrap().job_batch.cpu_needed);
            println!("{:#?}", self.job_info.as_ref().unwrap().job_batch.ram_needed);



            if self.job_info.as_ref().unwrap().job_batch.cpu_needed as i64 > self.cores_available as i64 {
                println!("Not enough cores available");
                self.reject_job().await;
                continue;
            }

            if self.job_info.as_ref().unwrap().job_batch.ram_needed as f64 > self.current_memory as f64 {
                println!("Not enough memory available");
                self.reject_job().await;
                continue;
            }
            job_selected = true;
        }
    }

    async fn reject_job(&mut self) {
        let s = SetStatus {
            id: self.job_info.as_ref().unwrap().id,
            status: 1
        };
        let client = reqwest::Client::new();
        let resp = client.patch(self.api_url.clone() + "/worker-nodes/set-status")
        .json(&s)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

        self.job_info = None;

        print!("{:?}", resp);
    }
    async fn set_status(&self) {
        let client = reqwest::Client::new();
        let _res = client.patch(self.api_url.clone() + "/worker-nodes/set-status/")
            .body(format!("{{\"job_id\": {}, \"job_status\": {}}}", self.job_id, self.job_status))
            .send()
            .await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::status_handler::StatusHandler;

    #[test]
    fn test_cpus() {
        assert_eq!(num_cpus::get(), 8);
    }

    #[test]
    fn test_ram() {
        use sysinfo::SystemExt;
        let s = sysinfo::System::new_all();
        // println!("{:?}", (s.total_memory() / 1024 / 1024) as f32);
    }

    #[test]
    fn test_http() {
        let mut s = StatusHandler::new("https://piapi.oscorp.ml".to_string());
        s.get_job();
    }

    #[test]
    fn test_reject_job() {
        let mut s = StatusHandler::new("https://piapi.oscorp.ml".to_string());
        s.cores_available = 0;
        s.get_job();
    }
}