use super::*;
use rustling_ontology_values::check::*;

pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ParsingContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "jetzt", "genau jetzt", "gerade eben");
    example!(v, check_moment!(c, [2013, 2, 12]), "heute", "zu dieser zeit");
    example!(v, check_moment!(c, [2013, 2, 11]), "gestern");
    example!(v, check_moment!(c, [2013, 2, 13]), "morgen");
    example!(v, check_moment!(c, [2013, 2, 18]), "montag", "mo.", "diesen montag");
    example!(v, check_moment!(c, [2013, 2, 18]), "Montag, Feb 18", "Montag, Februar 18");
    example!(v, check_moment!(c, [2013, 2, 19]), "dienstag");
    example!(v, check_moment!(c, [2013, 2, 14]), "donnerstag", "do", "do.");
    example!(v, check_moment!(c, [2013, 2, 15]), "freitag", "fr.");
    example!(v, check_moment!(c, [2013, 2, 16]), "samstag", "sa.");
    example!(v, check_moment!(c, [2013, 2, 17]), "sonntag", "so.");
    example!(v, check_moment!(c, [2013, 3, 1]), "1 marz", "erster marz"); //"1 märz", "erster märz"
    example!(v, check_moment!(c, [2013, 3, 3]), "marz 3"); //"märz 3"
    example!(v, check_moment!(c, [2015, 3, 3]), "marz 3 2015"); // "märz 3 2015"
    example!(v, check_moment!(c, [2013, 2, 15]), "am 15ten");
    example!(v, check_moment!(c, [2013, 2, 15]), "15. februar", "februar 15", "15te februar", "15.2.", "am 15.2.", "februar 15");
    example!(v, check_moment!(c, [2013, 8, 8]), "Aug 8");
    example!(v, check_moment!(c, [2014, 10]), "Oktober 2014");
    example!(v, check_moment!(c, [1974, 10, 31]), "31.10.1974", "31.10.74");
    example!(v, check_moment!(c, [2015, 4, 14]), "14 april 2015", "April 14, 2015", "14te April 15");
    example!(v, check_moment!(c, [2013, 2, 19]), "nachsten dienstag"); //"nächsten dienstag"
    example!(v, check_moment!(c, [2013, 2, 22]), "ubernachsten freitag"); // "übernächsten freitag"
    example!(v, check_moment!(c, [2013, 3]), "nachsten marz"); // "nachsten marz"
    example!(v, check_moment!(c, [2014, 3]), "ubernachsten marz"); // "ubernachsten marz"
    example!(v, check_moment!(c, [2013, 2, 10]), "Sonntag, Feb 10");
    example!(v, check_moment!(c, [2013, 2, 13]), "Mittwoch, Feb 13");
    example!(v, check_moment!(c, [2013, 2, 18]), "Montag, Feb 18");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "diese woche", "kommende woche");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "letzte woche");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "nachste woche"); //"nächste woche"
    example!(v, check_moment!(c, [2013, 1]), "letzten monat");
    example!(v, check_moment!(c, [2013, 3]), "nachsten monat"); //"nächsten monat"
    example!(v, check_moment!(c, [2013, 1, 1], Grain::Quarter), "dieses quartal");
    example!(v, check_moment!(c, [2013, 4, 1], Grain::Quarter), "nachstes quartal"); //"nachstes quartal"
    example!(v, check_moment!(c, [2013, 7, 1], Grain::Quarter), "drittes quartal");
    example!(v, check_moment!(c, [2018, 10, 1], Grain::Quarter), "4tes quartal 2018");
    example!(v, check_moment!(c, [2012]), "letztes jahr");
    example!(v, check_moment!(c, [2013]), "dieses jahr");
    example!(v, check_moment!(c, [2014]), "nachstes jahr"); //"nächstes jahr"
    example!(v, check_moment!(c, [2013, 2, 10]), "letzten sonntag", "sonntag der letzten woche", "sonntag letzte woche");

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