use rustling::*;
use dimension::*;
use moment::*;
use std::ops;
use regex::Regex;

pub fn compose_numbers(a: &NumberValue, b: &NumberValue) -> RuleResult<NumberValue> {
    let grain = a.grain().unwrap_or(0) as u32;
    if 10u64.pow(grain) as f32 > b.value() && a.value() >= 0.0 && b.value() >= 0.0 {
        match (a, b) {
            (&NumberValue::Integer(ref lhs), &NumberValue::Integer(ref rhs)) => {
                Ok(NumberValue::Integer(IntegerValue::new(lhs.value + rhs.value)?.with_grain(rhs.grain)?))
            }
            _ => Ok(NumberValue::Float(FloatValue::new(a.value() + b.value())?)),
        }
    } else {
        Err(RuleError::Invalid.into())
    }
}

pub fn compose_numbers_from_left(a: &NumberValue, b: &NumberValue) -> RuleResult<NumberValue> {
    if b.combined_from_left() { 
        Err(RuleError::Invalid.into())  
    } else {
        let res = compose_numbers(a, b)?;
        let combination_direction = match &res {
            &NumberValue::Integer(ref integer) if integer.grain.is_none() => {
                None
            }
            _ => Some(CombinationDirection::Left)
        };
        res.combine_from_opt(combination_direction)
    }
}

pub fn compose_duration_with_integer(duration: &DurationValue, value: &IntegerValue) -> RuleResult<DurationValue> {
    let grain = duration.period.finer_grain()
        .ok_or_else(|| -> RuleError { RuleError::Invalid.into() })?;
    let next_grain = grain.next();
    if next_grain <= grain  {
        return Err(RuleError::Invalid.into());
    }
    let period: Period = PeriodComp::new(next_grain, value.value).into();
    Ok(duration + DurationValue::new(period))
}

#[derive(Debug, Clone)]
pub struct RegexMatch<'a> {
    pub groups: Vec<Option<&'a str>>,
}

pub fn find_regex_group<'a>(regex: &Regex, sentence: &'a str) -> RuleResult<Vec<RegexMatch<'a>>> {
    let mut matches = Vec::new();
    for cap in regex.captures_iter(&sentence) {
        let _ = cap.get(0)
                    .ok_or_else(|| format_err!("No capture for regexp {} for sentence: {}", regex, sentence))?;
        let mut groups = Vec::new();
        for group in cap.iter() {
            groups.push(group.map(|g| g.as_str()));
        }
        matches.push(RegexMatch { groups })
    }
    Ok(matches)
}

pub fn decimal_hour_in_minute(a: &str, b: &str) -> RuleResult<i64> {
    let a_value: i64 = a.parse()?;
    let b_value: i64 = b.parse()?;
    Ok((b_value * 6) / 10i64.pow(b.len() as u32 - 1) + a_value * 60)
}

pub fn compose_money(a: &AmountOfMoneyValue,
                     b: &AmountOfMoneyValue)
                     -> RuleResult<AmountOfMoneyValue> {
    let amount = a.value + b.value / 100.0;
    Ok(AmountOfMoneyValue {
           value: amount,
           unit: a.unit,
           ..AmountOfMoneyValue::default()
       })
}

pub fn compose_money_number(a: &AmountOfMoneyValue,
                            b: &NumberValue)
                            -> RuleResult<AmountOfMoneyValue> {
    let amount = a.value + b.value() / 100.0;
    Ok(AmountOfMoneyValue {
           value: amount,
           unit: a.unit,
           ..AmountOfMoneyValue::default()
       })
}

impl Form {
    fn time_of_day_hour(full_hour: u32, is_12_clock: bool) -> Form {
        Form::TimeOfDay(TimeOfDayForm::hour(full_hour, is_12_clock))
    }

    fn time_of_day_hour_minute(full_hour: u32, minute: u32, is_12_clock: bool) -> Form {
        Form::TimeOfDay(TimeOfDayForm::hour_minute(full_hour, minute, is_12_clock))
    }

