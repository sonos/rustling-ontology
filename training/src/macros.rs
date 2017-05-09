macro_rules! check_moment {
    ($($item:expr),*) => ( check_moment(moment!($( $item ),*), grain!($( $item ),*)) );
}

macro_rules! check_moment_span {
    ([$($item1:expr),*], [$($item2:expr),*]) => ( check_moment_span(moment!($( $item1 ),*), moment!($( $item2 ),*), grain!($( $item1 ),*)) );
}

macro_rules! moment {
    ($y:expr) => ( Moment(Local.ymd($y, 1, 1).and_hms(0, 0, 0)));
    ($y:expr, $m:expr) => ( Moment(Local.ymd($y, $m, 1).and_hms(0, 0, 0)) );
    ($y:expr, $m:expr, $d:expr) => ( Moment(Local.ymd($y, $m, $d).and_hms(0, 0, 0)) );
    ($y:expr, $m:expr, $d:expr, $h:expr) => ( Moment(Local.ymd($y, $m, $d).and_hms($h, 0, 0)) );
    ($y:expr, $m:expr, $d:expr, $h:expr, $min:expr) => ( Moment(Local.ymd($y, $m, $d).and_hms($h, $min, 0)) );
    ($y:expr, $m:expr, $d:expr, $h:expr, $min:expr, $sec:expr) => ( Moment(Local.ymd($y, $m, $d).and_hms($h, $min, $sec)) );
}

macro_rules! grain {
    ($y:expr) => (Grain::Year);
    ($y:expr, $m:expr) => (Grain::Month);
    ($y:expr, $m:expr, $d:expr) => (Grain::Day);
    ($y:expr, $m:expr, $d:expr, $h:expr) => (Grain::Hour);
    ($y:expr, $m:expr, $d:expr, $h:expr, $min:expr) => (Grain::Minute);
    ($y:expr, $m:expr, $d:expr, $h:expr, $min:expr, $sec:expr) => (Grain::Second);
}


