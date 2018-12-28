use rfc5545::{
    Frequency as Freq,
    RecurRulePart as Part,
    RecurrenceRule as Rule,
};

fn main() {
    // Once per month, for 3 months total.
    println!("{}", Rule::new_with_parts(vec![
        Part::Freq(Freq::Monthly),
        Part::Count(3),
    ]));

    // Once every 3 weeks forever.
    println!("{}", Rule::new_with_parts(vec![
        Part::Freq(Freq::Weekly),
        Part::Interval(3),
    ]));

    // Once every 2 months, for a total of 8 occurrences (over 16 months, 8 * 2 = 16)
    println!("{}", Rule::new_with_parts(vec![
        Part::Interval(2),
        Part::Freq(Freq::Monthly),
        Part::Count(8),
    ]));
}