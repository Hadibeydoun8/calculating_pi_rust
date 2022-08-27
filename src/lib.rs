pub mod status_handler {
    struct DBHandler {
        conn: mysql::Pool,
    }

    impl DBHandler
    {
        pub fn new(_db_name: &str) -> DBHandler {
            let _pool = mysql::Pool::new("mysql://root:bdd9eb6d@192.168.86.34:6000").unwrap();
            // let mut _conn = _pool.get_conn().unwrap();
            return DBHandler { conn: _pool };
        }
    }


    pub struct StatusHandler {
        pub job_id: i8,
        pub batch_id: i8,
        pub job_status: i32,
        pub cores_used: i32,
        pub cores_available: i32,
        pub current_memory: i32,
        pub max_memory: i32,
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
                db_handler: DBHandler::new("calculating_pi"),
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

pub mod data_handler {
    use std::{fmt};
    use std::fmt::Debug;
    use std::fs::{create_dir, File, create_dir_all};
    use std::io::Write;
    use std::path::Path;

    use flate2::write::GzEncoder;
    use tar::Builder;



    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum HeaderError {
        FileTypeNotSupported(String),
        HeaderAlreadyWritten(Vec<String>),
        TooLateToAddHeader(i32),
        HeaderNotInitialized(),
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum DataWriterError {
        FileAlreadyExists(String),
    }


    impl fmt::Display for HeaderError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                HeaderError::FileTypeNotSupported(_s) =>
                    write!(f, "The file type {} does not support headers", _s),
                HeaderError::HeaderAlreadyWritten(_v) =>
                    write!(f, "The header: {:?} has already been written", _v),
                HeaderError::TooLateToAddHeader(_i) =>
                    write!(f, "{} lines have already been written, cannot add header", _i),
                HeaderError::HeaderNotInitialized() =>
                    write!(f, "The header has not been initialized"),
            }
        }
    }

    impl fmt::Display for DataWriterError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                DataWriterError::FileAlreadyExists(_s) =>
                    write!(f, "The file {} already exists", _s),
            }
        }
    }


    pub struct DataWriter {
        master_path: String,
        current_file: File,
        file_type: String,
        file_number: i32,

        f_ln_written: i32,
        t_ln_written: i32,
        max_size_per_file: u64,
        data_range: [i32; 2],

        header_written: bool,
        headers: Vec<String>,
        header_assigned: bool,
    }

    impl DataWriter {
        pub fn new(file_type: &str, base_file_path: Option<&str>) -> Self {
            let master_path = DataWriter::create_output_dir(base_file_path).unwrap();
            let file_number = 0;
            DataWriter {
                master_path: master_path.clone(),
                file_number,
                file_type: file_type.clone().parse().unwrap(),
                current_file: File::create(format!("{}/data{}.{}", &master_path, file_number, file_type)).unwrap(),
                f_ln_written: 0,
                max_size_per_file: 2_147_483_648,
                // max_size_per_file: 10_000_000,
                t_ln_written: 0,
                data_range: [0, 0],
                headers: Vec::new(),
                header_written: false,
                header_assigned: false,
            }
        }

        pub fn assign_headers(&mut self, headers: Vec<String>) -> Result<(), HeaderError> {
            // Check if file supports headers
            if self.file_type == "csv" {
                self.headers = headers;
                self.header_assigned = true;
                self.write_headers().unwrap();
                Ok(())
            } else {
                Err(HeaderError::FileTypeNotSupported(self.file_type.clone()))
            }
        }

        pub fn create_output_dir(base_folder_path: Option<&str>) -> std::io::Result<String> {
            let mut folder_number: i32 = 0;
            let mut folder_path;
            let mut result;

            if !(base_folder_path.is_none()) {
                match create_dir_all(base_folder_path.unwrap()) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }

            loop {
                folder_path = format!("{}/output_{}", base_folder_path.unwrap_or("."), folder_number);
                result = create_dir(&folder_path);
                match result {
                    Ok(_) => {
                        return Ok(format!("./{}", folder_path));
                    }
                    Err(error) => {
                        if error.kind() == std::io::ErrorKind::AlreadyExists {
                            folder_number += 1;
                        } else {
                            return Err(error);
                        }
                    }
                }
            }
        }

        pub fn write_data_using_headers(&mut self, _data: Vec<String>) -> Result<(), HeaderError> {
            if self.header_written == false {
                return Err(HeaderError::HeaderNotInitialized());
            }
            // TODO:Implement header to data conversion
            todo!()
        }

        pub fn write_data_using_array(&mut self, data: Vec<String>, add_new_line: Option<bool>) -> Result<(), DataWriterError> {
            self.check_if_file_is_full_and_update().unwrap();
            let mut data_string = String::new();
            for line in data.iter() {
                data_string.push_str(line);
                data_string.push_str(",");
            }
            if add_new_line.unwrap_or(true) {
                data_string.push_str("\n");
                self.f_ln_written += 1;
                self.t_ln_written += 1;
            }
            self.current_file.write_all(data_string.as_bytes()).unwrap();


            Ok(())
        }

        pub fn close_and_compress_output(&mut self) -> Result<(), DataWriterError> {
            self.close_current_file().unwrap();

            // TODO: Implement conversion form std error to data writer error
            // TODO: Implement proper naming of compressed file
            let tar_gz = File::create("archive.tar.gz").unwrap();
            let enc = GzEncoder::new(tar_gz, flate2::Compression::best());
            let mut tar = Builder::new(enc);
            // TODO: Implement conversion form std error to data writer error
            tar.append_dir_all("{}", &self.master_path).unwrap();
            tar.finish().unwrap();
            Ok(())
        }

        fn get_next_file(&mut self) -> Result<(), DataWriterError> {
            self.close_current_file().unwrap();

            self.file_number += 1;
            let _current_file_path = format!("{}/data{}.{}", self.master_path, self.file_number, self.file_type);
            if Path::new(&_current_file_path).exists() {
                return Err(DataWriterError::FileAlreadyExists(_current_file_path));
            }
            self.current_file = File::create(&_current_file_path).unwrap();
            self.header_written = false;
            self.f_ln_written = 0;

            Ok(())
        }

        fn check_if_file_is_full_and_update(&mut self) -> Result<(), HeaderError> {
            if self.current_file.metadata().unwrap().len() >= self.max_size_per_file {
                self.get_next_file().unwrap();
                if self.header_assigned {
                    self.write_headers().unwrap();
                }
            }
            Ok(())
        }

        fn write_headers(&mut self) -> Result<(), HeaderError> {
            if self.header_written == true {
                return Err(HeaderError::HeaderAlreadyWritten(self.headers.clone()));
            };
            if self.file_type == "csv" {
                let mut header_string = String::new();
                for header in self.headers.iter() {
                    header_string.push_str(header);
                    header_string.push_str(",");
                }
                header_string.push_str("\n");
                if self.f_ln_written == 0 {
                    self.current_file.write_all(header_string.as_bytes()).unwrap();
                    self.header_written = true;
                    self.t_ln_written += 1;
                    self.f_ln_written += 1;
                    Ok(())
                } else {
                    Err(HeaderError::TooLateToAddHeader(self.f_ln_written))
                }
            } else {
                Err(HeaderError::FileTypeNotSupported(self.file_type.clone()))
            }
        }

        fn close_current_file(&mut self) -> Result<(), DataWriterError> {
            self.current_file.flush().unwrap();
            self.current_file.sync_all().unwrap();
            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_header_error() {
            let mut writer = DataWriter::new("csv", Some("./testing/data_writer"));
            let headers = vec![String::from("a"), String::from("b"), String::from("c")];
            writer.assign_headers(headers).unwrap();
        }

        #[test]
        fn test_file_writer() {
            let mut writer = DataWriter::new("csv", Some("./testing/data_writer"));
            let headers = vec![String::from("a"), String::from("b"), String::from("c")];
            writer.assign_headers(headers).unwrap();
            let data = vec![String::from("1"), String::from("2"), String::from("3")];
            writer.write_data_using_array(data, None).unwrap();
            let mut data = vec![String::from("4"), String::from("5"), String::from("6")];
            writer.write_data_using_array(data, None).unwrap();
            for i in 1..=1000000 {
                data = vec![i.to_string(), (i + 1).to_string(), (i + 2).to_string()];
                writer.write_data_using_array(data, Some(true)).unwrap();
            }
            // assert_eq!(writer.f_ln_written, 3);
        }

        #[test]
        fn test_compress_function() {
            let mut writer = DataWriter::new("csv", Some("./testing/data_writer"));
            let headers = vec![String::from("a"), String::from("b"), String::from("c")];
            writer.assign_headers(headers).unwrap();
            let data = vec![String::from("1"), String::from("2"), String::from("3")];
            writer.write_data_using_array(data, None).unwrap();
            let mut data = vec![String::from("4"), String::from("5"), String::from("6")];
            writer.write_data_using_array(data, None).unwrap();
            for i in 1..=10000000 {
                data = vec![i.to_string(), (i + 1).to_string(), (i + 2).to_string()];
                writer.write_data_using_array(data, Some(true)).unwrap();
            }
            writer.close_and_compress_output().unwrap();
        }
    }
}

