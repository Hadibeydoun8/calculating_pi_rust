use std::fmt;
use std::fmt::Debug;
use std::fs::{create_dir, create_dir_all, File};
use std::io::Write;
use std::path::Path;

use flate2::write::GzEncoder;
use tar::Builder;

#[derive(Debug, PartialEq, Eq)]
pub enum HeaderError {
    FileTypeNotSupported(String),
    HeaderAlreadyWritten(Vec<String>),
    TooLateToAddHeader(i32),
    HeaderNotInitialized(),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DataWriterError {
    FileAlreadyExists(String),
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HeaderError::FileTypeNotSupported(_s) => {
                write!(f, "The file type {} does not support headers", _s)
            }
            HeaderError::HeaderAlreadyWritten(_v) => {
                write!(f, "The header: {:?} has already been written", _v)
            }
            HeaderError::TooLateToAddHeader(_i) => write!(
                f,
                "{} lines have already been written, cannot add header",
                _i
            ),
            HeaderError::HeaderNotInitialized() => write!(f, "The header has not been initialized"),
        }
    }
}

impl fmt::Display for DataWriterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataWriterError::FileAlreadyExists(_s) => write!(f, "The file {} already exists", _s),
        }
    }
}

pub struct ArchiveInfo {
    id: i32,
    batch_id: i32,
}

pub struct DataWriter {
    master_path: String,
    current_file: File,
    file_type: String,
    file_number: i32,

    f_ln_written: i32,
    t_ln_written: i32,
    max_size_per_file: u64,

    header_written: bool,
    headers: Vec<String>,
    header_assigned: bool,

    archive_info: Option<ArchiveInfo>,
}

impl DataWriter {
    pub fn new(file_type: &str, base_file_path: Option<&str>) -> Self {
        let master_path = DataWriter::create_output_dir(base_file_path).unwrap();
        let file_number = 0;
        DataWriter {
            master_path: master_path.clone(),
            file_number,
            file_type: file_type.to_owned().parse().unwrap(),
            current_file: File::create(format!(
                "{}/data{}.{}",
                &master_path, file_number, file_type
            ))
            .unwrap(),
            f_ln_written: 0,
            max_size_per_file: 2_147_483_648,
            // max_size_per_file: 10_000_000,
            t_ln_written: 0,
            headers: Vec::new(),
            header_written: false,
            header_assigned: false,

            archive_info: None,
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

        if base_folder_path.is_some() {
            create_dir_all(base_folder_path.unwrap())?;
        }

        loop {
            folder_path = format!(
                "{}/output_{}",
                base_folder_path.unwrap_or("."),
                folder_number
            );
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
        if !self.header_written {
            return Err(HeaderError::HeaderNotInitialized());
        }
        // TODO: implement header conversion
        Ok(())
    }

    pub fn write_data_using_array(
        &mut self,
        data: Vec<String>,
        add_new_line: Option<bool>,
    ) -> Result<(), DataWriterError> {
        self.check_if_file_is_full_and_update().unwrap();
        let mut data_string = String::new();
        for line in data.iter() {
            data_string.push_str(line);
            data_string.push(',');
        }
        if add_new_line.unwrap_or(true) {
            data_string.push('\n');
            self.f_ln_written += 1;
            self.t_ln_written += 1;
        }
        self.current_file.write_all(data_string.as_bytes()).unwrap();

        Ok(())
    }

    pub fn close_and_compress_output(&mut self) -> Result<(), DataWriterError> {
        self.close_current_file().unwrap();
        let archive_path: String;

        // TODO: Implement conversion form std error to data writer error
        if self.archive_info.is_some() {
            archive_path = format!(
                "pi_{}_{}.tar.gz",
                self.archive_info.as_ref().unwrap().batch_id,
                self.archive_info.as_ref().unwrap().id,
            );
        } else {
            archive_path = format!("archive.tar.gz");
        }
        let tar_gz = File::create(archive_path).unwrap();
        let enc = GzEncoder::new(tar_gz, flate2::Compression::best());
        let mut tar = Builder::new(enc);
        // TODO: Implement conversion form std error to data writer error
        tar.append_dir_all("{}", &self.master_path).unwrap();
        tar.finish().unwrap();
        Ok(())
    }

    pub fn set_archive_id(&mut self, id: i32, batch_id: i32) {
        self.archive_info = Some(ArchiveInfo { id, batch_id });
    }

    fn get_next_file(&mut self) -> Result<(), DataWriterError> {
        self.close_current_file().unwrap();

        self.file_number += 1;
        let _current_file_path = format!(
            "{}/data{}.{}",
            self.master_path, self.file_number, self.file_type
        );
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
        if self.header_written {
            return Err(HeaderError::HeaderAlreadyWritten(self.headers.clone()));
        };
        if self.file_type == "csv" {
            let mut header_string = String::new();
            for header in self.headers.iter() {
                header_string.push_str(header);
                header_string.push(',');
            }
            header_string.push('\n');
            if self.f_ln_written == 0 {
                self.current_file
                    .write_all(header_string.as_bytes())
                    .unwrap();
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
