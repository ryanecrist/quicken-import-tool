mod csv;
mod quicken;
mod sqlite3;

fn main() {

    let quicken_file_arg = std::env::args().nth(2).expect("quicken file should be provided via --quicken-file argument");
    let mut quicken_file_path = std::path::PathBuf::from(quicken_file_arg);
    quicken_file_path.push("data");
    let quicken_file = quicken_file_path.to_str().expect("quicken file path should exist");

    let mut quicken = quicken::Quicken::new(String::from(quicken_file));
    match quicken.open() {
        Ok(_) => (),
        Err(error) => panic!("failed to open quicken file {error:?}"),
    };
    println!("opened quicken file {}", quicken_file);

    let accounts = quicken.accounts().unwrap(); // TODO: don't force unwrap
    for account in accounts.iter() {
        println!("{}, {}, {}, {}", account.creation_timestamp, account.id, account.modification_timestamp, account.name);
    }

    match quicken.close() {
        Ok(_) => (),
        Err(error) => panic!("failed to close quicken file {error:?}"),
    };
    println!("closed quicken file {}", quicken_file);
}
