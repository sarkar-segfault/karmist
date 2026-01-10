#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => {{
        eprintln!("{}", console::style(format!($($arg)+)).red().bold());
        std::process::exit(1);
    }}
}

mod crud;
mod options;

fn main() {
    let opts: options::Options = argh::from_env();

    let mut db = serde_json::from_reader::<_, crud::Database>(
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&opts.db)
            .unwrap_or_else(|e| fatal!("failed to open database: {}", e)),
    )
    .unwrap_or_else(|e| {
        if e.is_eof() {
            crud::Database::new()
        } else {
            fatal!("failed to serialize database: {}", e)
        }
    });

    match opts.cmd {
        options::Command::Create(c) => crud::create(&mut db, c.id),
        options::Command::Read(r) => crud::read(&mut db, r.id),
        options::Command::Update(u) => crud::update(&mut db, u.id),
        options::Command::Delete(d) => crud::delete(&mut db, d.id),
    }

    std::fs::write(
        opts.db,
        serde_json::to_string(&db)
            .unwrap_or_else(|e| fatal!("failed to deserialize database: {}", e)),
    )
    .unwrap_or_else(|e| fatal!("failed to write to database: {}", e));
}
