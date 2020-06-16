use std::{fs::File, path::Path};

use serde::Serialize;
use wakfudecrypt::{
    document::Document, types::{item::Item, item_set::ItemSet, pet::Pet, static_effect::StaticEffect}, BinaryData
};

fn main() {
    let arg = std::env::args().nth(1).expect("Please provide a file path");
    let path = Path::new(&arg);

    dump_doc::<Item>(&path);
    dump_doc::<ItemSet>(&path);
    dump_doc::<Pet>(&path);
    dump_doc::<StaticEffect>(&path);
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
