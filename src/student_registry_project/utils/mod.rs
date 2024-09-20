use std::time::{SystemTime, UNIX_EPOCH};

pub fn convert_to_string(x: &str) -> String {
    x.to_string()
}

pub fn concat_time(h: String, m: u64, s: u64) -> String {
    format!("{}:{:02}:{:02}", h, m, s)
}

pub fn get_current_time() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards"); // Due to friction that occurs during the earth's orbit around the sun
    let total_seconds = now.as_secs();
    let seconds_in_day = total_seconds % 86400; // Seconds in a day
    let hour = seconds_in_day / 3600; // Seconds in an hour
    let minute = (seconds_in_day % 3600) / 60; // Seconds in a minute
    let second = seconds_in_day % 60;
    concat_time(format!("{:02}", hour), minute, second)
}

pub fn get_current_date() -> String {
    let datetime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let year = 1970 + (datetime.as_secs() / 31536000); // Number of seconds in a year
    let month = (datetime.as_secs() % 31536000) / 2629743; // Number of seconds in a month
    let day = (datetime.as_secs() % 2629743) / 86400; // Number of seconds in a day
    concat_time(year.to_string(), month, day)
}
