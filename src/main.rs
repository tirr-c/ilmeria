extern crate ilmeria;
#[macro_use]
extern crate clap;

fn main() {
    let _matches = clap_app!(ilmeria =>
        (version: "0.1.0")
        (author: "Wonwoo Choi <chwo9843@gmail.com>")
        (about: "Atelier Lydie & Suelle puzzle optimizer")
    ).get_matches();
}
