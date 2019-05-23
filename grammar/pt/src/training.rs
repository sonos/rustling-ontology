use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(1), "1", "um", "uma");
    example!(v, check_integer(11), "onze");
    example!(v, check_integer(17), "dezessete");
    example!(v, check_integer(21), "vinte e um");
    example!(v, check_integer(23), "vinte e três");
    example!(v, check_integer(70), "setenta");
    example!(v, check_integer(78), "setenta e oito");
    example!(v, check_integer(73), "setenta e três");
    example!(v, check_integer(80), "oitenta");
    example!(v, check_integer(81), "oitenta e um");
    example!(v, check_integer(90), "noventa");
    example!(v, check_integer(91), "noventa e um");
    example!(v, check_integer(99), "noventa e nove");
    example!(v, check_integer(5000), "cinco mil");
    example!(v, check_integer(200000), "duzentos mil");
    example!(v, check_integer(21011), "vinte e um mil e onze");
    example!(v, check_integer(721012), "setecentos e vinte e um mil e doze");
    example!(v, check_integer(31256721), "trinta e um milhões duzentos e cinquenta e seis mil setecentos e vinte e um");
    example!(v, check_integer(33), "33", "trinta e três");
    example!(v, check_integer(100000), "100.000", "100000", "100K", "100k", "cem mil");
    // TODO: support K for 1000 and M for 1000000 ?
    //example!(v, check_integer(3000000), "3M", "3000000", "3.000.000", "três milhões");
    //example!(v, check_integer(1200000), "1.200.000", "1200000", "1,2M", "um milhão e duzentos mil");
    //example!(v, check_integer(-1200000), "- 1.200.000", "-1200000", "menos 1200000", "-1,2M", "menos um milhão e duzentos mil");
    // TODO : float temperature
    //example!(v, check_float(1.1), "1,1", "1,10", "um vígula um", "um vírgula dez");
    example!(v, check_float(0.5), "0,5", "0,50", "zero vírgula cinco", "zero vírgula cinquenta");
    example!(v, check_float(32.75), "32,75", "trinta e dois vírgula setenta e cinco");
    example!(v, check_float(10.08), "10,08", "dez vírgula zero oito");
    // TODO: Check if want/need support for ordinal special character/overscript
    //example!(v, check_ordinal(1), "1o", "1a", "primeiro", "primeira", "1º", "1ª");
    //example!(v, check_ordinal(3), "3o", "3a", "3º", "3ª", "terceiro", "terceira");
    //example!(v, check_ordinal(2), "segundo", "2º", "2o");
    //example!(v, check_ordinal(5), "quinto", "5º", "5o");
}

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "0,3%", "zero vírgula três por cento");
    example!(v, check_percentage(15.0), "15%", "quinze por cento");
    example!(v, check_percentage(355.0), "355 %", "355 por cento", "trezentos e cinquenta e cinco por cento");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "três graus", "3 graus");
    example!(v, check_temperature(32.0, Some("celsius")), "trinta e dois graus celsius", "trinta e dois graus centígrados", "32°C", "32°c");
    example!(v, check_temperature(-27.0, Some("degree")), "menos 27 graus", "27 graus abaixo de zero","menos vinte e sete graus");
    example!(v, check_temperature(-27.0, Some("celsius")), "menos 27 graus celsius", "menos 27 graus centígrados", "-27°C", "-27°c");
    example!(v, check_temperature(-5.0, Some("fahrenheit")), "menos cinco graus fahrenheit", "cinco graus fahrenheit abaixo de zero", "-5°F", "-5°f");
    example!(v, check_temperature(168.0, Some("fahrenheit")), "cento e sessenta e oito fahrenheit", "cento e sessenta e oito graus fahrenheit", "168 graus fahrenheit", "168°F", "168°f");
    example!(v, check_temperature(10.0, Some("kelvin")), "dez kelvin", "10K", "10k", "dez graus kelvin");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    // TODO: Ask Drica for US$800
    //example!(v, check_finance(800.0, Some("$"), Precision::Exact), "US$800", "$800", "oitocentos dólares");
    example!(v, check_finance(10.0, Some("$"), Precision::Approximate), "uns dez dólares", "dez dólares mais ou menos");
    // TODO: Ask Drica for US$10
    //example!(v, check_finance(10.0, Some("USD"), Precision::Approximate), "uns dez dólares americanos", "US$10 mais ou menos", "quase $10", "uns 10 USD");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "três dólares australianos");
    example!(v, check_finance(3.5, Some("AUD"), Precision::Exact), "3,5 dólares australianos", "três dólares australianos e cinquenta centavos");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "zero dólares de hong kong");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "125 CAD", "cento e vinte e cinco dólares canadenses");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact), "45€", "quarenta e cinco euros");
    // TODO: Support money amounts with cents dois vírgula cinco euros =/= 2.5
    //example!(v, check_finance(2.05, Some("EUR"), Precision::Exact), "2,05€", "dois vírgula cinco euros");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "2£", "duas libras");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "vinte libras esterlinas");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "38 francos suiços", "trinta e oito francos suiços");
    // TODO: Ask Drica for coroa // corona
    //example!(v, check_finance(447.0, Some("KR"), Precision::Exact), "447 coronas", "10 mil kr", "quatrocentos e quarenta e sete coronas");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "dez mil DKK", "10.000 coroas dinamarquesas");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact), "100 coroas norueguesas", "cem coroas norueguesas", "100NOK");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "2005 coroas suecas", "duas mil e cinco coroas suecas");
    example!(v, check_finance(96.0, Some("INR"), Precision::Approximate), "aproximadamente 96 rúpias", "umas noventa e seis rúpias");
    example!(v, check_finance(5.3, Some("RUB"), Precision::Exact), "cinco rublos e trinta centavos", "5,3 rublos");
    example!(v, check_finance(89.0, Some("JPY"), Precision::Exact), "exatamente 89 JPY", "89 ienes", "exatamente oitenta e nove ienes japoneses");
    example!(v, check_finance(8.0, Some("¥"), Precision::Exact), "8¥");
    example!(v, check_finance(100.0, Some("CNY"), Precision::Exact), "cem yuanes exatos", "exatamente 100 yuanes chineses");
    example!(v, check_finance(7.0, Some("KRW"), Precision::Exact), "7KRW", "7₩", "sete wones sul-coreanos");
    example!(v, check_finance(3.0, Some("฿"), Precision::Exact), "3฿", "3BTC", "3XBT", "três bitcoins");
    example!(v, check_finance(15.0, Some("$"), Precision::Approximate), "uns quinze dólares");
    example!(v, check_finance(3000000.0, Some("EUR"), Precision::Exact), "três milhões de euros");
}

pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));

    example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 30]), "das 11h30 à 1h30", "das 11 e meia à uma e meia");
    example!(v, check_moment!(c, [2013, 9, 21, 13, 30]), "às 13:30 de sábado dia 21 de setembro", "às 13h30 de sábado dia 21 de setembro");
    example!(v, check_moment_span!(c, [2013, 3, 25], [2013, 4, 1]), "no final de março", "no fim do mês de março");
    example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 6]), "no começo de abril", "no início do mês de abril");
    // TODO: first half of april, second half of april
    // example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 15]), "a primeira quinzena de abril", "na primeira quinzena do mês de abril");
    // example!(v, check_moment_span!(c, [2013, 4, 15], [2013, 5, 01]), "a segunda quinzena de abril", "na segunda quinzena do mês de abril");
    // fix_example!(v, check_moment_span!(c, [2013, 12, 10], [2013, 12, 20]),  "meados de dezembro", "em meados de dezembro", "na metade do mês de dezembro");
    example!(v, check_moment!(c, [2013, 3]), "março", "em março", "durante o mês de março", "o mês de março");
    // example!(v, check_moment!(c, [2013, 2, 12, 4, 45, 0]), "dentro de quinze minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 0, 0]), "dentro de meia hora", "dentro de trinta minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 15, 0]), "dentro de quarenta e cinco minutos");
    example!(v, check_moment!(c, [2016, 12, 15]), "15.12.2016", "15.12.16", "15/12/2016");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 2]), "durante duas horas", "por duas horas");
    // TODO: todo o dia"
    /// example!(v, check_duration!([0, 0, 0, 1]), "durante um dia", "por um dia", "todo o dia");
    example!(v, check_duration!([0, 1, 0]), "durante um mês", "por um mês");
    example!(v, check_duration!([1]), "durante um ano", "por um ano");
    example!(v, check_duration!([0, 0, 0, 0, 0, 1, 3]), "durante um minuto e três segundos", "um minuto e três segundos");
    //example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "cerca de uma hora e meia", "uma hora e meia mais ou menos", "uma hora e meia aproximadamente", "por volta de 1h30");
    // TODO: Ask Drica mas o menos // mas ou menos (and before / after possible?)
    example!(v, check_duration!([0, 0, 0, 0, 0, 15], Precision::Approximate), "durante um quarto de hora mais o menos", "aproximadamente durante um quarto de hora");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "durante uma hora", "por uma hora");
    example!(v, check_duration!([0, 0, 2]), "durante 2 semanas", "por duas semanas");
}