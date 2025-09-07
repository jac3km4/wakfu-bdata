use std::fs::File;
use std::path::Path;

use serde::Serialize;
use wakfudecrypt::data::BinaryData;
use wakfudecrypt::document::Document;
use wakfudecrypt::types::item::Item;
use wakfudecrypt::types::item_set::ItemSet;
use wakfudecrypt::types::pet::Pet;
use wakfudecrypt::types::static_effect::StaticEffect;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Please provide a path to the game root dir");
    let path = Path::new(&arg);

    dump_doc::<Item>(path);
    dump_doc::<ItemSet>(path);
    dump_doc::<Pet>(path);
    dump_doc::<StaticEffect>(path);
    // Can add any other types from the `types` crate here.
}

fn dump_doc<A: BinaryData + Serialize>(path: &Path) {
    let doc = Document::<A>::load(path).unwrap();
    let name = std::any::type_name::<A>();
    let name = name.rsplit_once("::").map_or(name, |(_, n)| n);
    serde_json::to_writer_pretty(
        File::create(Path::new(name).with_extension("json")).unwrap(),
        &doc.elements,
    )
    .unwrap();
}
