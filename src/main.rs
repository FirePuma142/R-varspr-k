use text_io::read;

fn main() {
    println!("Enter text to translate:");
    let input: String = read!();
    let translation = translate(&input);
    let de_translation = de_translate(&input);
    println!("Original string: '{}'\nTranslation: '{}'\nDetranslation: '{}'",input, translation, de_translation);
}
fn translate(input: &str) -> String{
    let consonants = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ";
    let mut translation = String::new(); 
    for letter in input.chars(){
        if consonants.contains(letter){
            translation.push(letter);
            translation.push_str("o");
            translation.push(letter);
        }
        else{
            translation.push(letter);
        }
    }
    translation
}
fn de_translate(input: &str) ->String{
    let consonants = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ";
    let mut translation = String::new();
    let mut i = 0;
    let input_chars: Vec<char> = input.chars().collect();

    while i < input_chars.len(){
        let letter: char = input_chars[i];
        
        if consonants.contains(letter) && i + 2 <   input_chars.len() && input_chars[i+1] == 'o' && input_chars[i+2] == letter{
            translation.push(letter);
            i += 3;
        }
        else{
            translation.push(letter);
            i += 1;
        }
    }
    translation
}
