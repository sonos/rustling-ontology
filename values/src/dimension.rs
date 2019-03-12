use std::{fmt, result};

use rustling::*;
use moment::{RcConstraint, Period, Grain, Local};

/// Union of all possible values parsed by the ontology.
rustling_value! {
    #[doc="Union of all possible values parsed by the ontology."]
    #[derive(Clone,PartialEq,Debug)]
    Dimension DimensionKind {
        Number(NumberValue),
        AmountOfMoney(AmountOfMoneyValue),
        Ordinal(OrdinalValue),
        Temperature(TemperatureValue),
        MoneyUnit(MoneyUnitValue),
        Datetime(DatetimeValue),
        Duration(DurationValue),
        Percentage(PercentageValue),
        Cycle(CycleValue),
        UnitOfDuration(UnitOfDurationValue),
        RelativeMinute(RelativeMinuteValue),
    }

    fn latent(v: &Dimension) -> bool {
        match v {
            &Dimension::Number(_) => false,
            &Dimension::Percentage(_) => false,
            &Dimension::AmountOfMoney(_) => false,
            &Dimension::Ordinal(_) => false,
            &Dimension::Temperature(ref temp) => temp.latent,
            &Dimension::MoneyUnit(_) => true,
            &Dimension::Datetime(ref tv) => tv.latent,
            &Dimension::Duration(_) => false,
            &Dimension::Cycle(_) => true,
            &Dimension::UnitOfDuration(_) => true,
            &Dimension::RelativeMinute(_) => true,
        }
    }

    fn extract_payload(v: &Dimension) -> Option<Payload> {
        match v {
            &Dimension::Number(_) => None,
            &Dimension::Percentage(_) => None,
            &Dimension::AmountOfMoney(_) => None,
            &Dimension::Ordinal(_) => None,
            &Dimension::Temperature(_) => None,
            &Dimension::MoneyUnit(_) => None,
            &Dimension::Datetime(ref tv) => Some(Payload(tv.constraint.grain())),
            &Dimension::Duration(_) => None,
            &Dimension::Cycle(_) => None,
            &Dimension::UnitOfDuration(_) => None,
            &Dimension::RelativeMinute(_) => None,
        }
    }
}


impl Dimension {
    pub fn is_too_ambiguous(&self) -> bool {
        match self {
            &Dimension::Number(_) => false,
            &Dimension::Percentage(_) => false,
            &Dimension::AmountOfMoney(_) => false,
            &Dimension::Ordinal(_) => false,
            &Dimension::Temperature(_) => false,
            &Dimension::MoneyUnit(_) => false,
            &Dimension::Datetime(ref tv) => tv.is_too_ambiguous(),
            &Dimension::Duration(_) => false,
            &Dimension::Cycle(_) => true,
            &Dimension::UnitOfDuration(_) => true,
            &Dimension::RelativeMinute(_) => true,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Payload(pub Grain);

impl fmt::Display for Dimension {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            &Dimension::Number(ref number) => {
                match number {
                    &NumberValue::Integer(ref v) => write!(fmt, "Number: {}", v.value),
                    &NumberValue::Float(ref v) => write!(fmt, "Number: {}", v.value),
                }
            }
            &Dimension::Percentage(ref v) => write!(fmt, "Percentage: {}", v.0),
            &Dimension::Ordinal(_) => write!(fmt, "Ordinal"),
            &Dimension::Temperature(_) => write!(fmt, "Temperature"),
            &Dimension::AmountOfMoney(_) => write!(fmt, "AmountOfMoney"),
            &Dimension::MoneyUnit(_) => write!(fmt, "MoneyUnit"),
            &Dimension::Datetime(_) => write!(fmt, "Datetime"),
            &Dimension::Duration(_) => write!(fmt, "Duration"),
            &Dimension::Cycle(_) => write!(fmt, "Cycle"),
            &Dimension::UnitOfDuration(_) => write!(fmt, "UnitOfDuration"),
            &Dimension::RelativeMinute(_) => write!(fmt, "RelativeMinute"),
        }
    }
}

/// Payload for the ordinal numbers of Dimension
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OrdinalValue {
    pub value: i64,
    pub prefixed: bool,
    pub grain: Option<u8>,
}

impl OrdinalValue {
    pub fn new(value: i64) -> OrdinalValue {
        OrdinalValue {
            value,
            prefixed: false,
            grain: None,
        }
    }

