pub mod inputs {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn fread_lines(file: &str) -> Vec<String> {
        return File::open(file)
            .map(BufReader::new)
            .map(|br| br.lines().map(|l| l.unwrap()))
            .unwrap()
            .collect::<Vec<String>>();
    }
}
