#[macro_use]
extern crate bencher;
extern crate rustling_ontology_moment;
extern crate chrono;

use rustling_ontology_moment::*;
use bencher::Bencher;
use chrono::{TimeZone, Weekday};
use chrono::offset::local::Local;

fn build_context(moment: Moment) -> Context {
    let now = Interval::starting_at(moment, Grain::Second);
    Context::for_reference(now)
}

fn bench_hour_minute_with_intersection(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = Hour::clock_24(10).intersect(&Minute::new(5));
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_hour_minute(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = HourMinute::clock_24(10, 5);
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_month_day_with_intersection(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = Month::new(10).intersect(&DayOfMonth::new(5));
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_month_day(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = MonthDay::new(10, 5);
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_year_month_day_with_intersection(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = Year::new(2017).intersect(Month::new(10)).intersect(&DayOfMonth::new(5));
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_year_month_day(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = YearMonthDay::new(2017, 10, 5);
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

benchmark_group!(benches,
                 bench_hour_minute,
                 bench_hour_minute_with_intersection,
                 bench_month_day_with_intersection,
                 bench_month_day
                 );
benchmark_main!(benches);