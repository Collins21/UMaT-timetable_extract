use crate::model::Period;

pub fn sort_table(classes: &mut Vec<Period>) {
    fn day_order(day: &str) -> u32 {
        match day {
            "MONDAY" => 1,
            "TUESDAY" => 2,
            "WEDNESDAY" => 3,
            "THURSDAY" => 4,
            "FRIDAY" => 5,
            _ => 6,
        }
    }

    fn big_time(time_str: &str, is_morning: bool) -> f32 {
        let start = time_str.split('-').next().unwrap();
        let parts: Vec<&str> = start.split('.').collect();
        let hours: f32 = parts[0].parse().unwrap();
        let hours = if !is_morning && hours < 12.0 {
            hours + 12.0
        } else {
            hours
        };

        hours
    }

    classes.sort_by(|a, b| {
        let day_a = day_order(&a.day);
        let day_b = day_order(&b.day);
        match day_a.cmp(&day_b) {
            std::cmp::Ordering::Equal => big_time(&a.start_time, a.is_morning)
                .partial_cmp(&big_time(&b.start_time, b.is_morning))
                .unwrap(),
            other => other,
        }
    });
}
