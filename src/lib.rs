pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let p = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&p){
            result.push(line);
        }
    }
    result

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "ruST";
         let contents = "\
Rust:
safe, fast, productive.
Pick three.
trust me ";

         assert_eq!(vec!["Rust:" , "trust me " ], search_case_insensitive(query, contents));
    }
}
