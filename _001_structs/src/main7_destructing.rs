#[derive(Clone /*Copy*/)] // not possible since Strings 'name' & 'real_name' contain heap data
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}
fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    let Person {
        name,
        real_name,
        height,
        happiness,
    } = papa_doc.clone();
    println!( "They call him {name} but his real name is {real_name}. He is {height} cm tall and is he happy? {happiness}");

    let Person {
        name: fake_name,
        real_name,
        height: cm,
        happiness,
    } = &papa_doc;
}
