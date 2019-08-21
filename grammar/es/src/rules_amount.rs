use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;


pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        // FIXME
        b.reg(r#"(?:%|p\.c\.|por ?cien(?:tos?)?)"#)?,
        |number, _| Ok(PercentageValue(number.value().value()))
    );
    Ok(())
}


pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value()));
    b.rule_3("intersect (and X cents)",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             b.reg(r#"y"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit != Some("cent")),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1_terminal("$",
        b.reg(r#"\$|d[oóò]lar(?:es)?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("EUR",
        b.reg(r#"€|(?:[e€]uro?s?)"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
        b.reg(r#"(?:pound|libra)s?|£"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("USD",
        b.reg(r#"us[d\$]|d[oóò]lar(?:es)? (?:estadounidense|americano)s?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("CAD",
                      b.reg(r#"cad|d[oóò]lar(?:es)? canadienses?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"d[oóò]lar(?:es)? australianos?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("Bitcoin",
        b.reg(r#"฿|bitc[oóò]in(?:e?s)?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("GBP",
        b.reg(r#"gbp|libras? esterlinas?"#)?,
        |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("JPY",
                      b.reg(r#"jpy|yen(?:es)?(?: japoneses?)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("¥",
                      b.reg(r#"¥"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
                      b.reg(r#"₩|krw|won(?:es)?(?: surcoreanos?)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("RMB|CNH|CNY",
                      b.reg(r#"cny|cnh|rmb|yuan(?:es)?(?: chinos?)?|renmimbis?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CNY") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"rupias?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("HKD",
                      b.reg(r#"hkd|d[oóò]lar(?:es)? de hong[- ]kong"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("HKD") })
    );
    b.rule_1_terminal("CHF",
                      b.reg(r#"chf|francos suizos?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
    b.rule_1_terminal("KR",
                      b.reg(r#"kr|coronas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KR") })
    );
    b.rule_1_terminal("DKK",
                      b.reg(r#"dkk|coronas? danesas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("DKK") })
    );
    b.rule_1_terminal("NOK",
                      b.reg(r#"nok|coronas? noruegas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("NOK") })
    );
    b.rule_1_terminal("SEK",
                      b.reg(r#"sek|coronas? suecas?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("SEK") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"c[eéè]nt(?:avo|imo)s?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
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
    b.rule_3("<amount> of <unit>",
             number_check!(),
             b.reg(r#"de"#)?,
             money_unit!(),
             |a, _, b| {
                 Ok(AmountOfMoneyValue {
                     value: a.value().value(),
                     unit: b.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("<unit> <amount>",
             money_unit!(),
             number_check!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: b.value().value(),
                     unit: a.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("about <amount-of-money>",
             b.reg(r#"aproximadamente|sobre|cerca de|casi|un[oa]s"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("<amount-of-money> about",
             amount_of_money_check!(),
             b.reg(r#"m[aáà]s o menos|aproximadamente"#)?,
             |a, _| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"exactamente"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    b.rule_2("<amount-of-money> exactly",
             amount_of_money_check!(),
             b.reg(r#"exactos"#)?,
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
    b.rule_2("<latent temp> temp",
             temperature_check!(),
             b.reg(r#"(?:grados?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"(?:cent(?:i|í)grados?|c(?:el[cs]?(?:ius)?)?\.?)"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });
    // FIXME: Check double Kelvin removal
    b.rule_2("<temp> Kelvin",
             temperature_check!(),
             b.reg(r#"k(?:elvin)?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("kelvin"),
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
    b.rule_2("<latent temp> temp bajo cero",
             temperature_check!(),
             b.reg(r#"bajo cero"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: -1.0 * a.value().value,
                     latent: false,
                     ..*a.value()
                 })
             });
    Ok(())
}
