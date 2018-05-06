extern crate time;

fn print_time_horizontally() {
    let time = time::now();
    let hour: usize = time.tm_hour as usize;
    let min: usize = time.tm_min as usize;
    let mut hour_tens_digit: usize = 0;
    let hour_singles_digit: usize;
    let min_tens_digit: usize = min / 10;
    let min_singles_digit: usize = min % 10;

    if hour == 0 || hour == 12 {
        hour_tens_digit = 1;
        hour_singles_digit = 2;
    }

    else if hour > 12 {
        hour_tens_digit = (hour - 12) / 10;
        hour_singles_digit = (hour - 12) % 10;
    }

    else {
        hour_singles_digit = hour;
    }

    let number_zero = ["00000000",
                       "00000000",
                       "000  000",
                       "000  000",
                       "000  000",
                       "000  000",
                       "000  000",
                       "000  000",
                       "00000000",
                       "00000000"];

    let number_one =  ["111111  ",
                       "111111  ",
                       "   111  ",
                       "   111  ",
                       "   111  ",
                       "   111  ",
                       "   111  ",
                       "   111  ",
                       "11111111",
                       "11111111"];

    let number_two =  ["22222222",
                       "22222222",
                       "     222",
                       "     222",
                       "22222222",
                       "22222222",
                       "222     ",
                       "222     ",
                       "22222222",
                       "22222222"];

    let number_three = ["33333333",
                        "33333333",
                        "     333",
                        "     333",
                        "33333333",
                        "33333333",
                        "     333",
                        "     333",
                        "33333333",
                        "33333333"];

    let number_four =  ["444  444",
                        "444  444",
                        "444  444",
                        "444  444",
                        "44444444",
                        "44444444",
                        "     444",
                        "     444",
                        "     444",
                        "     444"];

    let number_five =  ["55555555",
                        "55555555",
                        "555     ",
                        "555     ",
                        "55555555",
                        "55555555",
                        "     555",
                        "     555",
                        "55555555",
                        "55555555"];

    let number_six =   ["66666666",
                        "66666666",
                        "666     ",
                        "666     ",
                        "66666666",
                        "66666666",
                        "666  666",
                        "666  666",
                        "55555555",
                        "55555555"];

    let number_seven = ["77777777",
                        "77777777",
                        "     777",
                        "     777",
                        "     777",
                        "     777",
                        "     777",
                        "     777",
                        "     777",
                        "     777"];

    let number_eight = ["88888888",
                        "88888888",
                        "888  888",
                        "888  888",
                        "88888888",
                        "88888888",
                        "888  888",
                        "888  888",
                        "88888888",
                        "88888888"];

    let number_nine =  ["99999999",
                        "99999999",
                        "999  999",
                        "999  999",
                        "99999999",
                        "99999999",
                        "     999",
                        "     999",
                        "     999",
                        "     999"];

    let number_colon = ["        ",
                        "        ",
                        "   ;;   ",
                        "   ;;   ",
                        "        ",
                        "        ",
                        "   ;;   ",
                        "   ;;   ",
                        "        ",
                        "        "];

    let letter_a =     ["aaaaaaaa",
                        "aaaaaaaa",
                        "aaa  aaa",
                        "aaa  aaa",
                        "aaaaaaaa",
                        "aaaaaaaa",
                        "aaa  aaa",
                        "aaa  aaa",
                        "aaa  aaa",
                        "aaa  aaa"];

    let letter_p =     ["pppppppp",
                        "pppppppp",
                        "ppp  ppp",
                        "ppp  ppp",
                        "pppppppp",
                        "pppppppp",
                        "ppp     ",
                        "ppp     ",
                        "ppp     ",
                        "ppp     "];

    let letter_m =     ["mmmmmmmm",
                        "mmmmmmmm",
                        "mm mm mm",
                        "mm mm mm",
                        "mm mm mm",
                        "mm mm mm",
                        "mm mm mm",
                        "mm mm mm",
                        "mm mm mm",
                        "mm mm mm"];

    let clock_numbers = [number_zero,
                         number_one,
                         number_two,
                         number_three,
                         number_four,
                         number_five,
                         number_six,
                         number_seven,
                         number_eight,
                         number_nine];

    for row in 0..number_zero.len() {
        print! ("{}", clock_numbers[hour_tens_digit][row]);
        print! (" ");
        print! ("{}", clock_numbers[hour_singles_digit][row]);
        print! ("{}", number_colon[row]);
        print! ("{}", clock_numbers[min_tens_digit][row]);
        print! (" ");
        print! ("{}", clock_numbers[min_singles_digit][row]);
        print! ("   ");

        if hour > 12 {
            print! ("{}", letter_p[row]);
        }

        else {
            print! ("{}", letter_a[row]);
        }

        print! (" ");
        print! ("{}", letter_m[row]);
        println!();
    }
}

fn main() {
    print_time_horizontally();
}
