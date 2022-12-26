use serde_json::Value;
use std::collections::HashMap;
use std::fs::{metadata, read_dir, read_to_string};
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

pub type ArcMutHashMap<K, V> = Arc<Mutex<HashMap<K, V>>>;

pub const MUSIC_JSON: &str = "data/music.json";
pub const CHORDS_JSON: &str = "data/chords/chords.json";
pub const THOUGHTS_JSON: &str = "data/thoughts.json";
pub const CHORDS_DIR: &str = "data/chords";
pub const SOURCES_DIR: &str = "data/sources";
pub const STATIC_DIR: &str = "static";

pub fn get_string<K: Eq + Hash, S: ::std::hash::BuildHasher>(
    key: &K,
    map: &Mutex<HashMap<K, (String, SystemTime), S>>,
) -> Option<String> {
    map.lock()
        .expect("Failed to lock mutex.")
        .get(key)
        .map(|pair| pair.0.clone())
}
pub fn get_value<K: Eq + Hash, S: ::std::hash::BuildHasher>(
    key: &K,
    map: &Mutex<HashMap<K, (Value, SystemTime), S>>,
) -> Option<Value> {
    map.lock()
        .expect("Failed to lock mutex.")
        .get(key)
        .map(|pair| pair.0.clone())
}

pub fn read_all(
    jsons: &ArcMutHashMap<PathBuf, (Value, SystemTime)>,
    chords: &ArcMutHashMap<PathBuf, (String, SystemTime)>,
) {
    read_texts(&PathBuf::from(CHORDS_DIR), chords);
    read_json(&PathBuf::from(MUSIC_JSON), jsons);
    read_json(&PathBuf::from(CHORDS_JSON), jsons);
    read_json(&PathBuf::from(THOUGHTS_JSON), jsons);

    thread::sleep(std::time::Duration::from_secs(15 * 60));
}

pub fn read_texts(dir: &PathBuf, map: &ArcMutHashMap<PathBuf, (String, SystemTime)>) {
    let map_arc = Arc::clone(map);
    let mut map_mut = map_arc.lock().expect("Failed to lock mutex.");

    read_dir(dir)
        .expect(format!("Directory doesn't exist: {}", dir.to_str().unwrap()).as_str())
        .for_each(|dir_entry| {
            let path = dir_entry.expect("DirEntry doesn't exist anymore.").path();
            let modified = metadata(&path)
                .expect("File not found")
                .modified()
                .expect("Unsupported platform");
            let pair = map_mut.get(&path);

            if pair.is_none() || modified > pair.expect("Some is actually None.").1 {
                let content = read_to_string(&path).expect("Error reading file");
                map_mut.insert(path.clone(), (content, modified));
            }
        });
}
pub fn read_json(path: &PathBuf, map: &ArcMutHashMap<PathBuf, (Value, SystemTime)>) {
    let modified = metadata(path)
        .expect("File not found")
        .modified()
        .expect("Unsupported platform");
    let jsons_arc = Arc::clone(map);
    let mut jsons_mut = jsons_arc.lock().expect("Failed to lock mutex.");
    let pair = jsons_mut.get(path);

    if pair.is_none() || modified > pair.expect("Some is actually None.").1 {
        let file = std::fs::File::open(path).expect("Error reading file");
        let reader = std::io::BufReader::new(file);
        let json: Value = serde_json::from_reader(reader).expect("Expected json value");

        jsons_mut.insert(path.clone(), (json, modified));
    }
}
