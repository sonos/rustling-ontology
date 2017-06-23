#[macro_export]
macro_rules! check_finance {
    ($value:expr) => (check_finance($value, None, Precision::Exact));
    ($value:expr, $unit:expr) => (check_finance($value, $unit, Precision::Exact));
    ($value:expr, $unit:expr, $precision:expr) => (check_finance($value, $unit, $precision));
}

#[macro_export]
macro_rules! check_moment {
    ($context:expr, [$($item:expr),*]) => ( ::rustling_ontology_values::check::check_moment($context, moment!($( $item ),*), grain!($( $item ),*), Precision::Exact, None) );
    ($context:expr, [$($item:expr),*], $grain:expr) => ( ::rustling_ontology_values::check::check_moment($context, moment!($( $item ),*), $grain, Precision::Exact, None) );
}

#[macro_export]
macro_rules! check_moment_with_precision {
    ($context:expr, [$($item:expr),*], $precision:expr) => ( ::rustling_ontology_values::check::check_moment($context, moment!($( $item ),*), grain!($( $item ),*), $precision, None) );
}

#[macro_export]
macro_rules! check_moment_with_direction {
    ($context:expr, [$($item:expr),*], $direction:expr) => ( ::rustling_ontology_values::check::check_moment($context, moment!($( $item ),*), grain!($( $item ),*), Precision::Exact, Some($direction)) );
}

#[macro_export]
macro_rules! check_moment_span {
    ($context:expr, [$($item1:expr),*], [$($item2:expr),*]) => ( ::rustling_ontology_values::check::check_moment_span($context, Precision::Exact, moment!($( $item1 ),*), moment!($( $item2 ),*), grain!($( $item1 ),*)) );
    ($context:expr, [$($item1:expr),*], [$($item2:expr),*], $grain:expr) => ( ::rustling_ontology_values::check::check_moment_span($context, Precision::Exact, moment!($( $item1 ),*), moment!($( $item2 ),*), $grain) );
}

#[macro_export]
macro_rules! check_moment_span_with_precision {
    ($context:expr, [$($item1:expr),*], [$($item2:expr),*], $precision:expr) => ( ::rustling_ontology_values::check::check_moment_span($context, $precision, moment!($( $item1 ),*), moment!($( $item2 ),*), grain!($( $item1 ),*)) );
    ($context:expr, [$($item1:expr),*], [$($item2:expr),*], $precision:expr, $grain:expr) => ( ::rustling_ontology_values::check::check_moment_span($context, $precision, moment!($( $item1 ),*), moment!($( $item2 ),*), $grain) );
}

#[macro_export]
macro_rules! moment {
    ($y:expr) => ( Moment(Local.ymd($y, 1, 1).and_hms(0, 0, 0)));
    ($y:expr, $m:expr) => ( Moment(Local.ymd($y, $m, 1).and_hms(0, 0, 0)) );
    ($y:expr, $m:expr, $d:expr) => ( Moment(Local.ymd($y, $m, $d).and_hms(0, 0, 0)) );
    ($y:expr, $m:expr, $d:expr, $h:expr) => ( Moment(Local.ymd($y, $m, $d).and_hms($h, 0, 0)) );
    ($y:expr, $m:expr, $d:expr, $h:expr, $min:expr) => ( Moment(Local.ymd($y, $m, $d).and_hms($h, $min, 0)) );
    ($y:expr, $m:expr, $d:expr, $h:expr, $min:expr, $sec:expr) => ( Moment(Local.ymd($y, $m, $d).and_hms($h, $min, $sec)) );
}

#[macro_export]
macro_rules! grain {
    ($y:expr) => (Grain::Year);
    ($y:expr, $m:expr) => (Grain::Month);
    ($y:expr, $m:expr, $d:expr) => (Grain::Day);
    ($y:expr, $m:expr, $d:expr, $h:expr) => (Grain::Hour);
    ($y:expr, $m:expr, $d:expr, $h:expr, $min:expr) => (Grain::Minute);
    ($y:expr, $m:expr, $d:expr, $h:expr, $min:expr, $sec:expr) => (Grain::Second);
}