pub mod pi_math {
    use std::ops::{Add, Div, Mul, Sub};

    use rug::{Complete, Integer};
    use rug::ops::Pow;

    use crate::data_handler::DataWriter;


    pub struct CalcPi {
        pub n_start: i128,
        pub n_end: i128,

        mode: i8,
        recursion_ready: bool,

        data_handler: DataWriter,

        last_n: Integer,
        last_l: Integer,
        last_m: Integer,
        last_x: Integer,
        _k: Integer,
    }

    impl CalcPi {
        pub fn new(n_start: i128, n_end: i128, mode: i8, base_output_path: Option<&str>) -> Self {
            CalcPi {
                n_start,
                n_end,
                mode,
                recursion_ready: false,
                data_handler: DataWriter::new("csv", base_output_path),
                last_n: Integer::from(0),
                last_l: Integer::from(0),
                last_m: Integer::from(0),
                last_x: Integer::from(0),
                _k: Integer::from(0),
            }
        }

        pub fn calc_pi(&mut self) -> std::io::Result<()> {
            self.init_data_handler();
            for n in self.n_start..self.n_end {
                self.calc_l_m_x(Integer::from(n));
                self.write_most_recent_l_m_x();
            }
            self.data_handler.close_and_compress_output().unwrap();
            Ok(())
        }

