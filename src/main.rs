#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rand;

use std::io;
use std::fs::File;
use rocket::response::NamedFile;
use rocket::request::Form;
use rocket_contrib::Template;
use rocket::response::Redirect;
use std::io::Write;
use std::io::Read;
use rand::Rng;

#[derive(Serialize)]
struct TemplateContext {
    feminine_words: Vec<String>,
    masculine_words: Vec<String>,
    ad_text: String
}

#[derive(FromForm)]
struct Ad {
    ad_text: String
}

fn ad_decoder(ad: &str, word_list: Vec<String>) -> Vec<String> {
    let mut results = vec![];
    let ad_words = ad.split(|s: char| !s.is_alphabetic())
        .map(|s| s.to_lowercase())
        .collect::<Vec<_>>();

    for word in &word_list {
        if let Some(found) = ad_words.iter().find(|ad_word| ad_word.starts_with(word)) {
            results.push(found.to_string());
        }
    }

    results
}

fn get_words(path: &str) -> Vec<String> {
    let file = File::open(path).expect(format!("Failed to load: {}", path).as_str());
    serde_json::from_reader(file).expect(format!("Failed to deserialize json: {}", path).as_str())
}

const BASE62: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn get_id() -> String {
    let size = 10;
    let mut id = String::with_capacity(size);
    let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
    id
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("templates/index.html")
}

#[get("/<id>")]
fn get_by_id(id: String) -> Template {
    let mut ad_text = String::new();

    File::open(format!("uploads/{id}", id = id)).unwrap().read_to_string(&mut ad_text);
    
    let feminine_words = get_words("src/feminine_words.json");
    let masculine_words = get_words("src/masculine_words.json");
    let feminine_results = ad_decoder(&ad_text, feminine_words);
    let masculine_results = ad_decoder(&ad_text, masculine_words);

    let context = TemplateContext {
        feminine_words: feminine_results,
        masculine_words: masculine_results,
        ad_text: ad_text
    };

    Template::render("decoded", &context)
}

#[post("/decode", data = "<ad_form>")]
fn decode(ad_form: Form<Ad>) -> Redirect {
    let id = get_id();
    let path = format!("uploads/{id}", id = id);
    let ad_text = &ad_form.get().ad_text;

    File::create(path).unwrap().write_all(ad_text.as_bytes());

    Redirect::to(&format!("/{id}", id = id))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, decode, get_by_id])
        .launch();
}
