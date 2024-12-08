use std::io;

fn main() {
    // Define staff categories and APS levels using a vector of tuples
    let aps_levels = vec![
        ("Office Administrator", "Intern", (1, 2)),
        ("Office Administrator", "Administrator", (3, 5)),
        ("Office Administrator", "Senior Administrator", (5, 8)),
        ("Office Administrator", "Office Manager", (8, 10)),
        ("Office Administrator", "Director", (10, 13)),
        ("Office Administrator", "CEO", (13, 15)),
        ("Academic", "Research Assistant", (3, 5)),
        ("Academic", "PhD Candidate", (5, 8)),
        ("Academic", "Post-Doc Researcher", (8, 10)),
        ("Academic", "Senior Lecturer", (10, 13)),
        ("Academic", "Dean", (13, 15)),
        ("Lawyer", "Paralegal", (1, 2)),
        ("Lawyer", "Junior Associate", (3, 5)),
        ("Lawyer", "Associate", (5, 8)),
        ("Lawyer", "Senior Associate 1-2", (8, 10)),
        ("Lawyer", "Senior Associate 3-4", (10, 13)),
        ("Lawyer", "Partner", (13, 15)),
        ("Teacher", "Placement", (1, 2)),
        ("Teacher", "Classroom Teacher", (3, 5)),
        ("Teacher", "Snr Teacher", (5, 8)),
        ("Teacher", "Leading Teacher", (8, 10)),
        ("Teacher", "Deputy Principal", (10, 13)),
        ("Teacher", "Principal", (13, 15)),
    ];

    
    let mut role = String::new();
    let mut position = String::new();
    let mut experience = String::new();

    println!("Enter your role (e.g., Lawyer, Academic,Teacher,Office Administrator): ");
    io::stdin().read_line(&mut role).unwrap();
    let role = role.trim();

    println!("Enter your position (e.g., Associate, CEO): ");
    io::stdin().read_line(&mut position).unwrap();
    let position = position.trim();

    println!("Enter your years of experience: ");
    io::stdin().read_line(&mut experience).unwrap();
    let experience: i32 = experience.trim().parse().unwrap_or(0);

    //To validate role, position, and experience
    let mut found = false;
    for (r, p, years) in &aps_levels {
        if role == *r && position == *p && (experience >= years.0 && experience <= years.1) {
            println!("Your APS level is: {}-{}", years.0, years.1);
            found = true;
            break;
        }
    }

    if !found {
        println!("No matching APS level found for the given details.");
    }
}
