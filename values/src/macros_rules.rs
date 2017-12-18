
#[macro_export]
macro_rules! b {
    ($a:expr) => (Box::new($a))
}


#[macro_export]
macro_rules! integer_check_by_range {
    ($min:expr) => ( ::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!(|integer: &IntegerValue| integer.value >= $min)]) );
    ($min:expr, $max:expr) => ( ::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!(|integer: &IntegerValue| integer.value >= $min && integer.value <= $max)]) );
    ($min:expr, $max:expr, $predicate:expr) => ( ::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!(|integer: &IntegerValue| integer.value >= $min && integer.value <= $max), b!($predicate)]) );
}


#[macro_export]
macro_rules! integer_check {
    () => ( ::rustling::core::AnyNodePattern::<IntegerValue>::new());
    ($predicate:expr) => ( ::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!($predicate)]) );
}


#[macro_export]
macro_rules! number_check {
    () => ( ::rustling::core::AnyNodePattern::<NumberValue>::new() );
    ($predicate:expr) => ( ::rustling::core::FilterNodePattern::<NumberValue>::filter(vec![b!($predicate)]) );
}


#[macro_export]
macro_rules! number_check_by_range {
    ($min:expr) => ( ::rustling::core::FilterNodePattern::<NumberValue>::filter(vec![b!(|number: &NumberValue| number.value() >= $min)]) );
    ($min:expr, $max:expr)  => ( ::rustling::core::FilterNodePattern::<NumberValue>::filter(vec![b!(|number: &NumberValue| number.value() >= $min && number.value() <= $max)]) );
}


#[macro_export]
macro_rules! ordinal_check {
    () => ( ::rustling::core::AnyNodePattern::<OrdinalValue>::new() );
    ($predicate:expr) => ( ::rustling::core::FilterNodePattern::<OrdinalValue>::filter(vec![b!($predicate)]) );
}


#[macro_export]
macro_rules! ordinal_check_by_range {
    ($min:expr) => ( ::rustling::core::FilterNodePattern::<OrdinalValue>::filter(vec![b!(|ordinal: &OrdinalValue| ordinal.value >= $min)]) );
    ($min:expr, $max:expr) => ( ::rustling::core::FilterNodePattern::<OrdinalValue>::filter(vec![b!(|ordinal: &OrdinalValue| ordinal.value >= $min && ordinal.value <= $max)]) );
    ($min:expr, $max:expr, $predicate:expr) => ( ::rustling::core::FilterNodePattern::<OrdinalValue>::filter(vec![b!(|ordinal: &OrdinalValue| ordinal.value >= $min && integer.value <= $max), b!($predicate)]) );
}


#[macro_export]
macro_rules! amount_of_money_check {
    () => ( ::rustling::core::AnyNodePattern::<AmountOfMoneyValue>::new() );
    ($predicate:expr) => ( ::rustling::core::FilterNodePattern::<AmountOfMoneyValue>::filter(vec![b!($predicate)]) );
}


#[macro_export]
macro_rules! money_unit {
    () => ( ::rustling::core::AnyNodePattern::<MoneyUnitValue>::new() );
}


#[macro_export]
macro_rules! cycle_check {
    () => ( ::rustling::core::AnyNodePattern::<CycleValue>::new() );
    ($predicate:expr) => ( ::rustling::core::FilterNodePattern::<CycleValue>::filter(vec![b!($predicate)]) );
}


#[macro_export]
macro_rules! unit_of_duration_check {
    () => ( ::rustling::core::AnyNodePattern::<UnitOfDurationValue>::new() );
    ($predicate:expr) => ( ::rustling::core::FilterNodePattern::<UnitOfDurationValue>::filter(vec![b!($predicate)]) );
}


#[macro_export]
macro_rules! temperature_check {
    () => ( ::rustling::core::AnyNodePattern::<TemperatureValue>::new() );
    ($predicate:expr) => ( ::rustling::core::FilterNodePattern::<TemperatureValue>::filter(vec![b!($predicate)]) );
}


#[macro_export]
macro_rules! time_check {
    () => ( ::rustling::core::AnyNodePattern::<TimeValue>::new() );
    ($($predicate:expr),*) => ( ::rustling::core::FilterNodePattern::<TimeValue>::filter(vec![ $( b!($predicate) ),*]) );
}


#[macro_export]
macro_rules! duration_check {
    () => ( ::rustling::core::AnyNodePattern::<DurationValue>::new() );
}


#[macro_export]
macro_rules! relative_minute_check {
    () => ( ::rustling::core::AnyNodePattern::<RelativeMinuteValue>::new() );
}


#[macro_export]
macro_rules! form {
    ($form:pat) => (|time: &TimeValue| if let $form = time.form { true } else { false })
}


#[macro_export]
macro_rules! excluding_form {
    ($form:pat) => (|time: &TimeValue| if let $form = time.form { false } else { true })
}
