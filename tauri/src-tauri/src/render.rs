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

            render_to_file(proj, path);
        },
    }
}

fn render_to_file(proj: Project, path: &str) {
    // TODO: Render to WAV!

    let mut all_notes = vec![];
    for (name, pat) in proj.patterns() {
        for note in pat.notes() {
            all_notes.push(note);
        }
    }

    let mut song_data = vec![];
    for note in all_notes {
        // TODO: Make this work for songs with multiple instruments!
        let instrument_name = proj.patterns().iter().collect::<Vec<_>>()[0].1.instrument();
        let instrument_notes = proj.instruments().iter().collect::<Vec<_>>()[0].1.clips();

        let start = note.start_sample(proj.bpm(), proj.sample_rate());
        let end = note.end_sample(proj.bpm(), proj.sample_rate());

        // TODO: Check for clip endings and make it end there if needs be
        
        // TODO: Extend the vec with each note!
        song_data.extend_from_slice(other);
    }
}
