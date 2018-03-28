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
    println! ("         ");
    println! ("         ");
    println! ("    ;;   ");
    println! ("    ;;   ");
    println! ("         ");
    println! ("         ");
    println! ("    ;;   ");
    println! ("    ;;   ");
    println! ("         ");
    println! ("         ");
}

fn print_time() {
    let time = time::now();
    let hour = time.tm_hour;
    let min = time.tm_min;

    if hour > 12 {
        print! ("{}:{}", hour - 12, min);
    }

    print! ("{}:{}", hour, min);

    if hour < 12 {
        println! (" AM");
    }

    else {
        print! (" PM");
    }
}

fn main() {
    print_time();
    print_zero();
}
