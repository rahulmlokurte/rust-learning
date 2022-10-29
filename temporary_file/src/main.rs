use std::env::temp_dir;
use std::fs::File;
use std::io::Result;
use uuid::Uuid;

fn main() -> Result<()> {
    let mut dir = temp_dir();
    println!("{}", dir.to_str().unwrap());

    let file_name = format!("{}.txt", Uuid::new_v4());
    println!("{}", file_name);
    dir.push(file_name);

    let file = File::create(dir)?;
    Ok(())
}
