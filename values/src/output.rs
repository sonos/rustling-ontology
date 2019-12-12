use crate::dimension::*;
use moment::*;
use rustling::Value;

#[derive(Clone, PartialEq, Debug)]
pub enum Output {
    Integer(IntegerOutput),
    Float(FloatOutput),
    Percentage(PercentageOutput),
    Ordinal(OrdinalOutput),
    Datetime(DatetimeOutput),
    DatetimeInterval(DatetimeIntervalOutput),
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
            Output::Datetime(datetime_output_value) => {
                match datetime_output_value.datetime_kind {
                    // Only Date and Time should occur here
                    DatetimeKind::Date => OutputKind::Date,
                    DatetimeKind::Time => OutputKind::Time,
                    DatetimeKind::DatePeriod => OutputKind::DatePeriod,
                    DatetimeKind::TimePeriod => OutputKind::TimePeriod,
                    _ => OutputKind::Datetime,
                }
            }
            Output::DatetimeInterval(datetime_interval_output_value) => {
                match datetime_interval_output_value.datetime_kind {
                    // Only DatePeriod and TimePeriod should occur here
                    DatetimeKind::Date => OutputKind::Date,
                    DatetimeKind::Time => OutputKind::Time,
                    DatetimeKind::DatePeriod => OutputKind::DatePeriod,
                    DatetimeKind::TimePeriod => OutputKind::TimePeriod,
                    _ => OutputKind::Datetime,
                }
            }
            &Output::AmountOfMoney(_) => OutputKind::AmountOfMoney,
            &Output::Temperature(_) => OutputKind::Temperature,
            &Output::Duration(_) => OutputKind::Duration,
            &Output::Percentage(_) => OutputKind::Percentage,
        }
    }
}

enum_kind!(
    OutputKind,
    [
        Number,
        Ordinal,
        Date,
        Time,
        DatePeriod,
        TimePeriod,
        Datetime,
        Duration,
        AmountOfMoney,
        Temperature,
        Percentage
    ]
);

impl OutputKind {
    pub fn to_dim(&self) -> DimensionKind {
        match self {
            &OutputKind::Number => DimensionKind::Number,
            &OutputKind::Ordinal => DimensionKind::Ordinal,
            &OutputKind::Datetime => DimensionKind::Datetime,
            &OutputKind::Date => DimensionKind::Datetime,
            &OutputKind::Time => DimensionKind::Datetime,
            &OutputKind::DatePeriod => DimensionKind::Datetime,
            &OutputKind::TimePeriod => DimensionKind::Datetime,
            &OutputKind::AmountOfMoney => DimensionKind::AmountOfMoney,
            &OutputKind::Temperature => DimensionKind::Temperature,
            &OutputKind::Duration => DimensionKind::Duration,
            &OutputKind::Percentage => DimensionKind::Percentage,
        }
    }

    pub fn match_dim(&self, dimension_value: &Dimension) -> bool {
        match dimension_value {
            Dimension::Datetime(datetime_value) => {
                match self {
                    OutputKind::Date => DatetimeKind::Date == datetime_value.datetime_kind,
                    OutputKind::Time => DatetimeKind::Time == datetime_value.datetime_kind,
                    OutputKind::DatePeriod => {
                        DatetimeKind::DatePeriod == datetime_value.datetime_kind
                    }
                    OutputKind::TimePeriod => {
                        DatetimeKind::TimePeriod == datetime_value.datetime_kind
                    }
                    // If the dimension is datetime and none of the 4 subtypes, then it's the
                    // complement subtype, hence Datetime
                    // This works if the arm matching hasn't matched something first
                    OutputKind::Datetime => true,
                    _ => false,
                }
            }
            _ => self.to_dim() == dimension_value.kind(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct IntegerOutput(pub i64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FloatOutput(pub f64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PercentageOutput(pub f64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OrdinalOutput(pub i64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DatetimeOutput {
    pub moment: Moment<Local>,
    pub grain: Grain,
    pub precision: Precision,
    pub latent: bool,
    pub datetime_kind: DatetimeKind,
}

impl DatetimeOutput {
    pub fn datetime_kind(self, datetime_kind: DatetimeKind) -> DatetimeOutput {
        DatetimeOutput {
            datetime_kind,
            ..self
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DatetimeIntervalOutput {
    pub interval_kind: DatetimeIntervalKind,
    pub datetime_kind: DatetimeKind,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DatetimeIntervalKind {
    After(DatetimeOutput),
    Before(DatetimeOutput),
    Between {
        start: Moment<Local>,
        end: Moment<Local>,
        precision: Precision,
        latent: bool,
    },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AmountOfMoneyOutput {
    pub value: f64,
    pub precision: Precision,
    pub unit: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TemperatureOutput {
    pub value: f64,
    pub unit: Option<&'static str>,
    pub latent: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DurationOutput {
    pub period: Period,
    pub precision: Precision,
}

variant_converters!(Output, Integer, IntegerOutput);
variant_converters!(Output, Float, FloatOutput);
variant_converters!(Output, Percentage, PercentageOutput);
variant_converters!(Output, Ordinal, OrdinalOutput);
variant_converters!(Output, Datetime, DatetimeOutput);
variant_converters!(Output, DatetimeInterval, DatetimeIntervalOutput);
variant_converters!(Output, AmountOfMoney, AmountOfMoneyOutput);
variant_converters!(Output, Temperature, TemperatureOutput);
variant_converters!(Output, Duration, DurationOutput);
