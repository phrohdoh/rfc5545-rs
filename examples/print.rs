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
}