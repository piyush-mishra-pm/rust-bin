use std::{fs, io::Error};

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }
    return results;
}

fn main() -> Result<(), Error> {
    validation_and_strings_test_code();

    // Method 1: When can meanigfully deal with error.
    // match fs::read_to_string("logs.txt")  {
    //     Ok(parsed_text) => {
    //         println!("✅ Parsed successfully. Length {} chars.", parsed_text.len());
    //         let error_logs = extract_errors(parsed_text.as_str());
    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("✅ Written to file"),
    //             Err(reason_write_failed) => {
    //                 println!("❌ Error in writing errors.txt file: {}", reason_write_failed);
    //             }
    //         }
    //     },
    //     Err(why_failed) => {
    //         println!("❌ Failed to read: {}", why_failed);
    //     }
    // } ;

    // Method 2: unwrap, expect:
    // let text = fs::read_to_string("logs.txt")
    //         .expect("❌ Failed to read");
    // let error_logs = extract_errors(text.as_str());
    // fs::write("errors.txt", error_logs.join("\n")).expect("❌ Error in writing errors.txt file");

    // Method 3: When not method 1. Dont have a workaround for the error.
    let text = fs::read_to_string("sdlogs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;
    Ok(())
}

fn string_test(a:String, b:&String, c:&str){

}

fn validation_and_strings_test_code(){
    string_test("Red".to_string(), &String::from("Red"), String::from("Red").as_str());
    string_test("Red".to_string(), &String::from("Red"), &String::from("Red"));

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