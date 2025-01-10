use std::fs::File;
use std::io::Write;

fn main() {

    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let geopolitical_zones = vec!["South West", "North East", "South South", "South West", "South East"];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let mut file = File::create("merged_data.txt").expect("Failed to create file");

    writeln!(file, "S/N, Name of Commissioner, Geopolitical Zone, Ministry").expect("Failed to write to file");
    for (i, ((commissioner, zone), ministry)) in commissioners.iter()
        .zip(geopolitical_zones.iter())
        .zip(ministries.iter())
        .enumerate()
    {
        writeln!(file, "{}, {}, {}, {}", i + 1, commissioner, zone, ministry).expect("Failed to write to file");
    }

    println!("Merged data has been saved to 'merged_data.txt'.");
}

