pub fn print_clock() {
    let time = time::now();
    print_horizontal_border();
    print!("| ");
    print_date(time);
    print_am_or_pm(time);
    println!(" |");
    print!("| ");
    print_divider();
    println!(" |");
    print_time_horizontally(time);
    print_horizontal_border();
}

fn print_horizontal_border() {
    println!(" ----------------------------------------------");
}

fn print_date(time: time::Tm) {
    let weekday = get_day(time.tm_wday);
    let month = get_month(time.tm_mon);
    let day_number = time.tm_mday;
    let year = 1900 + time.tm_year;

    print!("{}: {} {}, {}", weekday, month, day_number, year);

    // The following code prints out the remaining spaces needed to keep the AM/PM code within
    // the clock border
    let mut spaces_to_print: usize = 32;

    spaces_to_print -= weekday.to_string().chars().count()
        + month.to_string().chars().count()
        + day_number.to_string().chars().count()
        + year.to_string().chars().count();

    print_space(spaces_to_print);
}

fn get_day(day_number: i32) -> String {
    match day_number {
        0 => "Sunday".to_string(),
        1 => "Monday".to_string(),
        2 => "Tuesday".to_string(),
        3 => "Wednesday".to_string(),
        4 => "Thursday".to_string(),
        5 => "Friday".to_string(),
        _ => "Saturday".to_string(),
    }
}

fn get_month(month_number: i32) -> String {
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

fn print_space(spaces_to_print: usize) {
    for _ in 0..spaces_to_print {
        print!(" ");
    }
}

fn print_am_or_pm(time: time::Tm) {
    if time.tm_hour > 12 {
        print!(" AM *PM");
    } else {
        print!("*AM  PM");
    }
}

fn print_divider() {
    print!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~")
}

fn print_time_horizontally(time: time::Tm) {
    let hour: usize = time.tm_hour as usize;
    let min: usize = time.tm_min as usize;
    let mut hour_tens_digit: usize = 0;
    let hour_singles_digit: usize;
    let min_tens_digit: usize = min / 10;
    let min_singles_digit: usize = min % 10;

    if hour == 0 || hour == 12 {
        hour_tens_digit = 1;
        hour_singles_digit = 2;
    } else if hour > 12 {
        hour_tens_digit = (hour - 12) / 10;
        hour_singles_digit = (hour - 12) % 10;
    } else {
        hour_singles_digit = hour;
    }

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

    for (i, _item) in number_zero.iter().enumerate() {
        print!("|");
        print!(" ");
        print!("{}", clock_numbers[hour_tens_digit][i]);
        print!(" ");
        print!("{}", clock_numbers[hour_singles_digit][i]);
        print!(" ");
        print!("{}", number_colon[i]);
        print!(" ");
        print!("{}", clock_numbers[min_tens_digit][i]);
        print!(" ");
        print!("{}", clock_numbers[min_singles_digit][i]);
        print!(" ");
        print!("|");
        println!();
    }
}
