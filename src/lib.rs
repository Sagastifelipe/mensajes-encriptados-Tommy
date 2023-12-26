use std::{io,fs, collections::HashMap};


pub fn encriptar_mensaje(path: String) -> io::Result<()> {
    let conversion_hash = conversion_hash_char_as_int()?;
    let file_content = fs::read_to_string(path.clone())?    ;
    let mut result = String::new();

    for char in file_content.chars() {
        if let Some(converted_char) = conversion_hash.get(&char) {
            result.push(*converted_char);
            if *converted_char != ' ' {
                result.push('-')
            }
        }
    }

    fs::write(path, result.clone())?;
    println!("{}",result);
    Ok(())
}

fn conversion_hash_char_as_int() -> io::Result<HashMap<char,char>> {
    let mut translation_hash:HashMap<char,char> = HashMap::new();
    let spanish_alphabet: [char; 27] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
        'n', 'ñ', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

    for (i, char) in spanish_alphabet.iter().enumerate() {
        let number_to_map = i+1;
        if let Some(number_as_char) = std::char::from_u32(number_to_map as u32) {
            if !translation_hash.insert(*char,number_as_char).is_none() {
                return Err(io::Error::new(io::ErrorKind::Other, "Sapen't"));
            }
        }
    }
    // we add the ' ' char as a '/' becouse thats how my nephew wanted it to be
    translation_hash.insert(' ','/');
    translation_hash.insert('\n', '\n');
    Ok(translation_hash)
}