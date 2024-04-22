use std::{path::PathBuf, process::Command};

use crate::{templates::get_temp_root_path, TEMPLATE_DIR};

pub fn git_pull_template(git_path: &PathBuf, temp_name: &String) {
    let mut command = Command::new("git");
    let mut temp_dir = get_temp_root_path();
    temp_dir.push(TEMPLATE_DIR);
    command
        .current_dir(temp_dir)
        .arg("clone")
        .arg(git_path)
        .arg(temp_name)
        .output()
        .expect("git command failed to start");
    println!("{:?}模板下载成功", git_path);
}
