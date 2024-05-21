use std::fs;
use std::io::Write;
fn main() -> std::io::Result<()> {
    // fs::File::create("myfilename.txt")? .write_all(b"Let's put this in the file")?
    let mut file = fs::File::create("myfilename.txt")?;
    file.write_all(b"Let's put this in the file")?;

    fs::write(
        "calvin_with_dad.txt",
        "Calvin: Dad, how come old photographs are always black and white? Didn't
    they have color film back then?
    Dad: Sure they did. In fact, those photographs are in color. It's just the
    world was black and white then.",
    )?;
    
    
    Ok(())
}
