use std::time::Duration;

pub fn get_formatted_time(duration: Duration) -> String {
    let mut secs = duration.as_secs();
    let mins = secs / 60;
    secs %= 60;
    let millis = duration.subsec_millis();
    
    format!("{}:{:0>2}.{:0>3}", mins, secs, millis)
}

pub fn get_formatted_pps(pieces: usize, duration: Duration) -> String {
    let pps = pieces as f32 / duration.as_secs_f32();
    format!("{:.2}/s", pps)
}

pub fn get_formatted_apm(attack: usize, duration: Duration) -> String {
    let apm = 60. * attack as f32 / duration.as_secs_f32();
    format!("{:.2}/min", apm)
}

pub fn get_formatted_score(score: usize) -> String {
    let mut s = score;
    let mega = s / 1_000_000;
    s %= 1_000_000;
    let kilo = s / 1_000;
    s %= 1_000;

    if mega == 0 {
        if kilo == 0 {
            return s.to_string();
        }
        return format!("{},{:0>3}", kilo, s);
    }
    return format!("{},{:0>3},{:0>3}", mega, kilo, s);
}