use chrono::{DateTime, Datelike, Local, Timelike};

pub fn print_clock() {
    let local_datetime = chrono::offset::Local::now();

    print_horizontal_border();
    print!("| ");
    print_date(&local_datetime);
    print_am_or_pm(&local_datetime);
    println!(" |");
    print!("| ");
    print_divider();
    println!(" |");
    print_time_horizontally(&local_datetime);
    print_horizontal_border();
}

fn print_horizontal_border() {
    println!(" {}", "-".repeat(46));
}

fn print_date(local_datetime: &DateTime<Local>) {
    let weekday = get_day(local_datetime.weekday() as u8);
    let month = get_month(local_datetime.month0());
    let day_number = local_datetime.day();
    let year = local_datetime.year();

    print!("{}: {} {}, {}", weekday, month, day_number, year);

    // The following code prints out the remaining spaces needed to keep the AM/PM code within
    // the clock border
    let spaces_to_print = 32 - weekday.chars().count()
        + month.chars().count()
        + day_number.to_string().chars().count()
        + year.to_string().chars().count();

    print!("{}", " ".repeat(spaces_to_print));
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

fn print_am_or_pm(local_datetime: &DateTime<Local>) {
    let is_pm = local_datetime.hour12().0;

    match is_pm {
        true => print!(" AM *PM"),
        false => print!("*AM  PM"),
    }
}

fn print_divider() {
    print!("{}", "~".repeat(44));
}

fn print_time_horizontally(local_datetime: &DateTime<Local>) {
    #[rustfmt::skip]
    let number_zero = [
        "00000000",
        "00000000",
        "000  000",
        "000  000",
        "000  000",
        "000  000",
        "000  000",
        "000  000",
        "00000000",
        "00000000",
    ];

    #[rustfmt::skip]
    let number_one = [
        "111111  ",
        "111111  ",
        "   111  ",
        "   111  ",
        "   111  ",
        "   111  ",
        "   111  ",
        "   111  ",
        "11111111",
        "11111111",
    ];

    #[rustfmt::skip]
    let number_two = [
        "22222222",
        "22222222",
        "     222",
        "     222",
        "22222222",
        "22222222",
        "222     ",
        "222     ",
        "22222222",
        "22222222",
    ];

    #[rustfmt::skip]
    let number_three = [
        "33333333",
        "33333333",
        "     333",
        "     333",
        "33333333",
        "33333333",
        "     333",
        "     333",
        "33333333",
        "33333333",
    ];

    #[rustfmt::skip]
    let number_four = [
        "444  444",
        "444  444",
        "444  444",
        "444  444",
        "44444444",
        "44444444",
        "     444",
        "     444",
        "     444",
        "     444",
    ];

    #[rustfmt::skip]
    let number_five = [
        "55555555",
        "55555555",
        "555     ",
        "555     ",
        "55555555",
        "55555555",
        "     555",
        "     555",
        "55555555",
        "55555555",
    ];

    #[rustfmt::skip]
    let number_six = [
        "66666666",
        "66666666",
        "666     ",
        "666     ",
        "66666666",
        "66666666",
        "666  666",
        "666  666",
        "55555555",
        "55555555",
    ];

    #[rustfmt::skip]
    let number_seven = [
        "77777777",
        "77777777",
        "     777",
        "     777",
        "     777",
        "     777",
        "     777",
        "     777",
        "     777",
        "     777",
    ];

    #[rustfmt::skip]
    let number_eight = [
        "88888888",
        "88888888",
        "888  888",
        "888  888",
        "88888888",
        "88888888",
        "888  888",
        "888  888",
        "88888888",
        "88888888",
    ];

    #[rustfmt::skip]
    let number_nine = [
        "99999999",
        "99999999",
        "999  999",
        "999  999",
        "99999999",
        "99999999",
        "     999",
        "     999",
        "     999",
        "     999",
    ];

    #[rustfmt::skip]
    let number_colon = [
        "        ",
        "        ",
        "   ::   ",
        "   ::   ",
        "        ",
        "        ",
        "   ::   ",
        "   ::   ",
        "        ",
        "        "
    ];

    let clock_numbers = [
        number_zero,
        number_one,
        number_two,
        number_three,
        number_four,
        number_five,
        number_six,
        number_seven,
        number_eight,
        number_nine,
    ];

    let hour: usize = local_datetime.hour12().1 as usize;
    let hour_tens_digit: usize = hour / 10;
    let hour_singles_digit: usize = hour % 10;

    let minute: usize = local_datetime.minute() as usize;
    let minute_tens_digit: usize = minute / 10;
    let minute_singles_digit: usize = minute % 10;

    for (i, _) in number_zero.iter().enumerate() {
        println!(
            "| {} {} {} {} {} |",
            clock_numbers[hour_tens_digit][i],
            clock_numbers[hour_singles_digit][i],
            number_colon[i],
            clock_numbers[minute_tens_digit][i],
            clock_numbers[minute_singles_digit][i]
        );
    }
}

// TODO:
// Change to be unit-testable
// Move these to their own file (a structure of some sort?)
