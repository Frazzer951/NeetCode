use neet_code::nc_150::array_and_hashing::group_anagrams::group_anagrams;

fn main() {
    let result = group_anagrams(vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ]);
    println!("{:#?}", result);
}
