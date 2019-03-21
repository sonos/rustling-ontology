use moment::*;
use dimension::*;
use rustling::Value;

#[derive(Clone,PartialEq,Debug)]
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
//
//impl Output {
//    // Seems unused
//    pub fn kind(&self) -> OutputKind {
//        match self {
//            &Output::Integer(_) => OutputKind::Number,
//            &Output::Float(_) => OutputKind::Number,
//            &Output::Ordinal(_) => OutputKind::Ordinal,
//            &Output::Datetime(_) => OutputKind::Datetime,
//            &Output::DatetimeInterval(_) => OutputKind::Datetime,
//            &Output::AmountOfMoney(_) => OutputKind::AmountOfMoney,
//            &Output::Temperature(_) => OutputKind::Temperature,
//            &Output::Duration(_) => OutputKind::Duration,
//            &Output::Percentage(_) => OutputKind::Percentage,
//        }
//    }
//}

enum_kind!(OutputKind,
    [
        Number,
        Ordinal,
        Duration,
        Datetime,
        Date,
        Time,
        DatePeriod,
        TimePeriod,
        AmountOfMoney,
        Temperature,
        Percentage
    ]
);

impl OutputKind {

    pub fn match_dim(&self, dimension: Dimension) -> bool {
        match dimension {
            Dimension::Datetime(datetime_value) => {
                eprintln!("DatetimeValue: direction={:?}, form={:?}, grain={:?}", datetime_value.direction, datetime_value.form, datetime_value.constraint.coarse_grain_step());
                let has_date_form = Some(true) == datetime_value.form.date_form();
                let has_time_form = Some(true) == datetime_value.form.time_form();
match self {
                    &OutputKind::Date => has_date_form,
                    &OutputKind::Time => Some(true) == has_time_form,
                    &OutputKind::DatePeriod => false,
                    &OutputKind::TimePeriod => false,
                    &OutputKind::Datetime => true,
                    _ => false,
                }
            },
            // temporary
            _ => self.to_dim() == dimension.kind(),
        }
    }

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

}


#[derive(Clone,Copy,PartialEq,Debug)]
pub struct IntegerOutput(pub i64);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct FloatOutput(pub f32);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct PercentageOutput(pub f32);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct OrdinalOutput(pub i64);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct DatetimeOutput {
    pub moment: Moment<Local>, 
    pub grain: Grain, 
    pub precision: Precision,
    pub latent: bool,
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub enum DatetimeIntervalOutput {
    After(DatetimeOutput),
    Before(DatetimeOutput),
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
variant_converters!(Output, Percentage, PercentageOutput);
variant_converters!(Output, Ordinal, OrdinalOutput);
variant_converters!(Output, Datetime, DatetimeOutput);
variant_converters!(Output, DatetimeInterval, DatetimeIntervalOutput);
variant_converters!(Output, AmountOfMoney, AmountOfMoneyOutput);
variant_converters!(Output, Temperature, TemperatureOutput);
variant_converters!(Output, Duration, DurationOutput);