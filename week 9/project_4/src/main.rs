fn main() {
    // Representing the datasets using vectors
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Elieye",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Merging and displaying the data
    println!("{:<4} {:<30} {:<20} {:<15}", "S/N", "Name of Commissioner", "Ministry", "Geopolitical Zone");
    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&");

    for i in 0..ministries.len() {
        println!("{:<4} {:<30} {:<20} {:<15}",
            i + 1, // Serial Number
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }
}