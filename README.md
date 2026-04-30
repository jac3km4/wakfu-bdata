# wakfu-bdata

Binary parsers for Wakfu game data files.

This crate (`wakfudecrypt`) provides a safe and easy-to-use Rust interface for reading and decoding Wakfu's binary data files. It allows developers to extract in-game information such as items, item sets, pets, effects, and more, directly from the game client files.

## Features

- **Read Wakfu binary data:** Easily load binary data documents from the game's contents.
- **Strictly Typed:** Game data objects are typed and automatically generated.
- **Serialization:** Types implement `serde::Serialize` and `serde::Deserialize` for easy conversion to formats like JSON.

## Usage

Here is a basic example of how to extract items, item sets, pets, and static effects from the game's data files.
You can find this example in [`src/bin/main.rs`](src/bin/main.rs).

```rust
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

    // Write the document elements to a formatted JSON file
    serde_json::to_writer_pretty(
        File::create(Path::new(name).with_extension("json")).unwrap(),
        &doc.elements,
    )
    .unwrap();
}
```

To run this example, pass the path to your Wakfu game installation directory:

```bash
cargo run --bin main -- /path/to/Wakfu
```
*(This will generate `.json` files in your current working directory for the dumped types.)*

## Data Types

The binary data types can be automatically refreshed using [wakfu-bdata-gen](https://github.com/jac3km4/wakfu-bdata-gen).

The types are located in the `wakfudecrypt::types` module. You can use any of the available models, such as `Item`, `ItemSet`, `Pet`, `StaticEffect`, and others.
