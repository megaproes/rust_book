use tokio::try_join;
async fn wait_then_u8(num: u8, worked: bool) -> Result<u8, &'static str> {
    if worked {
        Ok(num)
    } else {
        Err("Oops, didn't work")
    }
}
#[tokio::main]
async fn main() {
    let failed_join = try_join!(
        wait_then_u8(1, true),
        wait_then_u8(2, false),
        wait_then_u8(3, true)
    );
    let successful_join = try_join!(
        wait_then_u8(1, true),
        wait_then_u8(2, true),
        wait_then_u8(3, true)
    );
    println!("{failed_join:?}");
    println!("{successful_join:?}");
}
