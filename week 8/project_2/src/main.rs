fn find_highest_experience(candidates: Vec<(&str, u32)>) -> Option<(&str, u32)> {
    // Find the candidate with the maximum years of experience
    candidates.into_iter().max_by_key(|candidate| candidate.1)
}

fn main() {
    // List of candidates as tuples (name, years of experience)
    let candidates = vec![
        ("Judith", 5),
        ("Michael", 8),
        ("Josephine", 12),
        ("Amarachi", 10),
    ];

    // Find the candidate with the highest experience
    match find_highest_experience(candidates) {
        Some((name, years_of_experience)) => {
            println!("The candidate with the highest experience is {} with {} years of experience.", name, years_of_experience);
        },
        None => println!("No candidates found."),
    }
}