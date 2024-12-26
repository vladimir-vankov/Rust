use std::fs::File;
use std::io::{self, Write, Read};

pub fn write_into_file(content: &str, file_name: &str) -> io::Result<()>{
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

pub fn read_from_file(file_name: &str) -> io::Result<String>{
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

pub fn slice_to_string(slice: &[u32]) -> String {
    slice.iter().map(|highscore| highscore.to_string()).collect::<Vec<String>>().join(" ")
}

pub fn save_highscores_and_lines(highscores: &[u32], number_of_lines: &[u32]) -> bool {
    let s_high_scores = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);
    write_into_file(format!("{} \n{}\n", s_high_scores, s_number_of_lines).as_str(), "score.txt").is_ok()
}

pub fn line_to_slice(line: &str) -> Vec<u32>{
    line.split(" ").filter_map(|nb| nb.parse::<u32>().ok()).collect()
}

pub fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)>{
    if let Ok(content) = read_from_file("score.txt"){
        let mut lines = content.splitn(2, "\n").
        map(|line| line_to_slice(line)).collect::<Vec<_>>();
        if lines.len() == 2 {
            let (number_lines, highscore) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((number_lines, highscore))
        }
        else{
            None
        }
    }
    else{
        None
    }
}
