fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    // This saves code by performing the error handling for us
    let v = do_something_that_might_fail(12)?;
    println!("found {}", v);
    Ok(())
}