    fn time_of_day_hour_minute_second(full_hour: u32, minute: u32, second: u32, is_12_clock: bool) -> Form {
        Form::TimeOfDay(TimeOfDayForm::hour_minute_second(full_hour, minute, second, is_12_clock))
    }
    

    fn is_time_of_day(&self) -> bool {
         if let &Form::TimeOfDay(_) = self {
            true
         } else {
            false
         }
    }   
}

impl TimeOfDayForm {

    fn build_time_value(&self, is_12_clock: bool) -> RuleResult<DatetimeValue> {
        match self {
            &TimeOfDayForm::Hour { full_hour, .. } => hour(full_hour, is_12_clock),
            &TimeOfDayForm::HourMinute { full_hour, minute, .. } => hour_minute(full_hour, minute, is_12_clock),
            &TimeOfDayForm::HourMinuteSecond { full_hour, minute, second, .. } => hour_minute_second(full_hour, minute, second, is_12_clock),
        }
    }

    pub fn is_12_clock(&self) -> bool {
         match self {
            &TimeOfDayForm::Hour { is_12_clock, .. } => is_12_clock,
            &TimeOfDayForm::HourMinute { is_12_clock, .. } => is_12_clock,
            &TimeOfDayForm::HourMinuteSecond { is_12_clock, .. } => is_12_clock,
        }
    }

    pub fn full_hour(&self) -> u32 {
        match self {
            &TimeOfDayForm::Hour { full_hour, .. } => full_hour,
            &TimeOfDayForm::HourMinute { full_hour, .. } => full_hour,
            &TimeOfDayForm::HourMinuteSecond { full_hour, .. } => full_hour,
        }
    }
}


pub fn precision_resolution(lhs: Precision, rhs: Precision) -> Precision {
    if lhs == Precision::Approximate || rhs == Precision::Approximate {
        Precision::Approximate
    } else {
        Precision::Exact
    }
}
fn from_addition_resolution(lhs: Option<FromAddition>, rhs: Option<FromAddition>) -> FromAddition {
    match (lhs, rhs) {
        (Some(lhs), None) => lhs,
        (None, Some(rhs)) => rhs,
        (None, None) => FromAddition::Left,
        (Some(_), Some(_)) => FromAddition::Left, 
    }
}
impl DatetimeValue {
    pub fn constraint(constraint: RcConstraint<Local>) -> DatetimeValue {
        DatetimeValue {
            constraint: constraint,
            form: Form::Empty,
            direction: None,
            precision: Precision::Exact,
            latent: false,
            ambiguity: Ambiguity::No,
        }
    }

    pub fn with_latent(self, latent: bool) -> DatetimeValue {
        let ambiguity = if latent {
            Ambiguity::Small
        } else {
            Ambiguity::No
        };
        DatetimeValue { latent, ambiguity, ..self }
    }

    pub fn latent(self) -> DatetimeValue {
        DatetimeValue { latent: true, ambiguity: Ambiguity::Small, ..self }
    }

    pub fn not_latent(self) -> DatetimeValue {
        DatetimeValue { latent: false, ambiguity: Ambiguity::No, .. self }
    }

    pub fn too_ambiguous(self) -> DatetimeValue {
        DatetimeValue { latent: true, ambiguity: Ambiguity::Big, .. self }
    }

    pub fn is_too_ambiguous(&self) -> bool {
        return self.ambiguity == Ambiguity::Big;
    }

    pub fn form(self, form: Form) -> DatetimeValue {
        DatetimeValue { form: form, ..self }
    }

    pub fn direction(self, direction: Option<BoundedDirection>) -> DatetimeValue {
        DatetimeValue {
            direction: direction,
            ..self
        }
    }

    pub fn has_direction(&self) -> bool {
        self.direction.is_some()
    }

    pub fn mark_after_start(self) -> DatetimeValue {
        DatetimeValue {
            direction: Some(BoundedDirection::after_start()),
            .. self
        }
    }

    pub fn mark_after_end(self) -> DatetimeValue {
        DatetimeValue {
            direction: Some(BoundedDirection::after_end()),
            .. self
        }
    }

    pub fn mark_after_end_all(self) -> DatetimeValue {
        DatetimeValue {
            direction: Some(BoundedDirection::after_end_all()),
            .. self
        }
    }

