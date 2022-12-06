use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Cell, Color, Table,
};
use rocksdb::DB;
use tman::{
    add_todo, add_todo_note, add_todo_tag, complete_todo, delete_todo, drop_db, edit_todo_note,
    get_all_todos, remove_todo_note, remove_todo_tag, uncomplete_todo, Status, cli,
};

fn main() {
    let binding = dirs::home_dir().unwrap();
    let path = binding.to_str().unwrap().to_string() + "/.tman";
    let db = DB::open_default(&path).unwrap();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            if let Err(e) = add_todo(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("list", _)) => {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_SOLID_INNER_BORDERS)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(vec!["Name", "Status", "Note", "Tags"]);

            let todos = get_all_todos(&db);
            for todo in todos {
                let name = Cell::new(todo.name);
                let cell = match todo.status {
                    Status::InProgress => Cell::new(todo.status.to_string()).fg(Color::Red),
                    Status::Done => Cell::new(todo.status.to_string()).fg(Color::Green),
                };
                table.add_row(vec![
                    name,
                    cell,
                    Cell::new(todo.note),
                    Cell::new(todo.tags.join(", ")),
                ]);
            }

            println!("{table}");
        }
        Some(("complete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            if let Err(e) = complete_todo(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("uncomplete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            if let Err(e) = uncomplete_todo(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("add-note", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let note = sub_matches.get_one::<String>("NOTE").expect("required");
            if let Err(e) = add_todo_note(&db, &key, &note) {
                println!("{}", e);
            }
        }
        Some(("edit-note", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let new_note = sub_matches.get_one::<String>("NOTE").expect("required");
            if let Err(e) = edit_todo_note(&db, &key, &new_note) {
                println!("{}", e);
            }
        }
        Some(("remove-note", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            if let Err(e) = remove_todo_note(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("add-tag", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let tag = sub_matches.get_one::<String>("TAG").expect("required");
            if let Err(e) = add_todo_tag(&db, &key, &tag) {
                println!("{}", e);
            }
        }
        Some(("remove-tag", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let tag = sub_matches.get_one::<String>("TAG").expect("required");
            if let Err(e) = remove_todo_tag(&db, &key, &tag) {
                println!("{}", e);
            }
        }
        Some(("delete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");

            if let Err(e) = delete_todo(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("drop-db", _)) => {
            if let Err(e) = drop_db(&path) {
                println!("{}", e);
            }
        }
        _ => unreachable!(),
    }
}
