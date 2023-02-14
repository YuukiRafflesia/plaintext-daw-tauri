use std::{fs::File, io::Read, path::Path, process::Command};

use wav::{BitDepth, Header, WAV_FORMAT_PCM};

use crate::project::Project;

#[tauri::command]
pub fn render_project(path: &str, backend: &str) {
    println!("Project File Directory: {path}");
    let proj_file_path = Path::new(path).join("song.yml");

    let proj_file = File::open(&proj_file_path);
    match proj_file {
        Err(e) => eprintln!("Error: Couldn't open project file...\n{e}"),
        Ok(mut f) => {
            match backend {
                "rs" => {
                    let mut buf = String::new();
                    f.read_to_string(&mut buf).unwrap();
        
                    let proj: Project = serde_yaml::from_str(buf.as_str()).unwrap();
                    println!("Project File Deserialised: {proj:#?}");
        
                    render_to_file(proj, path);
                },
                "py" => {
                    // TODO: Fix this creating a blank, unclosable window
                    println!("{:?}",
                        Command::new("plaintext-daw")
                            .arg("render")
                            .arg(&proj_file_path)
                            .output()
                            .unwrap()
                    );
                },
                _ => eprintln!("Unrecognised backend: {backend}"),
            }
        },
    }
}

fn render_to_file(proj: Project, path: &str) {
    let mut all_notes = vec![];
    for pat in proj.patterns().values() {
        for note in pat.notes() {
            all_notes.push(note);
        }
    }

    // TODO: Make this work for songs with multiple instruments!
    let instrument_name = proj.patterns().iter().collect::<Vec<_>>()[0].1.instrument();
    let used_notes = proj.instruments()[instrument_name].clips();

    let mut song_data = vec![];
    for note in all_notes {
        let start = note.start_sample(proj.bpm(), proj.sample_rate());
        let end = note.end_sample(proj.bpm(), proj.sample_rate());

        // TODO: Handle these unwraps properly and make sure (idr what I was gonna put here, will write in when I remember)
        println!("Loading note from path: {}/{}", path, used_notes[note.value()].path());
        let mut input_note = File::open(Path::new(path).join(used_notes[note.value()].path())).unwrap();
        let (_in_header, data) = wav::read(&mut input_note).unwrap();

        if data.is_eight() {
            println!("Wav is 8bit int!");
        } else if data.is_sixteen() {
            println!("Wav is 16bit int!");
        } else if data.is_twenty_four() {
            println!("Wav is 24bit int!");
        } else if data.is_thirty_two_float() {
            println!("Wav is 32bit float!");
        }

        // TODO: Check for clip endings and make it end there if needs be
        
        // TODO: Extend the vec with each note!
        song_data.extend_from_slice(data.as_sixteen().unwrap());
    }

    println!("Song completed! Just rendering now...");

    let out_header = Header::new(
        WAV_FORMAT_PCM,
        1,
        proj.sample_rate(),
        16,
    );

    let mut output_song = File::create(Path::new(path).join("out.wav")).unwrap();
    wav::write(out_header, &BitDepth::Sixteen(song_data), &mut output_song).unwrap();

    println!("Song rendered!");
}
