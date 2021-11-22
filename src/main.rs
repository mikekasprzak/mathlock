const DATA_DIR: &str = ".mathlock";
const NAME_FILE: &str = "name";
const SECRET_FILE: &str = "secret";

fn data_dir() -> std::io::Result<std::path::PathBuf> {
    let mut path = home::home_dir().unwrap();

    // // add a dot (hidden) prefix to the executable name
    // let mut dir: OsString = OsString::new();
    // dir.push(".");
    // dir.push(std::env::current_exe()?.file_stem().unwrap()); // IMPORTANT: Should be file_prefix, but it's only available in nightly
    //
    // path.push(dir);

    path.push(DATA_DIR);

    Ok(path)
}

fn name_file() -> std::io::Result<std::path::PathBuf> {
    let mut path = data_dir()?;
    path.push(NAME_FILE);
    Ok(path)
}

fn secret_file() -> std::io::Result<std::path::PathBuf> {
    let mut path = data_dir()?;
    path.push(SECRET_FILE);
    Ok(path)
}

fn parse_paths() {
    let current = std::env::current_dir().unwrap();
    println!("current: {:?}", current);

    let home = home::home_dir().unwrap();
    println!("home: {:?}", home);

    let data = data_dir().unwrap();
    println!("data: {:?} -- {}", data, data.exists());

    let name = name_file().unwrap();
    println!("name: {:?} -- {}", name, name.exists());

    let secret = secret_file().unwrap();
    println!("secret: {:?} -- {}", secret, secret.exists());
}

fn main() -> std::io::Result<()> {
    parse_paths();
    Ok(())
}