        pub fn calc_pi_no_write(&mut self) -> std::io::Result<()> {
            self.init_data_handler();
            for n in self.n_start..self.n_end {
                self.calc_l_m_x(Integer::from(n));
            }

            Ok(())
        }

        fn calc_l_m_x(&mut self, n: Integer) {
            let _n: u32 = n.to_u32().unwrap();

            if self.recursion_ready {
                if self.last_n != Integer::sub(n.clone(), 1) {
                    println!("Recursion not ready at n={}, last_n={}", n, self.last_n);
                    self.recursion_ready = false;
                }
            }
            self.last_n = n;
            if !self.recursion_ready {

                // calc init m value

                let _q = Integer::factorial(6 * &_n).complete();
                let _w = Integer::factorial(3 * &_n).complete();
                let _e = Integer::pow(Integer::factorial(*&_n).complete(), 3);
                self.last_m = _q / (_w * _e);

                let _kh: i128 = -6 + (12 * &_n) as i128;
                self._k = Integer::from(_kh);

                // calc init l value
                let _a = Integer::mul(Integer::from(545140134), &_n);
                self.last_l = Integer::add(_a, 13591409);

                // calc init x value
                self.last_x = Integer::pow(Integer::from(-262537412640768000 as i64), &_n);

                self.recursion_ready = true;
            } else {
                self.last_l = Integer::from(&self.last_l + 545140134);
                self.last_x = Integer::from(&self.last_x * -262537412640768000 as i128);
                self._k = &self._k + Integer::from(12 * &_n);

                let _q = Integer::pow(Integer::from(&self._k), 3);
                let _w = Integer::mul(Integer::from(16), &self._k);
                let _e = Integer::pow(Integer::from(_n as i128), 3);

                let _num: Integer = Integer::sub(_q, _w);

                let _last_m_temp = self.last_m.clone();
                self.last_m = Integer::div(_num, _e);
                self.last_m = Integer::mul(Integer::from(&self.last_m), &_last_m_temp);
            }
        }

        fn write_most_recent_l_m_x(&mut self) {
            let data = vec!(self.last_n.to_string(), self.last_l.to_string(), self.last_m.to_string(), self.last_x.to_string());
            self.data_handler.write_data_using_array(data, Some(true)).unwrap();
        }

        fn init_data_handler(&mut self) {
            self.data_handler.assign_headers(vec!["n".to_string(), "l".to_string(), "m".to_string(), "x".to_string()]).unwrap();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;



        #[test]
        fn test_rug() {
            let a = Integer::from(1);
            println!("{}", a)
        }

        #[test]
        fn test_init_calc_l_m_x() {
            let test_path = Some("./testing");
            let mut _c = CalcPi::new(0 as i128, 1 as i128, 0, test_path);
            _c.calc_l_m_x(Integer::from(0));
            assert_eq!(_c.last_l, Integer::from(13591409));
            assert_eq!(_c.last_m, Integer::from(1));
            assert_eq!(_c.last_x, Integer::from(1));
            assert_eq!(_c.recursion_ready, true);
            println!("l: {}, x: {}, m: {}", _c.last_l, _c.last_x, _c.last_m);
        }

        #[test]
        fn test_recursive_calc_l_m_x() {
            let test_path = Some("./testing");
            let mut _c = CalcPi::new(0 as i128, 1 as i128, 0, test_path);
            _c.calc_l_m_x(Integer::from(0));
            _c.calc_l_m_x(Integer::from(1));
            // _c.calc_l_m_x(Integer::from(1));
            assert_eq!(_c.last_l, Integer::from(558731543));
            assert_eq!(_c.last_m, Integer::from(120));
            assert_eq!(_c.last_x, Integer::from(-262537412640768000 as i128));
            print!("l: {}, x: {}, m: {}", _c.last_l, _c.last_x, _c.last_m);
        }

        #[test]
        fn test_recursion_ready() {
            let test_path = Some("./testing");
            let mut _c = CalcPi::new(0 as i128, 1 as i128, 0, test_path);
            _c.calc_l_m_x(Integer::from(0));
            _c.calc_l_m_x(Integer::from(1));
            assert_eq!(_c.recursion_ready, true);
            _c.calc_l_m_x(Integer::from(3));
            assert_eq!(_c.recursion_ready, true);
            _c.calc_l_m_x(Integer::from(4));
            assert_eq!(_c.recursion_ready, true);
        }

        #[test]
        fn test_calc_pi() {
            let test_path = Some("./testing");
            let mut _c = CalcPi::new(0, 1000, 0, test_path);
            _c.calc_pi().unwrap();
        }
    }
}
