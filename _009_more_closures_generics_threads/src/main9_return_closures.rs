enum TimeOfDay {
    Dawn,
    Day,
    Sunset,
    Night,
}
fn make_fear_closure(input: TimeOfDay) -> impl FnMut(&mut f64) {
    match input {
        TimeOfDay::Dawn => |x: &mut f64| {
            *x *= 0.5;
            println!(
                "The morning sun has vanquished the horrible night.
	You no longer feel afraid.\n Fear: {x}"
            );
        },
        TimeOfDay::Day => |x: &mut f64| {
            *x *= 0.2;
            println!("What a nice day!\n Fear: {x}");
        },
        TimeOfDay::Sunset => |x: &mut f64| {
            *x *= 1.4;
            println!("The sun is almost down! Oh dear.\n Fear: {x}");
        },
        TimeOfDay::Night => |x: &mut f64| {
            *x *= 5.0;
            println!("What a horrible night to have a curse.\n Fear: {x}");
        },
    }
}
fn main() {
    use TimeOfDay::*;
    let mut fear = 10.0;
    let mut make_daytime = make_fear_closure(Day);
    let mut make_sunset = make_fear_closure(Sunset);
    let mut make_night = make_fear_closure(Night);
    let mut make_morning = make_fear_closure(Dawn);
    make_daytime(&mut fear);
    make_sunset(&mut fear);
    make_night(&mut fear);
    make_morning(&mut fear);
}
