//use std;
extern crate time;

fn print_time_horizontally (hour: i32, min: i32)
{
    let mut tens_digit_hour = 0 as usize;
    let mut singles_digit_hour = 0  as usize;
    let mut tens_digit_min = 0 as usize;
    let mut singles_digit_min = 0 as usize;

    if hour > 12 {
        tens_digit_hour = ((hour - 12) / 10) as usize;
        singles_digit_hour = ((hour - 12) % 10) as usize;
    }

    else {
        singles_digit_hour = hour  as usize;
    }

    if min > 10 {
        tens_digit_min = (min / 10)  as usize;
        singles_digit_min = (min % 10)  as usize;
    }

    else {
        tens_digit_min = 0 as usize;
        singles_digit_min = min as usize;
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

    let number_one =  ["     111",
                       "     111",
                       "     111",
                       "     111",
                       "     111",
                       "     111",
                       "     111",
                       "     111",
                       "     111",
                       "     111"];

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

    for x in 0..number_zero.len() {
        print! ("{}", clock_numbers[tens_digit_hour][x]);
        print! (" ");
        print! ("{}", clock_numbers[singles_digit_hour][x]);
        print! ("{}", number_colon[x]);
        print! ("{}", clock_numbers[tens_digit_min][x]);
        print! (" ");
        print! ("{}", clock_numbers[singles_digit_min][x]);
        print! ("   ");

        if hour > 12 {
            print! ("{}", letter_p[x]);
        }

        else {
            print! ("{}", letter_a[x]);
        }

        print! (" ");
        print! ("{}", letter_m[x]);
        println!();
    }
}

fn main() {
    let time = time::now();
    print_time_horizontally (time.tm_hour, time.tm_min);
}
