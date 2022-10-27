extern crate skim;
use skim::prelude::*;
use std::io::Cursor;

pub(crate) fn select_item(input: String) -> Option<String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))?;

    if selected_items.is_abort {
        return None;
    }

    //NOTE: Can it fail?
    Some(selected_items.selected_items[0].output().to_string())
}
