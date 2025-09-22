pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn empty_query_matches_all_lines() {
        let query = "";
        let contents = "a\nb\nc";
        assert_eq!(vec!["a", "b", "c"], search(query, contents));
    }

    #[test]
    fn empty_contents_returns_empty() {
        let query = "anything";
        let contents = "";
        assert_eq!(Vec::<&str>::new(), search(query, contents));
        assert_eq!(Vec::<&str>::new(), search_case_insensitive(query, contents));
    }

    #[test]
    fn case_insensitive_multiple_lines_mixed_casing() {
        let query = "RUST";
        let contents = "\
Trust in Rust
we love rustaceans
RUST is fun";
        assert_eq!(
            vec!["Trust in Rust", "we love rustaceans", "RUST is fun"],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive_unicode_simple() {
        // Unicode lowercasing should work for typical accents
        let query = "ÉCO";
        let contents = "\
École
ECOSYSTEM
économie";
        assert_eq!(
            vec!["École", "économie"],
            search_case_insensitive(query, contents)
        );
    }
}
