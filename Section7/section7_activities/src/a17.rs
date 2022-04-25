// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

pub fn standard_library() {

    let mut string: String = "Kenny".to_owned();
    
    println!("uppercase: {:?}", &string.to_uppercase());
    println!("lowercase: {:?}", &string.to_lowercase());
    
}