    pub fn mark_before_start(self) -> DatetimeValue {
        DatetimeValue {
            direction: Some(BoundedDirection::before_start()),
            .. self
        }
    }

    pub fn mark_before_end(self) -> DatetimeValue {
        DatetimeValue {
            direction: Some(BoundedDirection::before_end()),
            .. self
        }
    }
    // End moment for IntervalTime and IntantTime
    pub fn mark_before_end_all(self) -> DatetimeValue {
        DatetimeValue {
            direction: Some(BoundedDirection::before_end_all()),
            .. self
        }
    }

    pub fn precision(self, precision: Precision) -> DatetimeValue {
        DatetimeValue {
            precision: precision,
            ..self
        }
    }

    pub fn intersect(&self, other: &DatetimeValue) -> RuleResult<DatetimeValue> {
        Ok(DatetimeValue::constraint(self.constraint.intersect(&other.constraint))
               .direction(self.direction.or(other.direction))
               .precision(precision_resolution(self.precision, other.precision)))
    }

    pub fn last_of(&self, other: &DatetimeValue) -> RuleResult<DatetimeValue> {
        Ok(DatetimeValue::constraint(self.constraint.last_of(&other.constraint))
                .precision(precision_resolution(self.precision, other.precision)))
    }

    pub fn the_nth(&self, n: i64) -> RuleResult<DatetimeValue> {
        Ok(DatetimeValue::constraint(self.constraint.take_the_nth(n))
                .precision(self.precision))
    }

    pub fn the_nth_not_immediate(&self, n: i64) -> RuleResult<DatetimeValue> {
        Ok(DatetimeValue::constraint(self.constraint.take_the_nth_not_immediate(n))
                .precision(self.precision))
    }

    pub fn the_nth_after(&self, n: i64, after_value: &DatetimeValue) -> RuleResult<DatetimeValue> {
        Ok(DatetimeValue::constraint(self.constraint
                                     .the_nth(n)
                                     .after_not_immediate(&after_value.constraint))
                                     .precision(precision_resolution(self.precision, after_value.precision)))
    }

    pub fn smart_span_to(&self, to: &DatetimeValue, is_inclusive: bool) -> RuleResult<DatetimeValue> {
        if self.form.is_time_of_day() && to.form.is_time_of_day() {
            let start_clock = self.form_time_of_day()?;
            let end_clock = to.form_time_of_day()?;
            if !start_clock.is_12_clock() && start_clock.get_hour() > 12 && end_clock.is_12_clock() {
                start_clock.build_time_value(false)?
                    .span_to(&end_clock.build_time_value(false)?, is_inclusive)
            } else if start_clock.is_12_clock() && !end_clock.is_12_clock() && end_clock.get_hour() >= 12 {
                start_clock.build_time_value(false)?
                    .span_to(&end_clock.build_time_value(false)?, is_inclusive)
            } else {
                self.span_to(to, is_inclusive)
            }
        } else {
            self.span_to(to, is_inclusive)
        }
    }

    
    pub fn span_to(&self, to: &DatetimeValue, is_inclusive: bool) -> RuleResult<DatetimeValue> {
        if (self.constraint.grain() == Grain::Day && to.constraint.grain() == Grain::Day) ||
           is_inclusive {
            Ok(DatetimeValue::constraint(self.constraint.span_inclusive_to(&to.constraint))
                    .precision(precision_resolution(self.precision, to.precision)))
        } else {
            Ok(DatetimeValue::constraint(self.constraint.span_to(&to.constraint))
                    .precision(precision_resolution(self.precision, to.precision)))
        }
    }

    pub fn form_month(&self) -> RuleResult<u32> {
        if let Form::Month(m) = self.form {
            Ok(m)
        } else {
            Err(format_err!("Form {:?} is not a month form", self.form))?
        }
    }

    pub fn form_year_month_day(&self) -> RuleResult<Option<YearMonthDayForm>> {
        if let Form::YearMonthDay(v) = self.form {
            Ok(v)
        } else {
            Err(format_err!("Form {:?} is not a month form", self.form))?
        }
    }

