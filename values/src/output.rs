use moment::*;
use dimension::*;

#[derive(Clone,PartialEq,Debug)]
pub enum Output {
    Integer(IntegerOutput),
    Float(FloatOutput),
    Ordinal(OrdinalOutput),
    Time(TimeOutput),
    TimeInterval(TimeIntervalOutput),
    AmountOfMoney(AmountOfMoneyOutput),
    Temperature(TemperatureOutput),
    Duration(DurationOutput),
}

impl Output {
    pub fn kind(&self) -> OutputKind {
        match self {
            &Output::Integer(_) => OutputKind::Number,
            &Output::Float(_) => OutputKind::Number,
            &Output::Ordinal(_) => OutputKind::Ordinal,
            &Output::Time(_) => OutputKind::Time,
            &Output::TimeInterval(_) => OutputKind::Time,
            &Output::AmountOfMoney(_) => OutputKind::AmountOfMoney,
            &Output::Temperature(_) => OutputKind::Temperature,
            &Output::Duration(_) => OutputKind::Duration,
        }
    }
}

enum_kind!(OutputKind,
    [
        Number,
        Ordinal,
        Time,
        AmountOfMoney,
        Temperature,
        Duration
    ]
);

impl OutputKind {
    pub fn to_dim(&self) -> DimensionKind {
        match self {
            &OutputKind::Number => DimensionKind::Number,
            &OutputKind::Ordinal => DimensionKind::Ordinal,
            &OutputKind::Time => DimensionKind::Time,
            &OutputKind::AmountOfMoney => DimensionKind::AmountOfMoney,
            &OutputKind::Temperature => DimensionKind::Temperature,
            &OutputKind::Duration => DimensionKind::Duration,
        }
    }
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct IntegerOutput(pub i64);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct FloatOutput(pub f32);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct OrdinalOutput(pub i64);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct TimeOutput {
    pub moment: Moment<Local>, 
    pub grain: Grain, 
    pub precision: Precision,
    pub latent: bool,
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub enum TimeIntervalOutput {
    After(TimeOutput),
    Before(TimeOutput),
    Between { start: Moment<Local>, end: Moment<Local>, precision: Precision, latent: bool }
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct AmountOfMoneyOutput {
    pub value: f32, 
    pub precision: Precision, 
    pub unit: Option<&'static str>,
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct TemperatureOutput {
    pub value: f32, 
    pub unit: Option<&'static str>,
    pub latent: bool,
}

#[derive(Clone,PartialEq,Debug)]
pub struct DurationOutput {
    pub period: Period, 
    pub precision: Precision,
}

variant_converters!(Output, Integer, IntegerOutput);
variant_converters!(Output, Float, FloatOutput);
variant_converters!(Output, Ordinal, OrdinalOutput);
variant_converters!(Output, Time, TimeOutput);
variant_converters!(Output, TimeInterval, TimeIntervalOutput);
variant_converters!(Output, AmountOfMoney, AmountOfMoneyOutput);
variant_converters!(Output, Temperature, TemperatureOutput);
variant_converters!(Output, Duration, DurationOutput);