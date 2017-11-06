macro_rules! b {
    ($a:expr) => (Box::new($a))
}

macro_rules! integer_check_by_range {
    ($min:expr) => ( $crate::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!(|integer: &IntegerValue| integer.value >= $min)]) );
    ($min:expr, $max:expr) => ( $crate::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!(|integer: &IntegerValue| integer.value >= $min && integer.value <= $max)]) );
    ($min:expr, $max:expr, $predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!(|integer: &IntegerValue| integer.value >= $min && integer.value <= $max), b!($predicate)]) );
}

macro_rules! integer_check {
    () => ( $crate::rustling::core::AnyNodePattern::<IntegerValue>::new());
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!($predicate)]) );
}

macro_rules! number_check {
    () => ( $crate::rustling::core::AnyNodePattern::<NumberValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<NumberValue>::filter(vec![b!($predicate)]) );
}

macro_rules! number_check_by_range {
    ($min:expr) => ( $crate::rustling::core::FilterNodePattern::<NumberValue>::filter(vec![b!(|number: &NumberValue| number.value() >= $min)]) );
    ($min:expr, $max:expr)  => ( $crate::rustling::core::FilterNodePattern::<NumberValue>::filter(vec![b!(|number: &NumberValue| number.value() >= $min && number.value() <= $max)]) );
}

macro_rules! ordinal_check {
    () => ( $crate::rustling::core::AnyNodePattern::<OrdinalValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<OrdinalValue>::filter(vec![b!($predicate)]) );
}

macro_rules! ordinal_check_by_range {
    ($min:expr) => ( $crate::rustling::core::FilterNodePattern::<OrdinalValue>::filter(vec![b!(|ordinal: &OrdinalValue| ordinal.value >= $min)]) );
    ($min:expr, $max:expr) => ( $crate::rustling::core::FilterNodePattern::<OrdinalValue>::filter(vec![b!(|ordinal: &OrdinalValue| ordinal.value >= $min && ordinal.value <= $max)]) );
    ($min:expr, $max:expr, $predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<OrdinalValue>::filter(vec![b!(|ordinal: &OrdinalValue| ordinal.value >= $min && integer.value <= $max), b!($predicate)]) );
}

macro_rules! amount_of_money_check {
    () => ( $crate::rustling::core::AnyNodePattern::<AmountOfMoneyValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<AmountOfMoneyValue>::filter(vec![b!($predicate)]) );
}

macro_rules! money_unit {
    () => ( $crate::rustling::core::AnyNodePattern::<MoneyUnitValue>::new() );
}

macro_rules! cycle_check {
    () => ( $crate::rustling::core::AnyNodePattern::<CycleValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<CycleValue>::filter(vec![b!($predicate)]) );
}

macro_rules! unit_of_duration_check {
    () => ( $crate::rustling::core::AnyNodePattern::<UnitOfDurationValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<UnitOfDurationValue>::filter(vec![b!($predicate)]) );
}

macro_rules! temperature_check {
    () => ( $crate::rustling::core::AnyNodePattern::<TemperatureValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<TemperatureValue>::filter(vec![b!($predicate)]) );
}

macro_rules! time_check {
    () => ( $crate::rustling::core::AnyNodePattern::<TimeValue>::new() );
    ($($predicate:expr),*) => ( $crate::rustling::core::FilterNodePattern::<TimeValue>::filter(vec![ $( b!($predicate) ),*]) );
}

macro_rules! duration_check {
    () => ( $crate::rustling::core::AnyNodePattern::<DurationValue>::new() );
}

macro_rules! relative_minute_check {
    () => ( $crate::rustling::core::AnyNodePattern::<RelativeMinuteValue>::new() );
}

macro_rules! form {
    ($form:pat) => (|time: &TimeValue| if let $form = time.form { true } else { false })
}

macro_rules! excluding_form {
    ($form:pat) => (|time: &TimeValue| if let $form = time.form { false } else { true })
}