    pub fn form_month_day(&self) -> RuleResult<Option<MonthDayForm>> {
        if let Form::MonthDay(v) = self.form {
            Ok(v)
        } else {
            Err(format_err!("Form {:?} is not a month form", self.form))?
        }
    }

    pub fn form_year(&self) -> RuleResult<i32> {
        if let Form::Year(m) = self.form {
            Ok(m)
        } else {
            Err(format_err!("Form {:?} is not a year form", self.form))?
        }
    }

    pub fn form_time_of_day(&self) -> RuleResult<TimeOfDayForm> {
        if let Form::TimeOfDay(v) = self.form.clone() {
            Ok(v)
        } else {
            Err(format_err!("Form {:?} is not a time of day form", self.form))?
        }
    }

    pub fn form_part_of_day(&self) -> RuleResult<PartOfDayForm> {
        if let Form::PartOfDay(v) = self.form.clone() {
            Ok(v)
        } else {
            Err(format_err!("Form {:?} is not a part of day form", self.form))?
        }
    }

    pub fn form_part_of_form(&self) -> RuleResult<PartOfForm> {
        if let Form::PartOfForm(v) = self.form.clone() {
            Ok(v)
        } else {
            Err(format_err!("Form {:?} is not a part of form", self.form))?
        }
    }
}

pub fn normalize_year(y: i32) -> RuleResult<i32> {
    if y <= 99 && y >= 0 {
        Ok((y + 50) % 100 + 2000 - 50)
    } else {
        Ok(y)
    }
}

trait MomentToRuleError {
    type Output;
    fn invalid_if_err(self) -> RuleResult<Self::Output>;
}

impl<T> MomentToRuleError for MomentResult<T> {
    type Output = T;
    fn invalid_if_err(self) -> RuleResult<T> {
        Ok(self.map_err(|_| RuleError::Invalid)?)
    }
}

pub fn year(y: i32) -> RuleResult<DatetimeValue> {
    let y = normalize_year(y)?;
    Ok(DatetimeValue::constraint(Year::new(y)).form(Form::Year(y)))
}

pub fn month(m: u32) -> RuleResult<DatetimeValue> {
    if !(1 <= m && m <= 12) {
        return Err(RuleError::Invalid.into())
    }
    Ok(DatetimeValue::constraint(Month::new(m).invalid_if_err()?).form(Form::Month(m)))
}

pub fn day_of_month(dom: u32) -> RuleResult<DatetimeValue> {
    if !(1 <= dom && dom <= 31) {
        return Err(RuleError::Invalid.into())
    }
    Ok(DatetimeValue::constraint(DayOfMonth::new(dom).invalid_if_err()?).form(Form::DayOfMonth))
}

pub fn day_of_week(weekday: Weekday) -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(DayOfWeek::new(weekday)).form(Form::DayOfWeek { not_immediate: true }))
}

pub fn month_day(m: u32, d: u32) -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(MonthDay::new(m, d).invalid_if_err()?).form(Form::MonthDay(Some(MonthDayForm { month: m, day_of_month: d }))))
}

pub fn year_month_day(y: i32, m: u32, d: u32) -> RuleResult<DatetimeValue> {
    let y = normalize_year(y)?;
    Ok(DatetimeValue::constraint(YearMonthDay::new(y, m, d).invalid_if_err()?).form(Form::YearMonthDay(Some(YearMonthDayForm { year: y, month: m, day_of_month: d }))))
}

pub fn hour(h: u32, is_12_clock: bool) -> RuleResult<DatetimeValue> {
    if is_12_clock {
        Ok(DatetimeValue::constraint(Hour::clock_12(h).invalid_if_err()?).form(Form::time_of_day_hour(h, is_12_clock)))
    } else {
        Ok(DatetimeValue::constraint(Hour::clock_24(h).invalid_if_err()?).form(Form::time_of_day_hour(h, is_12_clock)))
    }
}

pub fn minute(m: u32) -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(Minute::new(m).invalid_if_err()?))
}

pub fn second(s: u32) -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(Second::new(s).invalid_if_err()?))
}

