use std::fs;
use std::fs::File;
use std::io::Read;
fn main() -> std::io::Result<()> {
	// Text, text, text
	let main = include_str!("main.rs");
	println!("Here's what main.rs looks like:\n\n{main}");
	
    fs::write(
        "calvin_with_dad.txt",
        "Calvin: Dad, how come old photographs are always black and white? Didn't
they have color film back then?
Dad: Sure they did. In fact, those photographs are in color. It's just the
world was black and white then.",
    )?;
    let mut calvin_file = File::open("calvin_with_dad.txt")?;
    let mut calvin_string = String::new();
    calvin_file.read_to_string(&mut calvin_string)?;
    calvin_string
        .split_whitespace()
        .for_each(|word| print!("{} ", word.to_uppercase()));

    fs::write(
        "calvin_with_dad2.txt",
        "Calvin: Dad, how come old photographs are always black and white? Didn't
	   ➥they have color film back then?
	   Dad: Sure they did. In fact, those photographs are in color. It's just the
	   ➥world was black and white then.",
    )?;
    let calvin_file = File::options()
        .write(true)
        .create_new(true)
        .open("calvin_with_dad2.txt")?;

    
    Ok(())
}
