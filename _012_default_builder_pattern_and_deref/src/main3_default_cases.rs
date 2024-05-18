// Case 1: Using Default in Functions or Traits
#[derive(Default, Debug)]
struct MyStruct {
    value: i32,
}

// Case 2: Using Default as a Parameter in Another Struct or Enum
// When you want to implement Default for a struct that has parameters, those parameters must also implement Default.
#[derive(Default, Debug)]
struct InnerStruct {
    value: i32,
}

#[derive(Default, Debug)]
struct OuterStruct {
    inner: InnerStruct, // InnerStruct must implement Default
}

// Case 3: Providing a General Usage Pattern with Default
// Implementing Default provides users with an easy way to create an instance of your type without needing to think about all the settings.
#[derive(Default, Debug)]
struct Config {
    host: String,
    port: u16,
}

impl Config {
    fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
}
fn main() {
    // Case 1:
    let optional_value: Option<MyStruct> = None;
    let value = optional_value.unwrap_or_default(); // Uses Default implementation
    println!("{:?}", value);

    // Case 2:
    let outer = OuterStruct::default(); // Uses Default implementation
    println!("{:?}", outer);

    // Case 3:
    let default_config = Config::default(); // Uses Default implementation
    let custom_config = Config::new("localhost".to_string(), 8080);
    println!("Default: {:?}", default_config);
    println!("Custom: {:?}", custom_config);

    // Case 4:
    let default_settings = Settings::default(); // Uses Default implementation
    let custom_settings = Settings::new(50, 70, "English".to_string());
    println!("Default: {:?}", default_settings);
    println!("Custom: {:?}", custom_settings);
    
    // Convenient:
    let only_height = Size {
	height: 1.0,
	..Default::default()
	};
	println!("{only_height:?}");
}

// Case 4: Convenience in Struct Parameters
// The Default trait makes it convenient to initialize parameters in a struct, especially when there are many fields.
#[derive(Default, Debug)]
struct Settings {
    brightness: u8,
    volume: u8,
    language: String,
}

impl Settings {
    fn new(brightness: u8, volume: u8, language: String) -> Self {
        Self {
            brightness,
            volume,
            language,
        }
    }
}
#[derive(Debug, Default)]
struct Size {
    height: f64,
    length: f64,
    width: f64,
}
