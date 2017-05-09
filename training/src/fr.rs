use super::*;

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(1), "1", "un", "une");
    example!(v, check_integer(11), "onze");
    example!(v, check_integer(17), "dix sept", "dix-sept");
    example!(v, check_integer(21), "vingt et un", "vingt-et-un");
    example!(v, check_integer(23), "vingt trois", "vingt-trois");
    example!(v, check_integer(70), "soixante dix");
    example!(v, check_integer(78), "soixante dix huit");
    example!(v, check_integer(73), "soixante treize");
    example!(v, check_integer(80), "quatre vingt");
    example!(v, check_integer(81), "quatre vingt un");
    example!(v, check_integer(90), "quatre vingt dix");
    example!(v, check_integer(91), "quatre vingt onze");
    example!(v, check_integer(99), "quatre vingt dix neuf");
    example!(v,
             check_integer(33),
             "33",
             "trente trois",
             "trente-trois",
             "trente 3");
    example!(v,
             check_integer(100000),
             "100.000",
             "100000",
             "100K",
             "100k");
    example!(v,
             check_integer(3000000),
             "3M",
             "3000K",
             "3000000",
             "3.000.000");
    example!(v,
             check_integer(1200000),
             "1.200.000",
             "1200000",
             "1,2M",
             "1200K",
             ",0012G");
    example!(v,
             check_integer(-1200000),
             "- 1.200.000",
             "-1200000",
             "moins 1200000",
             "-1,2M",
             "-1200K",
             "-,0012G");
    example!(v, check_ordinal(1), "1er", "1ere", "le 1er");
    example!(v,
             check_ordinal(3),
             "3ieme",
             "le 3ieme",
             "3eme",
             "3ème",
             "troisième",
             "troisieme");
}
