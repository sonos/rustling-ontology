use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Grain};
//use rustling_ontology_moment::{Weekday, Grain, PeriodComp, Period};

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));

    b.rule_1_terminal("0..9",
        b.reg(r"(零|一|二|三|四|五|六|七|八|九)")?,
        |text_match| {
            let value = match text_match.group(1).as_ref() {
                "零" => 0,
                "一" => 1,
                "二" => 2,
                "三" => 3,
                "四" => 4,
                "五" => 5,
                "六" => 6,
                "七" => 7,
                "八" => 8,
                "九" => 9,
                _ => return Err(RuleErrorKind::Invalid.into()),
            };
            IntegerValue::new(value) 
        }
    );

    b.rule_1_terminal("10",
        b.reg(r"十")?,
        |_| IntegerValue::new(10)
    );

    b.rule_2("10..19",
        b.reg(r"十")?,
        integer_check_by_range!(1, 9),
        |_, a| IntegerValue::new(a.value().value + 10)
    );

    b.rule_2("20..90",
        integer_check_by_range!(2, 9),
        b.reg(r"十")?,
        |a, _| IntegerValue::new(a.value().value * 10)
    );

    b.rule_3("21..99",
        integer_check_by_range!(2, 9),
        b.reg(r"十")?,
        integer_check_by_range!(1, 9),
        |a, _, b| IntegerValue::new(a.value().value * 10 + b.value().value)
    );

    b.rule_1_terminal("100",
        b.reg(r"百")?,
        |_| IntegerValue::new_with_grain(100, 2)
    );

    b.rule_2("200..900",
        integer_check_by_range!(2, 9),
        b.reg(r"百")?,
        |a, _| {
            Ok(IntegerValue {
                value: a.value().value * 100,
                grain: Some(2),
                ..IntegerValue::default()
            })
        }
    );

    b.rule_1_terminal("1000",
        b.reg(r"千")?,
        |_| IntegerValue::new_with_grain(1000, 3)
    );

    b.rule_2("2000..9000",
        integer_check_by_range!(2, 9),
        b.reg(r"千")?,
        |a, _| {
            Ok(IntegerValue {
                value: a.value().value * 1000,
                grain: Some(3),
                ..IntegerValue::default()
            })
        }
    );

    b.rule_1_terminal("10000",
        b.reg(r"万")?,
        |_| IntegerValue::new_with_grain(10000, 4)
    );

    b.rule_2("10000..99990000",
        integer_check_by_range!(1, 9999),
        b.reg(r"万")?,
        |a, _| {
            Ok(IntegerValue {
                value: a.value().value * 10000,
                grain: Some(4),
                ..IntegerValue::default()
            })
        }
    );

    b.rule_1_terminal("100000000",
        b.reg(r"億")?,
        |_| IntegerValue::new_with_grain(100000000, 8)
    );

    b.rule_2("100000000..999900000000",
        integer_check_by_range!(1, 9999),
        b.reg(r"億")?,
        |a, _| {
            Ok(IntegerValue {
                value: a.value().value * 100000000,
                grain: Some(8),
                ..IntegerValue::default()
            })
        }
    );
    b.rule_2("ordinal number",
            integer_check!(|integer: &IntegerValue| integer.value >= 0),
            b.reg(r"番目")?,
            |integer, _| Ok(OrdinalValue::new(integer.value().value))
    );
    Ok(())
}

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        b.reg(r"パーセント")?,
        |number, _| Ok(PercentageValue(number.value().value()))
    );
    Ok(())
}

pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value()));
    b.rule_3("intersect (and X cents)",
             amount_of_money_check!(),
             b.reg(r#"et"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1_terminal("USD",
        b.reg(r#"ドル"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("EUR",
        b.reg(r#"ユーロ"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("€") })
    );
    b.rule_1_terminal("GBP",
        b.reg(r#"ポンド"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("JPY",
        b.reg(r#"円"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("CNY",
        b.reg(r#"元"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
        b.reg(r#"ウォン"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("₩") })
    );
    b.rule_1_terminal("INR",
        b.reg(r#"ルピー"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("RUB",
        b.reg(r#"ルーブル"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"centimes?|cents?|penn(?:y|ies)|fens?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
    b.rule_1_terminal("unnamed currency",
                      b.reg(r#"(?:balle)s?"#)?,
                      |_| Ok(MoneyUnitValue { unit: None })
    );
    b.rule_2("<amount> <unit>",
             number_check!(),
             money_unit!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: a.value().value(),
                     unit: b.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_3("<amount> de <unit>",
        integer_check!(|integer: &IntegerValue| integer.group),
        b.reg(r#"d[e']"#)?,
        money_unit!(),
        |a, _, b| {
            Ok(AmountOfMoneyValue {
                value: a.value().value as f32,
                precision: Approximate,
                unit: b.value().unit,
                ..AmountOfMoneyValue::default()
            })
    });
    b.rule_2("about <amount-of-money>",
             b.reg(r#"(?:autour|pas loin|pr[eè]s|aux alentours) d[e']|environ|(?:approximative|quasi)ment"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"(?:tr[eè]s )?exactement|pr[eé]cis[eé]ment|pile(?: poil)?"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
        amount_of_money_check!(),
        b.reg(r#"pile(?: poil)?|tout rond"#)?,
        |a, _| {
            Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
            })
        }
    );
    Ok(())
}

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp", number_check!(), |a| {
        Ok(TemperatureValue {
            value: a.value().value(),
            unit: None,
            latent: true,
        })
    });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"度"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });
    b.rule_2("<latent temp> en dessous de zero",
             b.reg(r#"(?:マイナス|零下)"#)?,
             temperature_check!(),
             |_, a| {
                 Ok(TemperatureValue {
                     value: -1.0 * a.value().value,
                     latent: false,
                     ..*a.value()
                 })
             });
    Ok(())
}
