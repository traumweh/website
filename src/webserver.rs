use crate::config::{
    get_string, get_value, ArcMutHashMap, CHORDS_DIR, CHORDS_JSON, MUSIC_JSON, SOURCES_DIR,
    STATIC_DIR, THOUGHTS_JSON,
};
use rocket::{
    fs::{FileServer, NamedFile},
    response::Redirect,
    Build, Rocket, State,
};
use rocket_dyn_templates::{context, Template};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

pub fn start(
    jsons: &ArcMutHashMap<PathBuf, (Value, SystemTime)>,
    chords: &ArcMutHashMap<PathBuf, (String, SystemTime)>,
) -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![home, legal, thoughts, selfcare, chords, fsm, favicon],
        )
        .mount("/static", FileServer::from(STATIC_DIR))
        .mount("/sources", FileServer::from(SOURCES_DIR))
        .manage(Arc::clone(jsons))
        .manage(Arc::clone(chords))
        .attach(Template::fairing())
}

#[get("/")]
fn home() -> Template {
    Template::render(
        "home",
        context! {
            lang: "en",
            title: "traumweh",
            css: vec!["/sources/main.css"],
            js: vec!["/sources/main.js"],
        },
    )
}

#[get("/legal")]
fn legal() -> Template {
    Template::render(
        "legal",
        context! {
            lang: "en",
            title: "legal.stuff",
            css: vec!["/sources/main.css"],
            js: vec![] as Vec<&str>,
        },
    )
}

#[get("/thoughts")]
fn thoughts(
    jsons: &State<ArcMutHashMap<PathBuf, (Value, SystemTime)>>,
    _chords: &State<ArcMutHashMap<PathBuf, (String, SystemTime)>>,
) -> Template {
    Template::render(
        "thoughts",
        context! {
            lang: "en",
            title: "thoughts...",
            css: vec!["/sources/main.css", "/sources/thoughts.css"],
            js: vec!["/sources/main.js"],
            thoughts: match get_value(&PathBuf::from(THOUGHTS_JSON), jsons.as_ref()) {
                Some(value) => value,
                None => Value::Null,
            },
        },
    )
}

#[get("/selfcare")]
fn selfcare(
    jsons: &State<ArcMutHashMap<PathBuf, (Value, SystemTime)>>,
    _chords: &State<ArcMutHashMap<PathBuf, (String, SystemTime)>>,
) -> Template {
    Template::render(
        "selfcare",
        context! {
            lang: "en",
            title: "selfcare",
            css: vec!["/sources/main.css"],
            js: vec!["/sources/main.js"],
            music: match get_value(&PathBuf::from(MUSIC_JSON), jsons.as_ref()) {
                Some(value) => value,
                None => Value::Null,
            },
        },
    )
}

#[get("/chords")]
fn chords(
    jsons: &State<ArcMutHashMap<PathBuf, (Value, SystemTime)>>,
    chords: &State<ArcMutHashMap<PathBuf, (String, SystemTime)>>,
) -> Template {
    let chords_json = match get_value(&PathBuf::from(CHORDS_JSON), jsons.as_ref()) {
        Some(value) => value,
        None => Value::Null,
    };

    let mut chords_array: Vec<Value> = Vec::new();

    for (i, chord) in chords_json.as_array().unwrap().iter().enumerate() {
        let chord_object = chord.as_object().unwrap();
        let file_name = chord_object.get("file").unwrap().as_str().unwrap();
        let path = PathBuf::from(CHORDS_DIR).join(file_name);
        let text: String = get_string(&path, chords.as_ref()).expect("Invalid chord file");

        chords_array.push(json!({
            "lang": chord_object.get("lang"),
            "id": i,
            "title": chord_object.get("title"),
            "transpose": chord_object.get("transpose"),
            "text": text,
        }));
    }

    Template::render(
        "chords",
        &context! {
            lang: "en",
            title: "songbook",
            css: vec!["/sources/main.css"],
            js: vec!["/sources/main.js"],
            chords: chords_array,
        },
    )
}

#[get("/fsm")]
fn fsm() -> Redirect {
    Redirect::to("https://traumweh.github.io/fsm/www/")
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("./static/images/favicon.ico").await.ok()
}