pub fn hour_minute(h: u32, m: u32, is_12_clock: bool) -> RuleResult<DatetimeValue> {
    if is_12_clock {
        Ok(DatetimeValue::constraint(HourMinute::clock_12(h, m).invalid_if_err()?)
            .form(Form::time_of_day_hour_minute(h, m, is_12_clock)))
    } else {
        Ok(DatetimeValue::constraint(HourMinute::clock_24(h, m).invalid_if_err()?)
           .form(Form::time_of_day_hour_minute(h, m, is_12_clock)))
    }
}

pub fn hour_minute_second(h: u32,
                                   m: u32,
                                   s: u32,
                                   is_12_clock: bool)
                                   -> RuleResult<DatetimeValue> {
    Ok(hour_minute(h, m, is_12_clock)?
           .intersect(&second(s)?)?
           .form(Form::time_of_day_hour_minute_second(h, m, s, is_12_clock)))
}

pub fn hour_relative_minute(h: u32, m: i32, is_12_clock: bool) -> RuleResult<DatetimeValue> {
    if !(h <= 23) {
        Err(format_err!("Invalid hour {:?}", h))?
    }
    if !(-59 <= m && m <= 59) {
        Err(format_err!("Invalid relative minutes {:?}", m))?
    }
    let normalized_minute = ((m + 60) % 60) as u32;

    let shifter_hour = if m >= 0 {
        h
    } else {
        match (h, is_12_clock) {
            (0, true) => 23,
            (1, true) => 12,
            (0, false) => 23,
            (1, false) => 0,
            _ => h - 1,
        }
    };
    hour_minute(shifter_hour, normalized_minute, is_12_clock)
}

pub fn cycle(grain: Grain) -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(Cycle::rc(grain)).form(Form::Cycle(grain)))
}

pub fn cycle_nth(grain: Grain, n: i64) -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(Cycle::rc(grain).take_the_nth(n)).form(Form::Cycle(grain)))
}

pub fn cycle_nth_after(grain: Grain, n: i64, after_value: &DatetimeValue) -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(Cycle::rc(grain).the_nth(n).after(&after_value.constraint)).form(Form::Cycle(grain)))
}

pub fn cycle_nth_after_not_immediate(grain: Grain,
                                     n: i64,
                                     after_value: &DatetimeValue)
                                     -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(Cycle::rc(grain)
                                 .the_nth(n)
                                 .after_not_immediate(&after_value.constraint)).form(Form::Cycle(grain)))
}

pub fn cycle_n(grain: Grain, n: i64) -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(Cycle::rc(grain).take(n)).form(Form::Cycle(grain)))
}

pub fn cycle_n_not_immediate(grain: Grain, n: i64) -> RuleResult<DatetimeValue> {
    Ok(DatetimeValue::constraint(Cycle::rc(grain).take_not_immediate(n)).form(Form::Cycle(grain)))
}

pub fn grain_greather_than_day(grain: Grain) -> bool {
    if [Grain::Week, Grain::Month, Grain::Year, Grain::Quarter].contains(&grain) {
        true
    } else {
        false
    }
}

pub fn easter() -> RuleResult<DatetimeValue> {
    fn offset(i: &Interval<Local>, _: &Context<Local>) -> Option<Interval<Local>> {
        let (year, month, day) = computer_easter(i.start.year());
        Some(Interval::ymd(year, month, day))
    }
    Ok(DatetimeValue::constraint(Month::new(3).invalid_if_err()?.translate_with(offset)))
}

pub fn computer_easter(year: i32) -> (i32, u32, u32) {
    let a = year / 100;
    let b = year % 100;
    let c = (3 * (a + 25)) / 4;
    let d = (3 * (a + 25)) % 4;
    let e = (8 * (a + 11)) / 25;
    let f = (5 * a + b) % 19;
    let g = (19 * f + c - e) % 30;
    let h = (f + 11 * g) / 319;
    let j = (60 * (5 - d) + b) / 4;
    let k = (60 * (5 - d) + b) % 4;
    let m = (2 * j - k - g + h) % 7;
    let n = (g - h + m + 114) / 31;
    let p = (g - h + m + 114) % 31;
    let day = (p + 1) as u32;
    let month = n as u32;
    (year, month, day)
}

