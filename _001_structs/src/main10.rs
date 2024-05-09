fn main() {
    //     let my_name: String = "Billy".to_string();
    //     let other_name: String = "Billy".to_string();
    //     println!("{}", my_name == other_name);
    //     println!("{}", my_name == other_name);

    let my_name: String = "Billy".to_string();
    let double_ref: &&String = &&my_name;
    let ref_to_string: &String = *double_ref;
    let simple_string: String = (**double_ref).clone();
    let just_ref: &String = &**double_ref;
    let stil_double_ref: &&String = &*double_ref;
    
    assert_eq!(double_ref, stil_double_ref);
    assert_eq!(&my_name, &**double_ref);
   
    
    
    
    
    
    
    
    println!("{}", (double_ref).is_empty());
}
