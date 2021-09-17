use chrono::{DateTime, Datelike, Local, Timelike};
mod clock_characters;

// TODO:
// Change to be unit-testable

pub fn print_clock() {
    let local_datetime: DateTime<Local> = chrono::offset::Local::now();
    let border_length: usize = 46;
    let date: String = get_date(&local_datetime);
    let am_or_pm: String = get_am_or_pm(&local_datetime);
    let space_filler: usize = border_length - date.len() - am_or_pm.len() - 2;

    println!(" {} ", "-".repeat(border_length));
    println!("| {}{}{} |", date, " ".repeat(space_filler), am_or_pm);
    println!("| {} |", "~".repeat(border_length - 2));
    print_time_horizontally(&local_datetime);
    println!(" {} ", "-".repeat(border_length));
}

fn get_date(local_datetime: &DateTime<Local>) -> String {
    let weekday: String = get_day(local_datetime.weekday() as u8);
    let month: String = get_month(local_datetime.month0());
    let day_number: u32 = local_datetime.day();
    let year: i32 = local_datetime.year();
    format!("{}: {} {}, {}", weekday, month, day_number, year)
}

fn get_am_or_pm(local_datetime: &DateTime<Local>) -> String {
    let is_pm: bool = local_datetime.hour12().0;
    let (a, b): (&str, &str) = if is_pm { (" ", "*") } else { ("*", " ") };
    format!("{}AM {}PM", a, b)
}

fn get_day(day_number: u8) -> String {
    match day_number {
        0 => "Monday".to_string(),
        1 => "Tuesday".to_string(),
        2 => "Wednesday".to_string(),
        3 => "Thursday".to_string(),
        4 => "Friday".to_string(),
        5 => "Sunday".to_string(),
        _ => "Saturday".to_string(),
    }
}

fn get_month(month_number: u32) -> String {
    match month_number {
        0 => "January".to_string(),
        1 => "February".to_string(),
        2 => "March".to_string(),
        3 => "April".to_string(),
        4 => "May".to_string(),
        5 => "June".to_string(),
        6 => "July".to_string(),
        7 => "August".to_string(),
        8 => "September".to_string(),
        9 => "October".to_string(),
        10 => "November".to_string(),
        _ => "December".to_string(),
    }
}

fn print_time_horizontally(local_datetime: &DateTime<Local>) {
    let hour: usize = local_datetime.hour12().1 as usize;
    let hour_tens_digit: usize = hour / 10;
    let hour_singles_digit: usize = hour % 10;

    let minute: usize = local_datetime.minute() as usize;
    let minute_tens_digit: usize = minute / 10;
    let minute_singles_digit: usize = minute % 10;

    for (i, _) in clock_characters::COLON.iter().enumerate() {
        println!(
            "| {} {} {} {} {} |",
            clock_characters::CLOCK_NUMBERS[hour_tens_digit][i],
            clock_characters::CLOCK_NUMBERS[hour_singles_digit][i],
            clock_characters::COLON[i],
            clock_characters::CLOCK_NUMBERS[minute_tens_digit][i],
            clock_characters::CLOCK_NUMBERS[minute_singles_digit][i]
        );
    }
}
