use std::fs;

pub fn read_file(file_name: &str) -> String {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    let folder_name = current_dir.join(dotenvy::var("UPLOAD_FOLDER").unwrap());
    let file_path = folder_name.join(file_name);
    // fs::read_to_string(file_path).expect("Error reading file");
    file_path.to_str().unwrap().to_string()
}

pub fn write_file(file_path: &str, content: &str) {
    fs::write(file_path, content).expect("Error writing file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path};
    use std::path::Path;

    #[test]
    pub fn test() {
        let file = read_file("test.txt");
        dbg!(file);
    }
}
