pub mod utilities {
    use std::fs::{create_dir, File};
    use std::io::{BufReader, Read};

    fn create_output_dir() -> std::io::Result<String> {
        let mut folder_number: i32 = 0;
        let mut folder_path;
        let mut result;
        loop {
            folder_path = format!("output_{}", folder_number);
            result = create_dir(&folder_path);
            match result {
                Ok(_) => {
                    return Ok(folder_path);
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

    pub fn sum_from_file(filename: &str, mut err_mode: i32) -> std::io::Result<i32> {
        // check if error mode is valid
        err_mode = err_mode;
        if !(err_mode == 0 || err_mode == 1){dbg!("Error mode not supported, using default"); err_mode = 0;}

        let file_obj = File::open(filename);
        match file_obj {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                reader.read_to_string(&mut contents)?;
                let mut sum= 0;
                for line in contents.lines() {
                    match line.parse::<i32>() {
                        Ok(num) => sum += num,
                        Err(_) => {
                            if err_mode == 1 {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "Invalid data",
                                ));
                            }else if err_mode == 0 {continue}
                        },
                    }
                }
                Ok(sum)
            }
            Err(error) => Err(error),
        }

    }

    #[cfg(test)]
    mod tests{
        use std::fs;
        use std::io::Write;
        use rug::rand;
        use super::*;
        #[test]
        fn test_create_output_dir() {
            create_output_dir().unwrap();
        }
        // #[test]
        // fn test_sum_from_file() {
        //     let mut _test_file = fs::OpenOptions::new()
        //         .write(true)
        //         .create(true)
        //         .open("test_sum_from_file.txt")
        //         .unwrap();
        //     let mut vec: Vec<i64> = Vec::with_capacity(39);
        //     for _ in 0..vec.capacity() {
        //         vec.push(rand::random());
        //     };
        //     _test_file.write_all(_array.to_string().as_bytes()).unwrap();
        //     let result = sum_from_file("input.txt", 0);
        //     assert_eq!(result.unwrap(), _array.sum());
        // }
    }
}