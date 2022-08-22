
pub mod status_handler {

    struct DBHandler {
        conn: mysql::Pool,
    }
    impl DBHandler
    {
        pub fn new(_db_name: &str) -> DBHandler {
            let _pool = mysql::Pool::new("mysql://root:bdd9eb6d@192.168.86.34:6000").unwrap();
            // let mut _conn = _pool.get_conn().unwrap();
            return DBHandler { conn: _pool};
        }
    }


    pub struct StatusHandler {
        pub job_id:           i8,
        pub batch_id :        i8,
        pub job_status:      i32,
        pub cores_used:      i32,
        pub cores_available: i32,
        pub current_memory:  i32,
        pub max_memory:      i32,
        db_handler: DBHandler,
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
                db_handler: DBHandler::new("calculating_pi")
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
        // #[cfg(test)]
        // fn test_db() {
        //     let mut db = DBHandler::new("calculating_pi");
        //     let _result = db.conn.query("SELECT * FROM thread_status", ).unwrap();
        //     println!("{:?}", _result);
        //     // assert_eq!(_result, true);
        // }
    }
}