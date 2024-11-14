use std::env;
use std::io;

pub fn get_input_files() -> io::Result<(String, String, String)> {
    let exe_path = env::current_exe()?;
    let exe_name: String = exe_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .chars()
        .skip("quest".len())
        .collect();
    Ok((
        format!("inputs/everybody_codes_e2024_q{exe_name}_p1.txt"),
        format!("inputs/everybody_codes_e2024_q{exe_name}_p2.txt"),
        format!("inputs/everybody_codes_e2024_q{exe_name}_p3.txt"),
    ))
}
