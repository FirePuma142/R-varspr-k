fn main() {
    translate("lol");
}
fn translate(input: &str) -> &str{
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
    println!("{}", translation);
    input
}
fn de_translate(input: &str) ->&str{
    input
}
