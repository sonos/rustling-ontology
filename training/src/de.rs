use super::*;
use rustling_ontology_values::check::*;

pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ParsingContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
}
pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "0", "null");
    example!(v, check_integer(1), "1", "eins");
    example!(v, check_integer(3), "3", "drei");
    example!(v, check_integer(30), "30", "dreissig");
    example!(v, check_integer(33), "33", "drei und dreissig", "dreiunddreissig", "0033");
    example!(v, check_integer(14), "14", "vierzehn");
    example!(v, check_integer(16), "16", "sechzehn");
    example!(v, check_integer(17), "17", "siebzehn");
    example!(v, check_integer(18), "18", "achtzehn");
    example!(v, check_integer(200), "200", "zwei hundert");
    example!(v, check_integer(102), "102", "hundert zwei");
    example!(v, check_float(1.1), "1,1", "1 komma 1", "1,10", "01,10");
    example!(v, check_float(0.77), "0,77", ",77");
    example!(v, check_integer(100000), "100.000", "100000", "100K", "100k");
    example!(v, check_integer(3000000), "3M", "3000K", "3000000", "3.000.000");
    example!(v, check_integer(1200000), "1.200.000", "1200000", "1,2M", "1200K", ",0012G");
    example!(v, check_integer(-1200000), "- 1.200.000", "-1200000", "minus 1.200.000", "negativ 1200000", "-1,2M", "-1200K", "-,0012G");
    example!(v, check_integer(5000), "5 tausend", "funf tausend"); // "fünf tausend"
    example!(v, check_integer(200000), "zwei hundert tausend");
    example!(v, check_integer(21311), "ein und zwanzig tausend drei hundert elf");
    example!(v, check_integer(721012), "sieben hundert einundzwanzig tausend zwolf"); //"sieben hundert einundzwanzig tausend zwölf"
    example!(v, check_integer(31256721), "ein und dreissig millionen zwei hundert sechs und funfzig tausend sieben hundert ein und zwanzig"); // ein und dreissig millionen zwei hundert sechs und fünfzig tausend sieben hundert ein und zwanzig
    example!(v, check_ordinal(4), "vierter", "4ter");
    example!(v, check_float(1416.15), "1416,15");
    example!(v, check_float(1416.15), "1.416,15");
    example!(v, check_float(1000000.0), "1.000.000,00")
}