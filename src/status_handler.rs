use reqwest;
use serde::{Deserialize, Serialize};


use tokio::sync::mpsc;


use crate::pi_math::CalcPi;


pub struct StatusHandler {
    job_status: i32,

    api_url: String,

    cores_available: i32,
    current_memory: f32,

    process_id: i32,

    job_info: Option<JobInfo>,

}

#[derive(Debug)]
pub struct PercentUpdate {
    pub percent: f32,
}

impl PercentUpdate {
    pub fn new(percent: f32) -> Self {
        PercentUpdate {
            percent,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PStatusUpdate {
    id: f32,
    percentage_complete: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct JobBatch {
    cpu_needed: f32,
    ram_needed: f32,
    id: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct JobArgs {
    start_n: f32,
    end_n: f32,
    status_update_interval: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct JobInfo {
    id: f32,
    job_batch: JobBatch,
    job_args: JobArgs,
}

#[derive(Serialize, Deserialize)]
struct SetStatus {
    id: f32,
    status: i8,
}

#[derive(Serialize, Deserialize)]
struct NodeInfo {
    id: f32,
    available_cores: i32,
    available_ram: f32,
    process_id: i32,
}

impl NodeInfo {
    pub fn new(id: f32, available_cores: i32, available_ram: f32, process_id: i32) -> Self {
        NodeInfo {
            id,
            available_cores,
            available_ram,
            process_id,
        }
    }
}

impl StatusHandler {
    pub fn new(api_url: String) -> StatusHandler {
        use sysinfo::SystemExt;
        let s = sysinfo::System::new_all();
        StatusHandler {
            job_status: 0,

            api_url,

            cores_available: num_cpus::get() as i32,
            current_memory: (s.total_memory() / 1024 / 1024) as f32,

            job_info: None,

            process_id: -1,

        }
    }
    #[tokio::main]
    pub async fn get_job(&mut self) {
        let mut x_ids: Vec<u8> = vec![];
        let mut job_selected = false;
        let mut err_count = 0;
        while !&job_selected {
            let client = reqwest::Client::new();
            let resp = client.put(self.api_url.clone() + "/worker-nodes/get-job")
                .json(&x_ids)
                .send()
                .await
                .unwrap()
                .json::<JobInfo>()
                .await;

            match resp {
                Ok(job) => {
                    println!("Job selected: {:?}", job);
                    self.job_info = Some(job);
                }
                Err(e) => {
                    if err_count > 5 {
                        println!("Error: {:?}", e);
                        err_count += 1;
                        continue;
                    } else {
                        panic!("Error: {:?}", e);
                    }
                }
            }

            if self.job_info.as_ref().unwrap().job_batch.cpu_needed as i64 > self.cores_available as i64 {
                println!("Not enough cores available");
                x_ids.push(self.job_info.as_ref().unwrap().id as u8);
                self.reject_job().await;
                continue;
            }

            if self.job_info.as_ref().unwrap().job_batch.ram_needed as f64 > self.current_memory as f64 {
                println!("Not enough memory available");
                x_ids.push(self.job_info.as_ref().unwrap().id as u8);
                self.reject_job().await;
                continue;
            }

            self.accept_job().await;
            job_selected = true;
        }
    }
    #[tokio::main]
    pub async fn dispatch_job(&mut self) {
        let job = self.job_info.as_ref().unwrap();
        let start_n = job.job_args.start_n;
        let end_n = job.job_args.end_n;
        let mut calc_pi = CalcPi::new(start_n as i128, end_n as i128, Some("./"));
        calc_pi.set_status_update_interval(self.job_info.as_ref().unwrap().job_args.status_update_interval as i128);
        calc_pi.set_data_handler_archive_id(self.job_info.as_ref().unwrap().id as i32, self.job_info.as_ref().unwrap().job_batch.id as i32);
        self.job_status = 4;
        self.write_new_status().await;
        let (tx, mut rx) = mpsc::channel(32);


        tokio::spawn(
            async move {
                calc_pi.calc_pi_terms_with_status(tx).await;
            }
        );

        while let Some(message) = rx.recv().await {
            self.update_percent_complete(message).await;
        }
        self.complete_job().await;
    }
    pub fn set_process_id(&mut self, id: i32) {
        self.process_id = id;
    }
    async fn reject_job(&mut self) {
        self.job_status = 1;
        self.write_new_status().await;
        self.job_info = None;
    }
    async fn accept_job(&mut self) {
        self.job_status = 3;
        self.update_node_info().await;
        self.write_new_status().await;
    }
    async fn complete_job(&mut self) {
        self.job_status = 5;
        self.write_new_status().await;
    }
    async fn update_node_info(&mut self) {
        let client = reqwest::Client::new();
        let node_info = NodeInfo::new(self.job_info.as_ref().unwrap().id, self.cores_available, self.current_memory, self.process_id);
        let _ = client.patch(self.api_url.clone() + "/worker-nodes/set-info")
            .json(&node_info)
            .send()
            .await
            .unwrap();
    }
    async fn write_new_status(&mut self) {
        let s = SetStatus {
            id: self.job_info.as_ref().unwrap().id,
            status: self.job_status as i8,
        };
        let client = reqwest::Client::new();
        let mut err_count = 0;
        loop {
            let resp = client.patch(self.api_url.clone() + "/worker-nodes/set-status")
                .json(&s)
                .send()
                .await;
            match resp {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    if err_count > 5 {
                        println!("Error: {:?}", e);
                        err_count += 1;
                        continue;
                    } else {
                        panic!("Error: {:?}", e);
                    }
                }
            }
        }
    }
    async fn update_percent_complete(&mut self, percent: PercentUpdate) {
        let client = reqwest::Client::new();
        let mut err_count = 0;
        loop {
            let resp = client.patch(self.api_url.clone() + "/worker-nodes/update-percentage")
                .json(&PStatusUpdate {
                    id: self.job_info.as_ref().unwrap().id,
                    percentage_complete: percent.percent,
                })
                .send()
                .await;
            match resp {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    if err_count > 5 {
                        println!("Update Percent Complete Error: {:?}", e);
                        err_count += 1;
                        continue;
                    } else {
                        panic!("Update Percent Complete Error: {:?}", e);
                    }
                }
            }
        }
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
        dbg!("{:?}", (s.total_memory() / 1024 / 1024) as f32);
    }

    #[test]
    fn test_http() {
        let mut s = StatusHandler::new("https://piapi.oscorp.ml".to_string());
        s.get_job();
    }

    #[test]
    fn test_spawn() {
        let mut s = StatusHandler::new("https://piapi.oscorp.ml".to_string());
        s.get_job();
        s.dispatch_job();
    }

    #[test]
    fn test_update_node_info() {
        let mut s = StatusHandler::new("https://piapi.oscorp.ml".to_string());
        s.get_job();
    }
}