    pub fn new_with_grain(value: i64, grain: u8) -> OrdinalValue {
        OrdinalValue {
            value: value,
            prefixed: false,
            grain: Some(grain),
        }
    }

    pub fn prefixed(self) -> OrdinalValue {
        OrdinalValue {
            value: self.value,
            prefixed: true,
            grain: None,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Precision {
    Approximate,
    Exact,
}

impl Default for Precision {
    fn default() -> Precision {
        Precision::Exact
    }
}

/// Payload for the amount of money of Dimension
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct AmountOfMoneyValue {
    pub value: f32,
    pub precision: Precision,
    pub unit: Option<&'static str>,
}

/// Payload for the unit of money of Dimension
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct MoneyUnitValue {
    pub unit: Option<&'static str>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CombinationDirection {
    Left,
    Right,
}

/// Payload for the integral numbers of Dimension
#[derive(Debug, PartialEq, Clone, Default)]
pub struct IntegerValue {
    pub value: i64,
    #[doc(hidden)]
    pub grain: Option<u8>,
    #[doc(hidden)]
    pub group: bool,
    #[doc(hidden)]
    pub prefixed: bool,
    #[doc(hidden)]
    pub suffixed: bool,
    #[doc(hidden)]
    pub combine_from: Option<CombinationDirection>,
    #[doc(hidden)]
    pub precision: Precision,
}

impl IntegerValue {
    pub fn new(value: i64) -> RuleResult<IntegerValue> {
        Ok(IntegerValue {
            value: value,
            grain: None,
            ..IntegerValue::default()
        })
    }
    pub fn new_with_grain(value: i64, grain: u8) -> RuleResult<IntegerValue> {
        Ok(IntegerValue {
            value: value,
            grain: Some(grain),
            ..IntegerValue::default()
        })
    }

    pub fn with_grain(self, grain: Option<u8>) -> RuleResult<IntegerValue> {
        Ok(IntegerValue {
            grain,
            ..self
        })
    }

    #[doc(hidden)]
    pub fn combine_from(self, direction: CombinationDirection) -> RuleResult<IntegerValue> {
        Ok(IntegerValue {
            combine_from: Some(direction),
            ..self
        })
    }

    #[doc(hidden)]
    pub fn combine_from_opt(self, direction: Option<CombinationDirection>) -> RuleResult<IntegerValue> {
        Ok(IntegerValue {
            combine_from: direction,
            ..self
        })
    }

    #[doc(hidden)]
    pub fn combined_from_left(&self) -> bool {
        return self.combine_from == Some(CombinationDirection::Left)
    }

    #[doc(hidden)]
    pub fn combined_from_right(&self) -> bool {
        return self.combine_from == Some(CombinationDirection::Right)
    }
}

impl From<IntegerValue> for Dimension {
    fn from(v: IntegerValue) -> Dimension {
        Dimension::Number(NumberValue::Integer(v))
    }
}

impl NodePayload for IntegerValue {
    type Payload = Payload;
    fn extract_payload(&self) -> Option<Self::Payload> {
        None
    }
}

impl InnerStashIndexable for IntegerValue {
    type Index = DimensionKind;
    fn index() -> Self::Index {
        DimensionKind::Number
    }
}

impl From<FloatValue> for Dimension {
    fn from(v: FloatValue) -> Dimension {
        Dimension::Number(NumberValue::Float(v))
    }
}

impl NodePayload for FloatValue {
    type Payload = Payload;
    fn extract_payload(&self) -> Option<Self::Payload> {
        None
    }
}

impl InnerStashIndexable for FloatValue {
    type Index = DimensionKind;
    fn index() -> Self::Index {
        DimensionKind::Number
    }
}

impl From<IntegerValue> for NumberValue {
    fn from(v: IntegerValue) -> NumberValue {
        NumberValue::Integer(v)
    }
}

impl AttemptFrom<Dimension> for IntegerValue {
    fn attempt_from(v: Dimension) -> Option<IntegerValue> {
        if let Dimension::Number(value) = v {
            if let NumberValue::Integer(integer) = value {
                Some(integer)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl AttemptInto<i64> for Dimension {
    fn attempt_into(self) -> Option<i64> {
        IntegerValue::attempt_from(self.clone()).map(|it| it.value)
    }
}

impl AttemptFrom<Dimension> for FloatValue {
    fn attempt_from(v: Dimension) -> Option<FloatValue> {
        if let Dimension::Number(value) = v {
            if let NumberValue::Float(float) = value {
                Some(float)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Payload for the floating numbers of Dimension
#[derive(Debug, PartialEq, Clone, Default)]
pub struct FloatValue {
    pub value: f32,
    #[doc(hidden)]
    pub prefixed: bool,
    #[doc(hidden)]
    pub suffixed: bool,
    #[doc(hidden)]
    pub combine_from: Option<CombinationDirection>,
    #[doc(hidden)]
    pub precision: Precision,
}

impl FloatValue {
    pub fn new(value: f32) -> RuleResult<FloatValue> {
        Ok(FloatValue {
            value: value,
            ..FloatValue::default()
        })
    }

    #[doc(hidden)]
    pub fn combine_from(self, direction: CombinationDirection) -> RuleResult<FloatValue> {
        Ok(FloatValue {
            combine_from: Some(direction),
            ..self
        })
    }

    #[doc(hidden)]
    pub fn combine_from_opt(self, direction: Option<CombinationDirection>) -> RuleResult<FloatValue> {
        Ok(FloatValue {
            combine_from: direction,
            ..self
        })
    }

    #[doc(hidden)]
    pub fn combined_from_left(&self) -> bool {
        return self.combine_from == Some(CombinationDirection::Left)
    }

    #[doc(hidden)]
    pub fn combined_from_right(&self) -> bool {
        return self.combine_from == Some(CombinationDirection::Right)
    }
}

impl From<FloatValue> for NumberValue {
    fn from(v: FloatValue) -> NumberValue {
        NumberValue::Float(v)
    }
}

/// Enumeration acting as a Number supertype for IntegerValue and FloatValue.
#[derive(Debug, PartialEq, Clone)]
pub enum NumberValue {
    Float(FloatValue),
    Integer(IntegerValue),
}

impl NumberValue {
    #[doc(hidden)]
    pub fn prefixed(&self) -> bool {
        match self {
            &NumberValue::Float(ref v) => v.prefixed,
            &NumberValue::Integer(ref v) => v.prefixed,
        }
    }

    #[doc(hidden)]
    pub fn suffixed(&self) -> bool {
        match self {
            &NumberValue::Float(ref v) => v.suffixed,
            &NumberValue::Integer(ref v) => v.suffixed,
        }
    }

    #[doc(hidden)]
    pub fn combine_from(self, direction: CombinationDirection) -> RuleResult<NumberValue> {
        match self {
            NumberValue::Float(v) => Ok(v.combine_from(direction)?.into()),
            NumberValue::Integer(v) => Ok(v.combine_from(direction)?.into()),
        }
    }

    #[doc(hidden)]
    pub fn combine_from_opt(self, direction: Option<CombinationDirection>) -> RuleResult<NumberValue> {
        match self {
            NumberValue::Float(v) => Ok(v.combine_from_opt(direction)?.into()),
            NumberValue::Integer(v) => Ok(v.combine_from_opt(direction)?.into()),
        }
    }

    #[doc(hidden)]
    pub fn combined_from_left(&self) -> bool {
        match self {
            &NumberValue::Float(ref v) => v.combined_from_left(),
            &NumberValue::Integer(ref v) => v.combined_from_left(),
        }
    }

    #[doc(hidden)]
    pub fn combined_from_right(&self) -> bool {
        match self {
            &NumberValue::Float(ref v) => v.combined_from_right(),
            &NumberValue::Integer(ref v) => v.combined_from_right(),
        }
    }

    #[doc(hidden)]
    pub fn value(&self) -> f32 {
        match self {
            &NumberValue::Float(ref v) => v.value,
            &NumberValue::Integer(ref v) => v.value as f32,
        }
    }

    #[doc(hidden)]
    pub fn grain(&self) -> Option<u8> {
        match self {
            &NumberValue::Float(_) => None,
            &NumberValue::Integer(ref v) => v.grain,
        }
    }
}

/// Payload for the temperatures of Dimension
#[derive(Debug, PartialEq, Clone)]
pub struct TemperatureValue {
    pub value: f32,
    /// Celsius, Fahrenheit, ...
    pub unit: Option<&'static str>,
    /// true if it can not be confirmed that the value is actually a temperature
    pub latent: bool,
}

/// Payload for the cycle of Dimension
#[derive(Debug, PartialEq, Clone)]
pub struct CycleValue {
    pub is_plural: bool,
    pub grain: Grain,
}

impl CycleValue {
    pub fn new(grain: Grain) -> RuleResult<CycleValue> {
        Ok(CycleValue { is_plural: false, grain: grain })
    }

    pub fn mark_as_plural(self) -> RuleResult<CycleValue> {
        Ok(CycleValue { 
            is_plural: true,
            .. self
        })
    }
}

/// Payload for the unit of duration of Dimension
#[derive(Debug, PartialEq, Clone)]
pub struct UnitOfDurationValue {
    pub grain: Grain,
}

impl UnitOfDurationValue {
    pub fn new(grain: Grain) -> UnitOfDurationValue {
        UnitOfDurationValue { grain: grain }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Ambiguity {
    No,
    Small,
    Big,
}

/// Payload for the datetime of Dimension
#[derive(Clone)]
pub struct DatetimeValue {
    pub constraint: RcConstraint<Local>,
    pub form: Form,
    pub direction: Option<BoundedDirection>,
    pub precision: Precision,
    pub latent: bool,
    pub ambiguity: Ambiguity,
}

// We need partial eq to make Dimension partial eq happy, but this is only
// useful for testing.
impl PartialEq for DatetimeValue {
    fn eq(&self, _other: &DatetimeValue) -> bool {
        unimplemented!()
    }
}

impl ::std::fmt::Debug for DatetimeValue {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        write!(fmt, "<DatetimeValue>")
    }
}

impl DatetimeValue {
    pub fn is_coarse_grain_smaller_than(&self, grain: Grain) -> bool {
        (self.constraint.coarse_grain_step() as usize) < (grain as usize)
    }

    pub fn is_coarse_grain_greater_than(&self, grain: Grain) -> bool {
        (self.constraint.coarse_grain_step() as usize) > (grain as usize)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Form {
    Cycle(Grain),
    Year(i32),
    Month(u32),
    DayOfMonth,
    MonthDay(Option<MonthDayForm>),
    YearMonthDay(Option<YearMonthDayForm>),
    TimeOfDay(TimeOfDayForm),
    DayOfWeek { not_immediate: bool },
    PartOfDay(PartOfDayForm),
    PartOfWeek,
    PartOfMonth,
    PartOfYear,
    PartOfForm(PartOfForm),
    Meal,
    Celebration,
    Empty,
}

impl Form {
    pub fn not_immediate(&self) -> Option<bool> {
        match self {
            &Form::Cycle(_) => None,
            &Form::Year(_) => None,
            &Form::Month(_) => None,
            &Form::MonthDay(_) => None,
            &Form::YearMonthDay(_) => None,
            &Form::TimeOfDay(_) => None,
            &Form::DayOfWeek { not_immediate } => Some(not_immediate),
            &Form::Empty => None,
            &Form::PartOfDay { .. } => None,
            &Form::Meal => None,
            &Form::Celebration => None,
            &Form::PartOfMonth => None,
            &Form::PartOfYear => None,
            &Form::DayOfMonth => None,
            &Form::PartOfForm(_) => None,
            &Form::PartOfWeek => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    After,
    Before,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bound {
    Start,
    End { only_interval: bool },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundedDirection {
    pub bound: Bound,
    pub direction: Direction, 
}

impl BoundedDirection {
    pub fn after_start() -> BoundedDirection {
        BoundedDirection {
            bound: Bound::Start,
            direction: Direction::After,
        }
    }

    pub fn after_end() -> BoundedDirection {
        BoundedDirection {
            bound: Bound::End { only_interval: true },
            direction: Direction::After,
        }
    }

    pub fn after_end_all() -> BoundedDirection {
        BoundedDirection {
            bound: Bound::End { only_interval: false },
            direction: Direction::After,
        }
    }

    pub fn before_end() -> BoundedDirection {
        BoundedDirection {
            bound: Bound::End { only_interval: true },
            direction: Direction::Before,
        }
    }

    pub fn before_end_all() -> BoundedDirection {
        BoundedDirection {
            bound: Bound::End { only_interval: false },
            direction: Direction::Before,
        }
    }

    pub fn before_start() -> BoundedDirection {
        BoundedDirection {
            bound: Bound::Start,
            direction: Direction::Before,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Start,
    Middle,
    End,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartOfForm {
    pub inner_form: Box<Form>,
    pub position: Position,
}

impl PartOfForm {
    pub fn start_of(inner_form: Form) -> Form {
        Form::PartOfForm(PartOfForm {
           position: Position::Start,
           inner_form: Box::new(inner_form),
        })
    }

    pub fn middle_of(inner_form: Form) -> Form {
        Form::PartOfForm(PartOfForm {
           position: Position::Middle,
           inner_form: Box::new(inner_form),
        })
    }

    pub fn end_of(inner_form: Form) -> Form {
        Form::PartOfForm(PartOfForm {
           position: Position::End,
           inner_form: Box::new(inner_form),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PartOfDayForm {
    Morning,
    Afternoon,
    Evening,
    Night,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TimeOfDayForm {
    Hour { full_hour: u32, is_12_clock: bool },
    HourMinute {  full_hour: u32, minute: u32, is_12_clock: bool },
    HourMinuteSecond { full_hour: u32, minute: u32, second: u32, is_12_clock: bool },
}

impl TimeOfDayForm {
    pub fn hour(full_hour: u32, is_12_clock: bool) -> TimeOfDayForm {
        TimeOfDayForm::Hour {
            full_hour,
            is_12_clock,
        }
    }

    pub fn hour_minute(full_hour: u32, minute: u32, is_12_clock: bool) -> TimeOfDayForm {
        TimeOfDayForm::HourMinute {
            full_hour,
            is_12_clock,
            minute,
        }
    }

    pub fn hour_minute_second(full_hour: u32, minute: u32, second: u32, is_12_clock: bool) -> TimeOfDayForm {
        TimeOfDayForm::HourMinuteSecond {
            full_hour,
            is_12_clock,
            minute,
            second,
        }
    }

    pub fn get_hour(&self) -> u32 {
        match self {
            &TimeOfDayForm::Hour { full_hour, ..} => full_hour,
            &TimeOfDayForm::HourMinute {  full_hour, .. } => full_hour,
            &TimeOfDayForm::HourMinuteSecond { full_hour, .. } => full_hour,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonthDayForm {
    pub month: u32,
    pub day_of_month: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct YearMonthDayForm {
    pub year: i32,
    pub month: u32,
    pub day_of_month: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PercentageValue(pub f32);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FromAddition {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DurationValue {
    pub period: Period,
    pub precision: Precision,
    pub suffixed: bool,
    pub prefixed: bool,
    pub from_addition: Option<FromAddition>,
}

impl DurationValue {
    pub fn new(period: Period) -> DurationValue {
        DurationValue { period: period, precision: Precision::Exact, suffixed: false, prefixed: false, from_addition: None }
    }

    pub fn precision(self, precision: Precision) -> DurationValue {
        DurationValue { precision: precision, ..self }
    }

    pub fn from_addition(self, from_addition: FromAddition) -> DurationValue {
        DurationValue { from_addition: Some(from_addition), .. self}
    }

    pub fn is_added_by_left(&self) -> bool {
        if let Some(FromAddition::Left) = self.from_addition {
            true
        } else {
            false
        }
    }

    pub fn is_added_by_right(&self) -> bool {
        if let Some(FromAddition::Right) = self.from_addition {
            true
        } else {
            false
        }
    }

    pub fn is_from_addition(&self) -> bool {
        self.from_addition.is_some()
    }

    pub fn suffixed(self) -> DurationValue {
        DurationValue { suffixed: true, .. self}
    }

    pub fn prefixed(self) -> DurationValue {
        DurationValue { prefixed: true, .. self }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RelativeMinuteValue(pub i32);
