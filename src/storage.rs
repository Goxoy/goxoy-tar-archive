use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, MAIN_SEPARATOR};
use tar::{Archive, Builder};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Storage {
    pub file_name: String,
}

impl Storage {
    pub fn new(archive_file_name: &str) -> Self {
        _ = fs::create_dir("storage_temp");
        Storage {
            file_name: format!("{}.tar", archive_file_name),
        }
    }
    pub fn remove_file(&mut self, file_name: &str) -> bool {
        let inner_obj = File::open(self.file_name.clone());
        let mut file_list = Vec::new();

        let mut arc_obj = Archive::new(inner_obj.unwrap());
        let mut file_founded = false;
        for inner_file in arc_obj.entries().unwrap() {
            let mut inner_file_obj = inner_file.unwrap();
            let inner_file_name = inner_file_obj
                .header()
                .path()
                .unwrap()
                .display()
                .to_string();
            if inner_file_name.eq(&file_name.clone()) == false {
                file_list.push(inner_file_name.clone());
                let mut s = String::new();
                inner_file_obj.read_to_string(&mut s).unwrap();
                let mut write_file = File::create(format!(
                    "storage_temp{}{}",
                    MAIN_SEPARATOR,
                    inner_file_name.clone()
                ))
                .unwrap();
                _ = write_file.write_all(s.as_bytes());
            } else {
                file_founded = true;
            }
        }

        if file_founded == true {
            file_list.sort();
            let new_file_obj = File::create(self.file_name.clone()).unwrap();
            let mut new_build = Builder::new(new_file_obj);
            for item in file_list.iter() {
                _ = new_build.append_file(
                    item.clone(),
                    &mut File::open(format!("storage_temp{}{}", MAIN_SEPARATOR, item.clone()))
                        .unwrap(),
                );
            }
        }

        for item in file_list.iter() {
            _ = fs::remove_file(format!("storage_temp{}{}", MAIN_SEPARATOR, item.clone()));
        }
        true
    }
    pub fn add_file(&mut self, file_path: &str) -> bool {
        self.add_file_process(
            String::from(file_path),
            String::from(Path::new(&file_path).file_name().unwrap().to_str().unwrap()),
        )
    }
    pub fn add_file_with_name(&mut self, file_path: &str, new_name: &str) -> bool {
        self.add_file_process(String::from(file_path), String::from(new_name))
    }
    fn add_file_process(&mut self, file_path: String, new_name: String) -> bool {
        if Path::new(&file_path.clone()).exists() == false {
            return false;
        }

        let inner_obj = File::open(self.file_name.clone());
        if inner_obj.is_err() {
            let new_file_obj = File::create(self.file_name.clone()).unwrap();
            let mut new_build = Builder::new(new_file_obj);
            let added = new_build.append_file(
                new_name.clone(),
                &mut File::open(file_path.clone()).unwrap(),
            );
            return if added.is_ok() { true } else { false };
        }

        let mut file_list = vec![new_name.clone()];

        let mut arc_obj = Archive::new(inner_obj.unwrap());
        for inner_file in arc_obj.entries().unwrap() {
            let mut inner_file_obj = inner_file.unwrap();
            let inner_file_name = inner_file_obj
                .header()
                .path()
                .unwrap()
                .display()
                .to_string();
            if inner_file_name.eq(&new_name.clone()) == false {
                file_list.push(inner_file_name.clone());
                let mut s = String::new();
                inner_file_obj.read_to_string(&mut s).unwrap();
                let mut write_file = File::create(format!(
                    "storage_temp{}{}",
                    MAIN_SEPARATOR,
                    inner_file_name.clone()
                ))
                .unwrap();
                _ = write_file.write_all(s.as_bytes());
            }
        }

        file_list.sort();
        let new_file_obj = File::create(self.file_name.clone()).unwrap();
        let mut new_build = Builder::new(new_file_obj);
        for item in file_list.iter() {
            if item.eq(&new_name.clone()) {
                _ = new_build.append_file(
                    new_name.clone(),
                    &mut File::open(file_path.clone()).unwrap(),
                );
            } else {
                _ = new_build.append_file(
                    item.clone(),
                    &mut File::open(format!("storage_temp{}{}", MAIN_SEPARATOR, item.clone()))
                        .unwrap(),
                );
            }
        }
        for item in file_list.iter() {
            if item.eq(&new_name.clone()) == false {
                _ = fs::remove_file(format!("storage_temp{}{}", MAIN_SEPARATOR, item.clone()));
            }
        }
        return true;
    }
}
#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    let mut storage_obj = Storage::new("archive");
    let result1 = storage_obj.add_file_with_name("file2.txt", "example.txt");
    let result3 = storage_obj.add_file_with_name("file3.txt", "sky.md");
    let result2 = storage_obj.add_file_with_name("file1.txt", "rust.rtf");
    if result1 == true && result2 == true && result3 == true {
        let result4 = storage_obj.remove_file("sky.md");
        if result4 == true {
            assert!(true)
        } else {
            assert!(false)
        }
    } else {
        assert!(false)
    }
}
