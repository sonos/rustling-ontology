macro_rules! b {
    ($a:expr) => (Box::new($a))
}

macro_rules! integer_check {
    () => ( $crate::rustling::core::AnyNodePattern::<IntegerValue>::new());
    ($min:expr) => ( $crate::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!(|integer: &IntegerValue| integer.value >= $min)]) );
    ($min:expr, $max:expr) => ( $crate::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!(|integer: &IntegerValue| integer.value >= $min && integer.value <= $max)]) );
    ($min:expr, $max:expr, $predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!(|integer: &IntegerValue| integer.value >= $min && integer.value <= $max), b!($predicate)]) );
}

macro_rules! integer_filter {
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<IntegerValue>::filter(vec![b!($predicate)]) );
}

macro_rules! number_check {
    () => ( $crate::rustling::core::AnyNodePattern::<NumberValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<NumberValue>::filter(vec![b!($predicate)]) );
}

macro_rules! ordinal_check {
    () => ( $crate::rustling::core::AnyNodePattern::<OrdinalValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<OrdinalValue>::filter(vec![b!($predicate)]) );
}

macro_rules! amount_of_money_check {
    () => ( $crate::rustling::core::AnyNodePattern::<AmountOfMoneyValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<AmountOfMoneyValue>::filter(vec![b!($predicate)]) );
}

macro_rules! money_unit {
    () => ( $crate::rustling::core::AnyNodePattern::<MoneyUnitValue>::new() );
}

macro_rules! temperature_check {
    () => ( $crate::rustling::core::AnyNodePattern::<TemperatureValue>::new() );
}

macro_rules! time_check {
    () => ( $crate::rustling::core::AnyNodePattern::<TimeValue>::new() );
    ($predicate:expr) => ( $crate::rustling::core::FilterNodePattern::<TimeValue>::filter(vec![b!($predicate)]) );
}

macro_rules! time_check_form {
    ($form:pat) => ( $crate::rustling::core::FilterNodePattern::<TimeValue>::filter(vec![b!(|time: &TimeValue| if let $form = time.form { true } else { false })]) );
}
