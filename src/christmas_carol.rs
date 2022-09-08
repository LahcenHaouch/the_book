const CHRISTMAS_DAYS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];
pub fn twelve_days_of_christmas() {
    for day in 1..=12 {
        let ord = ordinal_number(day);
        println!("On the {ord} day of Christmas, my true love sent to me");
        for gift_day in 0..day {
            let christmas_gift = CHRISTMAS_DAYS.get(gift_day as usize).unwrap();
            if gift_day == 0 || gift_day == 1 || gift_day == day - 2 {
                println!("{}", format!("{christmas_gift}, and"));
            } else {
                println!("{christmas_gift}");
            }
        }
        println!("###################");
    }
}

fn ordinal_number(number: u8) -> String {
    let j = number % 10;
    let k = number % 100;

    if j == 1 && k != 11 {
        return number.to_string() + "st";
    }
    if j == 2 && k != 12 {
        return number.to_string() + "nd";
    }
    if j == 3 && k != 13 {
        return number.to_string() + "rd";
    }
    number.to_string() + "th"
}
