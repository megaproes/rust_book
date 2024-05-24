//  column! gives you column number where the macro is called.
//  file! gives the filename in which the macro is called.
//  line! gives the line number in which the macro is called.
//  module_path! gives the path to the module.

pub mod input_handling {
    pub struct User {
        pub name: String,
        pub bank: Bank,
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Bank {
        BankOfAmerica,
        Hsbc,
        Citigroup,
        DeutscheBank,
        TorontoDominionBank,
        SiliconValleyBank,
    }
    pub mod user_input {
        use crate::input_handling::{Bank, User};
        pub fn handle_user_input(user: &User) -> Result<(), ()> {
            match user.bank {
                Bank::SiliconValleyBank => {
                    println!(
                        "Darn it, looks like we have to handle this variant even though Silicon Valley Bank doesn't exist anymore: {}:{}:{}:{}",
                        module_path!(),
                        file!(),
                        column!(),
                        line!()
                    );
                    Ok(())
                }
                other_bank => {
                    println!("{other_bank:?}, no problem");
                    Ok(())
                }
            }
        }
    }
}


use crate::input_handling::{user_input::handle_user_input, Bank, User};
fn main() {
    let user = User {
        name: "SomeUser".to_string(),
        bank: Bank::SiliconValleyBank,
    };
    handle_user_input(&user).unwrap();
    let user2 = User {
        name: "SomeUser2".to_string(),
        bank: Bank::TorontoDominionBank,
    };
    handle_user_input(&user2).unwrap();
}
