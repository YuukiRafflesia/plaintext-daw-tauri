use std::{fs::File, io::Read};

use crate::project::Project;

#[tauri::command]
pub fn render_project(path: &str) {
    println!("Project File Path: {path}");

    let proj_file = File::open(path);
    match proj_file {
        Err(e) => eprintln!("Error: Couldn't open project file...\n{e}"),
        Ok(mut f) => {
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();

            let proj: Project = serde_yaml::from_str(buf.as_str()).unwrap();
            println!("Project File Deserialised: {proj:#?}");
        },
    }
}

fn render_to_file(data: Project) {
    // TODO: Render to WAV!
}