impl CycleValue {
    pub fn last_of(&self, base: &DatetimeValue) -> RuleResult<DatetimeValue> {
        cycle(self.grain)?.last_of(base)
    }
}

impl DurationValue {

    fn check_period(&self) -> RuleResult<()> {
        if self.period.coarse_num_secs() >= PeriodComp::years(1000).coarse_num_secs() { 
            Err(RuleError::Invalid.into()) 
        } else {
            Ok(())
        }
    }

    pub fn in_present(&self) -> RuleResult<DatetimeValue> {
        self.check_period()?;
        Ok(DatetimeValue::constraint(Cycle::rc(Grain::Second).take_the_nth(0).shift_by(self.period.clone())).precision(self.precision))
    }

    pub fn ago(&self) -> RuleResult<DatetimeValue> {
        self.check_period()?;
        Ok(DatetimeValue::constraint(Cycle::rc(Grain::Second)
                                     .take_the_nth(0)
                                     .shift_by(-self.period.clone())).precision(self.precision))
    }

    pub fn after(&self, datetime: &DatetimeValue) -> RuleResult<DatetimeValue> {
        self.check_period()?;
        Ok(DatetimeValue::constraint(datetime.constraint.shift_by(self.period.clone())).precision(self.precision))
    }

    pub fn before(&self, datetime: &DatetimeValue) -> RuleResult<DatetimeValue> {
        self.check_period()?;
        Ok(DatetimeValue::constraint(datetime.constraint.shift_by(-self.period.clone())).precision(self.precision))
    }
}
impl ops::Add<DurationValue> for DurationValue {
    type Output = DurationValue;
    fn add(self, duration: DurationValue) -> DurationValue {
        DurationValue { 
            period: self.period + duration.period,
            precision: precision_resolution(self.precision, duration.precision),
            suffixed: self.suffixed || duration.suffixed,
            prefixed: self.prefixed || duration.prefixed,
            from_addition: Some(from_addition_resolution(self.from_addition, duration.from_addition))
        }
    }
}

impl<'a> ops::Add<&'a DurationValue> for DurationValue {
    type Output = DurationValue;
    fn add(self, duration: &'a DurationValue) -> DurationValue {
        DurationValue { 
            period: self.period + &duration.period,
            precision: precision_resolution(self.precision, duration.precision),
            suffixed: self.suffixed || duration.suffixed,
            prefixed: self.prefixed || duration.prefixed,
            from_addition: Some(from_addition_resolution(self.from_addition, duration.from_addition))
        }
    }
}

impl<'a, 'b> ops::Add<&'a DurationValue> for &'b DurationValue {
    type Output = DurationValue;
    fn add(self, duration: &'a DurationValue) -> DurationValue {
        DurationValue { 
            period: &self.period + &duration.period,
            precision: precision_resolution(self.precision, duration.precision),
            suffixed: self.suffixed || duration.suffixed,
            prefixed: self.prefixed || duration.prefixed,
            from_addition: Some(from_addition_resolution(self.from_addition, duration.from_addition))
        }
    }
}

impl<'a> ops::Add<DurationValue> for &'a DurationValue {
    type Output = DurationValue;
    fn add(self, duration: DurationValue) -> DurationValue {
        DurationValue { 
            period: &self.period + duration.period,
            precision: precision_resolution(self.precision, duration.precision),
            suffixed: self.suffixed || duration.suffixed,
            prefixed: self.prefixed || duration.prefixed,
            from_addition: Some(from_addition_resolution(self.from_addition, duration.from_addition))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_hour() {
        assert_eq!(90, decimal_hour_in_minute("1", "5").unwrap());
        assert_eq!(93, decimal_hour_in_minute("1", "55").unwrap());
    }

    #[test]
    fn test_computer_easter() {
        assert_eq!((2017, 4, 16), computer_easter(2017));
        assert_eq!((2018, 4, 1), computer_easter(2018));
        assert_eq!((2019, 4, 21), computer_easter(2019));
    }
}
