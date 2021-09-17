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
    let day_name: &str = get_day_name(local_datetime.weekday() as u8);
    let month_name: &str = get_month_name(local_datetime.month0() as u8);
    let day_number: u32 = local_datetime.day();
    let year: i32 = local_datetime.year();
    format!("{}: {} {}, {}", day_name, month_name, day_number, year)
}

fn get_am_or_pm(local_datetime: &DateTime<Local>) -> String {
    let is_pm: bool = local_datetime.hour12().0;
    let (a, b): (&str, &str) = if is_pm { (" ", "*") } else { ("*", " ") };
    format!("{}AM {}PM", a, b)
}

fn get_day_name(day_number: u8) -> &'static str {
    match day_number {
        0 => "Monday",
        1 => "Tuesday",
        2 => "Wednesday",
        3 => "Thursday",
        4 => "Friday",
        5 => "Sunday",
        _ => "Saturday",
    }
}

fn get_month_name(month_number: u8) -> &'static str {
    match month_number {
        0 => "January",
        1 => "February",
        2 => "March",
        3 => "April",
        4 => "May",
        5 => "June",
        6 => "July",
        7 => "August",
        8 => "September",
        9 => "October",
        10 => "November",
        _ => "December",
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
