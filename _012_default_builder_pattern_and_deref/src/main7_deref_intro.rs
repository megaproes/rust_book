struct File(String);
fn main() {
    let my_file = File(String::from("I am file contents"));
    let my_string = String::from("I am file contents");
}
// We noted that the File struct holds a String, but it can’t use any of String’s methods.
// If you are just writing a bit of code in a single file, then you can, of course, use .0 to
// access the String inside. But if File is inside another mod and isn’t written struct
// File(pub String);, you won’t be able to use .0 to access String, and you won’t be
// able to use any of String’s methods. This is where the Deref trait comes in, so let’s
// take a look at how that works.

