use std::{fs, io::Error};

fn main() {
    match divide(5.0, 0.0) {
        Ok(res) => {
            println!("✅ Div result: {:#?}", res);
        },
        Err(err) => {
            println!("❌Error: {:#?}", err);
        }
    }

    match validate_email(String::from("gmail.com")) {
        Ok(..)=> println!("✅ Email valid"),
        Err(err)=>println!("❌ Error: {:#?}", err)
    }
    match validate_email(String::from("pm@gmail.com")) {
        Ok(..)=> println!("✅ Email valid"),
        Err(err)=>println!("❌ Error: {:#?}", err)
    }

    match fs::read_to_string("logs.txt")  {
        Ok(parsed_text) => {
            println!("✅ Parsed successfully. Length {} chars.", parsed_text.len())
        },
        Err(why_failed) => {
            println!("❌ Failed to read: {}", why_failed)
        }
    } ;
}

fn divide(a:f64, b: f64) -> Result<f64, Error> {
    if b==0.0 {
        Err(Error::other("cant divide by 0"))
    } else{
        Ok(a/b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Email must have @"))
    }
}