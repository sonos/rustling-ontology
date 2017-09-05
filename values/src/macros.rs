#[macro_export]
macro_rules! check_finance {
    ($value:expr) => (check_finance($value, None, Precision::Exact));
    ($value:expr, $unit:expr) => (check_finance($value, $unit, Precision::Exact));
    ($value:expr, $unit:expr, $precision:expr) => (check_finance($value, $unit, $precision));
}

#[macro_export]
macro_rules! check_duration {
    ([$($item:expr),*]) => ( ::rustling_ontology_values::check::check_duration(period!($( $item ),*), Precision::Exact) );
    ([$($item:expr),*], $precision:expr) => ( ::rustling_ontology_values::check::check_duration(period!($( $item ),*), $precision) );
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
macro_rules! period {
    ($y:expr) => ( year_period!($y) );
    ($y:expr, $m:expr) => ( year_period!($y) + month_period!($m) );
    ($y:expr, $m:expr, $w:expr) => ( year_period!($y) + month_period!($m) + week_period!($w) );
    ($y:expr, $m:expr, $w:expr, $d:expr) => ( year_period!($y) + month_period!($m) + week_period!($w) + day_period!($d) );
    ($y:expr, $m:expr, $w:expr, $d:expr, $h:expr) => ( year_period!($y) + month_period!($m) + week_period!($w) + day_period!($d) + hour_period!($h) );
    ($y:expr, $m:expr, $w:expr, $d:expr, $h:expr, $min:expr) => ( year_period!($y) + month_period!($m) + week_period!($w) + day_period!($d) + hour_period!($h) + minute_period!($min) );
    ($y:expr, $m:expr, $w:expr, $d:expr, $h:expr, $min:expr, $sec:expr) => ( year_period!($y) + month_period!($m) + week_period!($w) + day_period!($d) + hour_period!($h) + minute_period!($min) + second_period!($sec) );
}

#[macro_export]
macro_rules! year_period {
    ($y:expr) => ( Period::from(PeriodComp::new(Grain::Year, $y)) );
}

#[macro_export]
macro_rules! month_period {
    ($m:expr) => ( Period::from(PeriodComp::new(Grain::Month, $m)) );
}

#[macro_export]
macro_rules! week_period {
    ($w:expr) => ( Period::from(PeriodComp::new(Grain::Week, $w)) );
}

#[macro_export]
macro_rules! day_period {
    ($d:expr) => ( Period::from(PeriodComp::new(Grain::Day, $d)) );
}

#[macro_export]
macro_rules! hour_period {
    ($h:expr) => ( Period::from(PeriodComp::new(Grain::Hour, $h)) );
}

#[macro_export]
macro_rules! minute_period {
    ($min:expr) => ( Period::from(PeriodComp::new(Grain::Minute, $min)) );
}

#[macro_export]
macro_rules! second_period {
    ($sec:expr) => ( Period::from(PeriodComp::new(Grain::Second, $sec)) );
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

#[macro_export]
macro_rules! enum_kind {
    ($kindname:ident, [$($varname:ident),*]) => {
        #[derive(Debug,Copy,Clone,PartialEq, Hash, Eq)]
        pub enum $kindname {
            $( $varname ),*
        }

        impl $kindname {
            pub fn all() -> Vec<$kindname> {
                vec![
                    $( $kindname::$varname ),*
                ]
            }
        }

        impl ::std::str::FromStr for $kindname {
            type Err=String;
            fn from_str(s: &str) -> ::std::result::Result<$kindname, Self::Err> {
                match s {
                    $(
                        stringify!($varname) => Ok($kindname::$varname),
                    )*
                    _ => Err(format!("{} is not a known {}", s, stringify!($kindname)))
                }
            }
        }

        impl ::std::string::ToString for $kindname {
            fn to_string(&self) -> String {
                match self {
                    $(
                        &$kindname::$varname => stringify!($varname).to_string(),
                    )*
                }
            }
        }
    }
}
