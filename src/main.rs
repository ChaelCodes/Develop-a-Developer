use std::io;
use std::fs;

fn get_file_questions() -> Vec<String> {
    let contents = fs::read_to_string("src/questions.txt").unwrap();
    let contents = contents.trim();
    let split: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    split
}

fn main() {
    let questions = get_file_questions();
    let mut journey: Vec<(&String, bool)> = Vec::new();
    
    for question in questions.iter() {
        println!("{}", question);
        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        let answer = response(&answer);
        journey.push((question, answer));
    };

    println!("{:?}", journey);
}

fn response(input: &str) -> bool {
    let cleaned_input = input.trim();
    match cleaned_input { 
        "yes" | "Yes" | "Y" | "y" | "YES" => true,
        "no" | "No" | "N" | "n" | "NO" => false, 
        _ => {
            println!("I'll just put a no here...");
            false
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_is_true() {
        assert_eq!(response("yes\n"), true);
        assert_eq!(response("y\n"), true);
        assert_eq!(response("YES\n"), true);
        assert_eq!(response("Y\n"), true);
    }

    #[test]
    fn response_is_false() {
        assert_eq!(response("no\n"), false);
        assert_eq!(response("n\n"), false);
        assert_eq!(response("NO\n"), false);
    }

    #[test]
    fn response_is_invalid() {
        assert_eq!(response("42\n"), false);
        assert_eq!(response("word\n"), false);
        assert_eq!(response("noperoni\n"), false);
    }
}
