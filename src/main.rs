#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::io;
use std::fs::File;
use rocket::response::NamedFile;
use rocket::request::Form;
use rocket_contrib::Template;

#[derive(Serialize)]
struct TemplateContext {
    feminine_words: Vec<String>,
    masculine_words: Vec<String>
}

#[derive(FromForm)]
struct Ad<'r> {
    ad_text: &'r str
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

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("templates/index.html")
}

#[post("/decode", data = "<ad_form>")]
fn decode<'a>(ad_form: Form<'a, Ad<'a>>) -> Template {
    let ad_text = ad_form.get().ad_text;
    let feminine_words = get_words("src/feminine_words.json");
    let masculine_words = get_words("src/masculine_words.json");
    let feminine_results = ad_decoder(ad_text, feminine_words);
    let masculine_results = ad_decoder(ad_text, masculine_words);

    let context = TemplateContext {
        feminine_words: feminine_results,
        masculine_words: masculine_results
    };

    Template::render("decoded", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, decode])
        .launch();
}
