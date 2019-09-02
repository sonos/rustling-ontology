use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        b.reg(r"(?:%|p\.c\.|p. cents?|pour[ -]?cents?)")?,
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
    b.rule_1_terminal("$",
        b.reg(r#"\$|dollars?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("EUR",
        b.reg(r#"€|(?:[e€]uro?s?)"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
        b.reg(r#"£|livres?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("USD",
        b.reg(r#"us[d\$]|dollars? am[eé]ricains?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("AUD",
        b.reg(r#"au[d\$]|dollars? australiens?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("CAD",
        b.reg(r#"cad|dollars? canadiens?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_1_terminal("HKD",
        b.reg(r#"hkd|dollars? de hong[- ]kong"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("HKD") })
    );
    b.rule_1_terminal("KR",
        b.reg(r#"kr|couronnes?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("KR") })
    );
    b.rule_1_terminal("DKK",
        b.reg(r#"dkk|couronnes? danoises?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("DKK") })
    );
    b.rule_1_terminal("NOK",
        b.reg(r#"nok|couronnes? norv[ée]giennes?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("NOK") })
    );
    b.rule_1_terminal("SEK",
        b.reg(r#"sek|couronnes? su[ée]doises?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("SEK") })
    );
    b.rule_1_terminal("CHF",
        b.reg(r#"chf|francs? suisses?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
    b.rule_1_terminal("RUB",
        b.reg(r#"rub|roubles?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("INR",
        b.reg(r#"inr|roupies?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("JPY",
        b.reg(r#"jpy|yens?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("RMB|CNH|CNY",
        b.reg(r#"cny|cnh|rmb|yuans?|renmimbis?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("CNY") })
    );
    b.rule_1_terminal("¥",
        b.reg(r#"¥"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
        b.reg(r#"₩|krw|wons? (?:sud[- ])?cor[ée]ns?|wons?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("Bitcoin",
        b.reg(r#"฿|bitcoins?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("GBP",
        b.reg(r#"gbp|livres? sterlings?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("GBP") })
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
    b.rule_3("<amount> de <unit>",  // "un million de dollars"
        integer_check!(|integer: &IntegerValue| !integer.group),
        b.reg(r#"d[e']"#)?,
        money_unit!(),
        |a, _, b| {
            Ok(AmountOfMoneyValue {
                value: a.value().value as f32,
                precision: Exact,
                unit: b.value().unit,
                ..AmountOfMoneyValue::default()
            })
    });
    b.rule_3("<amount> de <unit>",  // "une douzaine de dollars"
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
             b.reg(r#"(?:autour|pas loin|pr[eè]s|aux alentours) d[e']|environ|presque|(?:approximative|quasi)ment"#)?,
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
    b.rule_1("number as temp",
            number_check!(),
            |a| {
                 Ok(TemperatureValue {
                    value: a.value().value(),
                    unit: None,
                    latent: true,
                })
            });
    b.rule_2("<latent temp> degrees",
             temperature_check!(),
             b.reg(r#"(?:deg(?:r[éeè])?s?\.?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"centigrades?|c(?:el[cs]?(?:ius)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"f(?:ah?reh?n(?:h?eit)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Kelvin",
             temperature_check!(),
             b.reg(r#"k(?:elvin)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("kelvin"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> en dessous de zero",
             temperature_check!(|temp: &TemperatureValue| !temp.latent),
             b.reg(r#"en dessous de (?:0|z[ée]ro)"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: -1.0 * a.value().value,
                     latent: false,
                     ..*a.value()
                 })
             });
    Ok(())
}
