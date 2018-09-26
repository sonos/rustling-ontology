use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(10.0, Some("$"), Precision::Exact), "$10", "10$", "dieci dollari");
    example!(v, check_finance(20.0, Some("EUR"), Precision::Exact), "20€", "20 euro");
    example!(v, check_finance(9.0, Some("£"), Precision::Exact),  "£9", "9 sterline");
}

pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 00]), "ora", "adesso");
    example!(v, check_moment!(c, [2013, 2, 11]), "ieri");
    example!(v, check_moment!(c, [2013, 2, 13]), "domani");
    example!(v, check_moment!(c, [2013, 2, 14]), "dopodomani");
    example!(v, check_moment!(c, [2013, 2, 18]), "lunedi", "lun.", "lun");
    example!(v, check_moment!(c, [2013, 2, 19]), "martedi", "mar", "mar.");
    example!(v, check_moment!(c, [2013, 2, 13]), "mercoledi", "mer", "mer.");
    example!(v, check_moment!(c, [2013, 2, 14]), "giovedi", "gio", "gio.");
    example!(v, check_moment!(c, [2013, 2, 15]), "venerdi", "ven", "ven.");
    example!(v, check_moment!(c, [2013, 2, 16]), "sabato", "sab.");
    example!(v, check_moment!(c, [2013, 2, 17]), "domenica", "dom", "dom.");
    // Hours
    example!(v, check_moment!(c, [2013, 2, 12, 15, 0]), "15:00", "15.00");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 0, 10]), "15:00.10");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(1), "1", "uno", "una", "un", "un'");
    example!(v, check_integer(16), "sedici");
    example!(v, check_integer(70), "settanta");
    example!(v, check_integer(80), "ottanta");
    example!(v, check_float(1.1), "1,1", "1,10", "01,10");
    example!(v, check_integer(100000), "100000");
    example!(v, check_integer(300), "trecento");
    example!(v, check_integer(243), "243");
    example!(v,
             check_integer(3000000),
             "3000000",
             "3.000.000");
    example!(v,
             check_integer(1200000),
             "1200000");
    example!(v,
             check_integer(-1200000),
             "- 1.200.000",
             "-1200000");
    example!(v,
             check_float(1.5),
             "1 punto 5",
             "uno punto cinque",
             "1,5");
}
