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
        Date,
        Time,
        DatePeriod,
        TimePeriod,
        Datetime,
        AmountOfMoney,
        Temperature,
        Percentage
    ]
);

impl OutputKind {

    pub fn match_dim(&self, dimension: Dimension) -> bool {
        match dimension {
            Dimension::Datetime(datetime_value) => {
                let date_time_grain = (datetime_value.constraint.grain_left().date_grain() &&
                    datetime_value.constraint.grain_right().time_grain()) ||
                    (datetime_value.constraint.grain_right().date_grain() &&
                        datetime_value.constraint.grain_left().time_grain());
                let has_date_grain = datetime_value.constraint.grain().date_grain() && !date_time_grain;
                let has_time_grain = datetime_value.constraint.grain().time_grain() && !date_time_grain;
                let is_span = Some(true) == datetime_value.period_form();
                // eprintln!("Value:\tdatetime grain={:?}\tgrain={:?}\tdate={:?}\ttime={:?}\tform={:?}\tspan={:?}",
                          date_time_grain,
                          datetime_value.constraint.grain(),
                          has_date_grain,
                          has_time_grain,
                          datetime_value.form,
                          is_span);
                let date = !is_span && has_date_grain;
                let time = !is_span && has_time_grain;
                let date_period = is_span && has_date_grain;
                let time_period = is_span && has_time_grain;
                // eprintln!("Kind:\tdate={:?}\ttime={:?}\tdateperiod={:?}\ttimeperiod=t{:?}", date, time, date_period, time_period);
                match self {
                    &OutputKind::Date => date,
                    &OutputKind::Time => time,
                    &OutputKind::DatePeriod => date_period,
                    &OutputKind::TimePeriod => time_period,
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