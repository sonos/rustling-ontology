use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(1), "1", "um", "uma");
}

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "0,3%", "cero vírgula tres porcento", "cero virgula tres porcento");
    example!(v, check_percentage(5.0), "5%", "cinco porcento");
    example!(v, check_percentage(355.0), "355 %", "355 por cento", "355 porcento");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "tres graus", "3 graus", "3°", "3 °");
    example!(v, check_temperature(32.0, Some("celsius")), "32°C", "32° C", "32° c", "32°c", "32 °c");
    example!(v, check_temperature(-27.0, Some("degree")), "-27 graus");
    example!(v, check_temperature(-27.0, Some("celsius")), "-27 graus celsius", "menos 27 graus centigrados", "-27C", "-27°C", "-27° C", "-27°c", "-27° c", "- 27 c");
    example!(v, check_temperature(-5.0, Some("fahrenheit")), "menos cinco graus fahrenheit", "cinco graus fahrenheit abaixo de cero", "-5 °F", "-5°F", "-5°f", "-5° f", "- 5°f");
    example!(v, check_temperature(168.0, Some("fahrenheit")), "168 graus fahrenheit", "168° F", "168 F", "168f");
    example!(v, check_temperature(10.0, Some("kelvin")), "dez kelvin", "10°K", "10° K", "10 °K", "10°k", "dez graus kelvin");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(800.0, Some("$"), Precision::Exact), "800 $", "800$");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact), "45€", "45 €", "45 euros");
    example!(v, check_finance(2.05, Some("EUR"), Precision::Exact), "2,05 €", "2,05€", "dois euros e cinco centavos");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "2 £", "2£", "duas libras");
    example!(v, check_finance(10.0, Some("GBP"), Precision::Exact), "dez libras esterlinas");
    example!(v, check_finance(8.0, Some("¥"), Precision::Exact), "8¥");
    example!(v, check_finance(3.0, Some("฿"), Precision::Exact), "3฿", "3 ฿", "tres bitcoins");
}

pub fn examples_datetime(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 12]), "hoje");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 1]), "por uma hora");
}
