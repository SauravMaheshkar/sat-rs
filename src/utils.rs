use std::io::Read;

pub fn read_file(path: &std::path::PathBuf) -> String {
    let mut buffer = String::new();

    let mut file = match std::fs::File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => (),
    }

    buffer
}
