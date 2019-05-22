use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(1), "1", "um", "uma");
}

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "0,3%", "zero vírgula três por cento");
    example!(v, check_percentage(15.0), "15%", "quinze por cento");
    example!(v, check_percentage(355.0), "355 %", "355 por cento", "trezentos e cinquenta e cinco por cento");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "três graus", "3 graus", "3°");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    // Ask Drica for US$800
    //example!(v, check_finance(800.0, Some("$"), Precision::Exact), "US$800", "$800", "oitocentos dólares");
    example!(v, check_finance(10.0, Some("$"), Precision::Approximate), "uns dez dólares", "dez dólares mais ou menos");
    // Ask Drica for US$10
    //example!(v, check_finance(10.0, Some("USD"), Precision::Approximate), "uns dez dólares americanos", "US$10 mais ou menos", "quase $10", "uns 10 USD");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "três dólares australianos");
    example!(v, check_finance(3.5, Some("AUD"), Precision::Exact), "3,5 dólares australianos", "três dólares australianos e cinquenta centavos");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "zero dólares de hong kong");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "125 CAD", "cento e vinte e cinco dólares canadenses");

    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact), "45€", "quarenta e cinco euros");
    //example!(v, check_finance(2.05, Some("EUR"), Precision::Exact), "2,05€", "dois vírgula cinco euros");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "2£", "duas libras");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "vinte libras esterlinas");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "38 francos suiços", "trinta e oito francos suiços");
    // Ask Drica
    //example!(v, check_finance(447.0, Some("KR"), Precision::Exact), "447 coronas", "10 mil kr", "quatrocentos e quarenta e sete coronas");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "dez mil DKK", "10.000 coroas dinamarquesas");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact), "100 coroas norueguesas", "cem coroas norueguesas", "100NOK");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "2005 coroas suecas", "duas mil e cinco coroas suecas");
    example!(v, check_finance(96.0, Some("INR"), Precision::Approximate), "aproximadamente 96 rúpias", "umas noventa e seis rúpias");
    // TODO: Support money amounts with cents
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
    example!(v, check_moment!(c, [2013, 2, 12]), "hoje");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 1]), "por uma hora");
}