use std::io::Write;
use text_io::read;

const DATA_DIR: &str = ".mathlock";
const NAME_FILE: &str = "name";
const SECRET_FILE: &str = "secret";

fn data_dir() -> std::io::Result<std::path::PathBuf> {
    let mut path = home::home_dir().unwrap();
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

fn parse_line_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    let data = std::fs::read_to_string(path)?; // this was originally a temporary, but complier recommended I use "let" so it survives until end of function
    let mut lines = data.lines(); // that was important because from here on, we're dealing with slices (&str)
    let line = lines.next();

    Ok(line.unwrap().trim().to_string()) // rust strings are so cool
}

fn parse_paths() {
    let current = std::env::current_dir().unwrap();
    println!("current: {:?}", current);

    let home = home::home_dir().unwrap();
    println!("home: {:?}", home);

    let data = data_dir().unwrap();
    println!("data: {:?} -- {}", data, data.exists());

    print!("How are you? ");
    std::io::stdout().flush().unwrap(); // flush needed std::io::Write for some reason
    let stu: String = read!("{}\n"); // without the \n, it reads first word. needs trimming after
    println!("You are: {}", stu.trim());

    let name = name_file().unwrap();
    println!("name: {:?} -- {}", name, name.exists());

    /*
    // checking first, but we don't need that
    if name.exists() {
        //let name_data: String = std::fs::read_to_string(name).unwrap();
        //println!("the data: {}", name_data.trim()); // should only use the first line, and trim
        println!("the data: {}", parse_line_file(name).unwrap());
    }
    */

//    println!("the data: {}", parse_line_file(name).unwrap()); // this fails when file doesn't exist
    println!("the data: {}", parse_line_file(name).unwrap_or("--".to_string()));

    let secret = secret_file().unwrap();
    println!("secret: {:?} -- {}", secret, secret.exists());
}

fn main() -> std::io::Result<()> {
    parse_paths();
    Ok(())
}
