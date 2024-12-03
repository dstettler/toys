use std::{fs::File, io::{self, Read, Write}};

fn encode(plain_string: &String) -> String {
    let mut result = String::new();
    for c in plain_string.chars() {
        if c as u8 <= 90 && c as u8 >= 65 {
            result.push((((c as u8 - 65 + 13) % 25) + 65) as char);
        } else if c as u8 <= 122 && c as u8 >= 97 {
            result.push((((c as u8 - 97 + 13) % 25) + 97) as char);
        }
        else {
            result.push(c);
        }
    }

    result
}

fn decode(encoded_string: &String) -> String {
    let mut result = String::new();
    for c in encoded_string.chars() {
        if c as u8 <= 90 && c as u8 >= 65 {
            result.push((((c as u8 - 65 + 25 - 13) % 25) + 65) as char);
        } else if c as u8 <= 122 && c as u8 >= 97 {
            result.push((((c as u8 - 97 + 25 - 13) % 25) + 97) as char);
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let args = std::env::args();
    let mut in_place_arg = false;
    let mut encode_arg = false;
    let mut decode_arg = false;
    let mut filepath: Option<String> = Option::None;
    for (i, arg) in args.enumerate() {
        if i == 0 {
            continue;
        } else if arg == "-i" {
            in_place_arg = true;
        } else if arg == "-e" {
            encode_arg = true;
        } else if arg == "-d" {
            decode_arg = true;
        } else {
            filepath = Some(arg);
        }
    }

    match filepath {
        Some(path) => {
            if encode_arg && decode_arg || (!encode_arg && !decode_arg) {
                panic!("Error: Must choose to either encode or decode.");
            }

            let result_string: String;
            let fd = File::open(&path);
            match fd {
                Ok(mut f) => {
                    let mut file_str: String = String::new();
                    let read_result = f.read_to_string(&mut file_str);
                    match read_result {
                        Ok(_) => {},
                        Err(e) => { panic!("Error reading file: {}", e); }
                    };

                    if encode_arg {
                        result_string = encode(&file_str);
                    } else if decode_arg {
                        result_string = decode(&file_str);
                    } else {
                        result_string = format!("ERROR");
                    }
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            };

            if in_place_arg {
                let fd = File::create(&path);
                match fd {
                    Ok(mut f) => {
                        let write_result = f.write(result_string.as_bytes());
                        match write_result {
                            Ok(_) => {}
                            Err(e) => { panic!("Error writing to file: {}", e); }
                        };
                    },
                    Err(e) => {
                        panic!("Error: {}", e);
                    }
                };
            } else {
                println!("Result:");
                println!("{}", result_string);
            }
        },
        None => {
            let mut answer = String::new();

            if !encode_arg && !decode_arg {
                println!("Encode or decode?");
                io::stdin().read_line(&mut answer).expect("encode");
                answer = answer.strip_suffix("\r\n").or(answer.strip_suffix("\n")).unwrap_or(&answer).to_string();
            }
        
            let result: String;
        
            if answer == "encode" || encode_arg {
                println!("Enter the text to encode");
                let mut toencode = String::new();
                io::stdin().read_line(&mut toencode).expect("fool");
        
                result = encode(&toencode);
            } else {
                println!("Enter the text to decode");
                let mut todecode = String::new();
                io::stdin().read_line(&mut todecode).expect("fool");
        
                result = decode(&todecode);
            }
        
            println!("Result: {}", result);
        }
    }
}
