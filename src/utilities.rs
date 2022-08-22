mod utilities {
    use std::fs::create_dir;
    fn create_output_dir() -> std::io::Result<i32> {
        let mut folder_number: i32 = 0;
        loop {
            match create_dir("output {}".format(folder_number)) {
                Ok(_) => Ok((folder_number)),
                Err(error) => continue,
            }
        }
    }
    mod tests{
        #[test]
        fn test_create_output_dir() {
            println!("{}", create_output_dir().unwrap());
        }
    }
}