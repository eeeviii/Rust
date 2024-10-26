fn time_conversion(s: &str) -> String {
    let (time, period) = s.split_at(s.len() - 2);
    let mut parts: Vec<&str> = time.split(':').collect();
    let mut hours: u32 = parts[0].parse().unwrap();

    if period == "PM" && hours < 12 {
        hours += 12;
    }
    if period == "AM" && hours == 12 {
        hours = 0;
    }

    let formatted_hours = format!("{:02}", hours);
    parts[0] = &formatted_hours;
    format!("{}:{}:{}", parts[0], parts[1], parts[2])
}

fn main() {
    let input = "07:05:45 PM";
    let output = time_conversion(input);
    println!("{}", output);
}
