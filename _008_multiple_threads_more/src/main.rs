enum FileState {
    CannotAccessFile,
    FileOpenedAndReady,
    NoSuchFileExists,
    SimilarFileNameInNextDirectory,
} // awkwardly named:
fn give_filestate(input: &FileState) {
    use FileState::{
        CannotAccessFile as NoAccess, FileOpenedAndReady as 잘됨, NoSuchFileExists as NoFile,
        SimilarFileNameInNextDirectory as OtherDirectory,
    }; // since rust 2021

    match input {
        NoAccess => println!("Can't access file."),
        잘됨 => println!("Here is your file"),
        NoFile => println!("Sorry, there is no file by that name."),
        OtherDirectory => println!("Please check the other directory."),
    }
}

fn main() {
    use String as S;
    let my_string = S::from("Hi!");
}
