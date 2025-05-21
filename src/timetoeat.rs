//! # Time To Eat
//! 
//! ## Challenge Description
//! 
//! Sloth is a very habitual person. He eats breakfast at 7:00 a.m. each morning, lunch at 12:00 p.m. and dinner at 7:00 p.m. in the evening.  
//! Create a function that takes in the current time as a string and determines the duration of time before Sloth's next meal.  
//! Represent this as an array with the first and second elements representing hours and minutes, respectively.  

struct Time {
    hour: i32,
    minute: i32,
}

/// Takes a time string in the format of hh:mm am/pm and returns the hours and minutes until the next closest "eating" time.  
/// Eating times are 7:00 a.m., 12:00 p.m., and 7:00 p.m.
/// # Examples
///
/// ```
/// use slothbytes::time_to_eat;
/// 
/// let time = "2:00 p.m.";
/// let expected = [5, 0];
/// assert_eq!(time_to_eat(time), expected);
/// 
/// let time = "5:50 a.m.";
/// let expected = [1, 10];
/// assert_eq!(time_to_eat(time), expected);
/// ```
pub fn time_to_eat(time: &str) -> Vec<i32> {
    let breakfast_time = Time {
        hour: 7,
        minute: 0,
    };
    let lunch_time = Time {
        hour: 12,
        minute: 0,
    };
    let dinner_time = Time {
        hour: 19,
        minute: 0,
    };
    let times = vec![breakfast_time, lunch_time, dinner_time];
    let current_time = parse_time(time);
    let mut time_differences = times.iter().map(|t| time_diff(&current_time, t)).collect::<Vec<Time>>();
    time_differences.sort_by_key(|t| t.hour);

    // for td in time_differences {
    //     let out = format!("hour: {}, minute: {}", td.hour, td.minute);
    //     println!("{out}");
    // }
    // Vec::new()

    vec![time_differences[0].hour, time_differences[0].minute]

}

fn parse_time(time: &str) -> Time {
    let toks = time.split_whitespace().collect::<Vec<&str>>();
    let is_am: bool;
    if toks[1] == "a.m." {
        is_am = true;
    } else {
        is_am = false;
    }
    let hour_minutes = toks[0]
        .split(':')
        .map(|x| x.parse().expect("Expected to be able to parse str to i32."))
        .collect::<Vec<i32>>();
    let hour: i32;
    if is_am {
        if hour_minutes[0] == 12 {
            hour = 0;
        } else {
            hour = hour_minutes[0];
        }
    } else {
        if hour_minutes[0] == 12 {
            hour = 12;
        } else {
            hour = hour_minutes[0] + 12;
        }
    }
    Time {
        hour,
        minute: hour_minutes[1],
    }
}

fn time_diff(current_time: &Time, next_time: &Time) -> Time {
    let sub_one_hour: i32;
    let minute: i32;
    // next: 00 min, current: 59 min, is 1 minute difference
    if next_time.minute < current_time.minute {
        minute = 60 + (next_time.minute - current_time.minute);
        sub_one_hour = 1; // used to carry forward into the hour calculation
    // next: 59 min, current: 00 min, is 59 minutes difference
    } else {
        minute = next_time.minute - current_time.minute;
        sub_one_hour = 0;
    }
    let hour: i32;
    // next: 1 hour (1am), current: 2 hour (2am), is 23 hours difference
    // BUT next: 1 hour, 00 min (1:00am), current 2 hour, 59 min (2:59am), is 22 hours, 59 minutes difference 
    if next_time.hour < current_time.hour {
        hour = 24 + (next_time.hour - current_time.hour) - sub_one_hour
    // next: 7 hour, 00 min (7:00am), current 7 hour, 01 min (7:01am), is 23 hours, 59 minutes difference
    // next: 7 hour, 01 min (7:01am), current 7 hour, 00 min (7:00am), is 0 hours, 1 minute difference
    } else if next_time.hour == current_time.hour {
        hour = 0 + (23 * sub_one_hour)
    } else {
        hour = next_time.hour - current_time.hour - sub_one_hour
    }
    Time {
        hour,
        minute,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_pm() {
        let time = "2:00 p.m.";
        let expected = [5, 0];
        assert_eq!(time_to_eat(time), expected);
    }

    #[test]
    fn five_fifty_am() {
        let time = "5:50 a.m.";
        let expected = [1, 10];
        assert_eq!(time_to_eat(time), expected);
    }

    #[test]
    fn seven_oh_one_pm() {
        let time = "7:01 p.m.";
        let expected = [11, 59];
        assert_eq!(time_to_eat(time), expected);
    }
}