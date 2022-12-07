mod dayfive;
mod dayfour;
mod dayone;
mod dayseven;
mod daysix;
mod daythree;
mod daytwo;

mod filereader;

fn main() {
    dbg!(dayone::day_1(1));
    dbg!(dayone::day_1(3));

    dbg!(daytwo::day_2_1());
    dbg!(daytwo::day_2_2());

    dbg!(daythree::day_3_1());
    dbg!(daythree::day_3_2());

    dbg!(dayfour::day_4_1());
    dbg!(dayfour::day_4_2());

    dbg!(dayfive::day_5_1());
    dbg!(dayfive::day_5_2());

    dbg!(daysix::day_6(4));
    dbg!(daysix::day_6(14));

    dbg!(dayseven::day_7_1());
    dbg!(dayseven::day_7_2());
}
