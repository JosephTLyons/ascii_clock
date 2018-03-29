extern crate time;

fn print_zero() {
    println! ("00000000");
    println! ("00000000");
    println! ("000  000");
    println! ("000  000");
    println! ("000  000");
    println! ("000  000");
    println! ("000  000");
    println! ("000  000");
    println! ("00000000");
    println! ("00000000");
}

fn print_one() {
    println! ("     111");
    println! ("     111");
    println! ("     111");
    println! ("     111");
    println! ("     111");
    println! ("     111");
    println! ("     111");
    println! ("     111");
    println! ("     111");
    println! ("     111");
}

fn print_two() {
    println! ("22222222");
    println! ("22222222");
    println! ("     222");
    println! ("     222");
    println! ("22222222");
    println! ("22222222");
    println! ("222     ");
    println! ("222     ");
    println! ("22222222");
    println! ("22222222");
}

fn print_three() {
    println! ("33333333");
    println! ("33333333");
    println! ("     333");
    println! ("     333");
    println! ("33333333");
    println! ("33333333");
    println! ("     333");
    println! ("     333");
    println! ("33333333");
    println! ("33333333");
}

fn print_four() {
    println! ("444  444");
    println! ("444  444");
    println! ("444  444");
    println! ("444  444");
    println! ("44444444");
    println! ("44444444");
    println! ("     444");
    println! ("     444");
    println! ("     444");
    println! ("     444");
}

fn print_five() {
    println! ("55555555");
    println! ("55555555");
    println! ("555     ");
    println! ("555     ");
    println! ("55555555");
    println! ("55555555");
    println! ("     555");
    println! ("     555");
    println! ("55555555");
    println! ("55555555");
}

fn print_six() {
    println! ("66666666");
    println! ("66666666");
    println! ("666     ");
    println! ("666     ");
    println! ("66666666");
    println! ("66666666");
    println! ("666  666");
    println! ("666  666");
    println! ("55555555");
    println! ("55555555");
}

fn print_seven() {
    println! ("77777777");
    println! ("77777777");
    println! ("     777");
    println! ("     777");
    println! ("     777");
    println! ("     777");
    println! ("     777");
    println! ("     777");
    println! ("     777");
    println! ("     777");
}

fn print_eight() {
    println! ("88888888");
    println! ("88888888");
    println! ("888  888");
    println! ("888  888");
    println! ("88888888");
    println! ("88888888");
    println! ("888  888");
    println! ("888  888");
    println! ("88888888");
    println! ("88888888");
}

fn print_nine() {
    println! ("99999999");
    println! ("99999999");
    println! ("999  999");
    println! ("999  999");
    println! ("99999999");
    println! ("99999999");
    println! ("     999");
    println! ("     999");
    println! ("     999");
    println! ("     999");
}

fn print_space() {
    println! (" ");
    println! (" ");
    println! (" ");
    println! (" ");
    println! (" ");
    println! (" ");
    println! (" ");
    println! (" ");
    println! (" ");
    println! (" ");
}

fn print_colon() {
    println! ("        ");
    println! ("        ");
    println! ("   ;;   ");
    println! ("   ;;   ");
    println! ("        ");
    println! ("        ");
    println! ("   ;;   ");
    println! ("   ;;   ");
    println! ("        ");
    println! ("        ");
}

fn print_a() {
    println! ("aaaaaaaa");
    println! ("aaaaaaaa");
    println! ("aaa  aaa");
    println! ("aaa  aaa");
    println! ("aaa  aaa");
    println! ("aaaaaaaa");
    println! ("aaaaaaaa");
    println! ("aaa  aaa");
    println! ("aaa  aaa");
    println! ("aaa  aaa");
}

fn print_p() {
    println! ("pppppppp");
    println! ("pppppppp");
    println! ("ppp  ppp");
    println! ("ppp  ppp");
    println! ("ppp  ppp");
    println! ("pppppppp");
    println! ("pppppppp");
    println! ("ppp     ");
    println! ("ppp     ");
    println! ("ppp     ");
}

fn print_m() {
    println! ("mmmmmmmm");
    println! ("mmmmmmmm");
    println! ("mm mm mm");
    println! ("mm mm mm");
    println! ("mm mm mm");
    println! ("mm mm mm");
    println! ("mm mm mm");
    println! ("mm mm mm");
    println! ("mm mm mm");
    println! ("mm mm mm");
}

fn print_number (number: i32) {
    if number == 0 {
        print_zero();
    }

    else if number == 1 {
        print_one();
    }

    else if number == 2 {
        print_two();
    }

    else if number == 3 {
        print_three();
    }

    else if number == 4 {
        print_four();
    }

    else if number == 5 {
        print_five();
    }

    else if number == 6 {
        print_six();
    }

    else if number == 7 {
        print_seven();
    }

    else if number == 8 {
        print_eight();
    }

    else if number == 9 {
        print_nine();
    }
}

fn print_time_vertical (hour: i32, min: i32) {
    let mut tens_digit_hour = 0;
    let mut singles_digit_hour = 0;
    let mut tens_digit_min = 0;
    let mut singles_digit_min = 0;

    if hour > 12 {
        tens_digit_hour = 0;
        singles_digit_hour = hour - 12;
    }

    else {
        singles_digit_hour = hour;
    }

    if min > 10 {
        tens_digit_min = min / 10;
        singles_digit_min = min % 10;
    }

    else {
        tens_digit_min = 0;
        singles_digit_min = min;
    }

    print_number (tens_digit_hour);
    print_number (singles_digit_hour);
    print_colon();
    print_number (tens_digit_min);
    print_number (singles_digit_min);

    if hour > 12 {
        print_p();
    }

    else {
        print_a();
    }

    print_m();
}

fn main() {
    let time = time::now();

    print_time_vertical (time.tm_hour, time.tm_min);
}
