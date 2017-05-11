use super::*;

pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ParsingContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second), 80);
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 00]), "maintenant", "tout de suite");
    example!(v, check_moment!(c, [2013, 2, 12]), "aujourd'hui", "ce jour", "dans la journée", "en ce moment");
    example!(v, check_moment!(c, [2013, 2, 11]), "hier", "le jour d'avant", "le jour précédent", "la veille");
    example!(v, check_moment!(c, [2013, 2, 10]), "avant-hier");
    example!(v, check_moment!(c, [2013, 2, 13]), "demain", "jour suivant", "le jour d'après", "le lendemain", "un jour après");
    example!(v, check_moment!(c, [2013, 2, 14]), "après-demain", "le lendemain du 13 février");
    example!(v, check_moment!(c, [2013, 2, 18]), "lundi", "lun.", "ce lundi");
    example!(v, check_moment!(c, [2013, 2, 18]), "lundi 18 février");
    example!(v, check_moment!(c, [2013, 2, 19]), "mardi");
    example!(v, check_moment!(c, [2013, 2, 13]), "mercredi 13 février");
    example!(v, check_moment!(c, [2013, 2, 14]), "jeudi", "deux jours plus tard", "deux jours après");
    example!(v, check_moment!(c, [2013, 2, 15]), "vendredi");
    example!(v, check_moment!(c, [2013, 2, 16]), "samedi");
    example!(v, check_moment!(c, [2013, 2, 17]), "dimanche");
    example!(v, check_moment!(c, [2013, 3, 1]), "le 1er mars", "premier mars", "le 1 mars", "vendredi 1er mars");
    example!(v, check_moment!(c, [2013, 3, 1]), "le premier mars 2013", "1/3/2013", "2013-03-01");
    example!(v, check_moment!(c, [2013, 3, 2]), "le 2 mars", "2 mars", "le 2/3");
    example!(v, check_moment!(c, [2013, 3, 2, 5]), "le 2 mars à 5h", "2 mars à 5h", "le 2/3 à 5h", "le 2 mars à 5h du matin", "le 2 mars vers 5h", "2 mars vers 5h", "2 mars à environ 5h", "2 mars aux alentours de 5h", "2 mars autour de 5h", "le 2/3 vers 5h");
    example!(v, check_moment!(c, [2013, 3, 2]), "le 2");
    example!(v, check_moment!(c, [2013, 3, 2, 5]), "le 2 à 5h", "le 2 vers 5h", "le 2 à 5h du mat");
    example!(v, check_moment!(c, [2013, 3, 3]), "le 3 mars", "3 mars", "le 3/3");
    example!(v, check_moment!(c, [2013, 4, 5]), "le 5 avril", "5 avril");
    example!(v, check_moment!(c, [2015, 3, 3]), "le 3 mars 2015", "3 mars 2015", "3/3/2015", "2015-3-3", "2015-03-03");
    example!(v, check_moment!(c, [2013, 2, 15]), "le 15 février", "15 février");
    example!(v, check_moment!(c, [2013, 2, 15]), "15/02/2013", "15 fev 2013");
    example!(v, check_moment!(c, [2013, 2, 16]), "le 16");
    example!(v, check_moment!(c, [2013, 2, 16, 18]), "le 16 à 18h", "le 16 vers 18h", "le 16 plutôt vers 18h", "le 16 à 6h du soir", "le 16 vers 6h du soir", "le 16 vers 6h dans la soirée", "samedi 16 à 18h");
    example!(v, check_moment!(c, [2013, 2, 17]), "17 février", "le 17 février", "17/2", "17/02", "le 17/02", "17 02", "17 2", "le 17 02", "le 17 2");
    example!(v, check_moment!(c, [2013, 2, 13]), "mercredi 13"); //when today is Tuesday 12, "mercredi 13" should be tomorrow
    example!(v, check_moment!(c, [2014, 2, 20]), "20/02/2014", "20/2/2014", "20/02/14", "le 20/02/14", "le 20/2/14", "20 02 2014", "20 02 14", "20 2 2014", "20 2 14", "le 20 02 2014", "le 20 02 14", "le 20 2 2014", "le 20 2 14");
    example!(v, check_moment!(c, [2013, 10, 31]), "31 octobre", "le 31 octobre", "31/10", "le 31/10", "31 10", "le 31 10");
    example!(v, check_moment!(c, [2014, 12, 24]), "24/12/2014", "24/12/14", "le 24/12/14", "24 12 2014", "24 12 14", "le 24 12 2014", "le 24 12 14");
    // smart two-digit year resolution
    example!(v, check_moment!(c, [1974, 10, 31]), "31/10/1974", "31/10/74");
    // when today is Tuesday, "lundi prochain" is a week from now
    example!(v, check_moment!(c, [2013, 2, 18]), "lundi prochain", "lundi la semaine prochaine", "lundi de la semaine prochaine");
    // when today is Tuesday, "mardi prochain" is a week from now
    example!(v, check_moment!(c, [2013, 2, 19]), "mardi prochain", "mardi suivant", "mardi d'après", "mardi la semaine prochaine", "mardi de la semaine prochaine", "mardi la semaine suivante", "mardi de la semaine suivante", "mardi la semaine d'après", "mardi de la semaine d'après");
    // when today is Tuesday, "mercredi prochain" should be tomorrow
    example!(v, check_moment!(c, [2013, 2, 13]), "mercredi prochain");
    example!(v, check_moment!(c, [2013, 2, 20]), "mercredi suivant", "mercredi d'après", "mercredi la semaine prochaine", "mercredi de la semaine prochaine", "mercredi la semaine suivante", "mercredi de la semaine suivante", "mercredi la semaine d'après", "mercredi de la semaine d'après");
    example!(v, check_moment!(c, [2013, 2, 25]), "lundi en huit", "lundi en 8");
    // today is Tuesday, so "mardi en huit" is in 2 weeks
    example!(v, check_moment!(c, [2013, 2, 19]), "mardi en huit", "mardi en 8");
    example!(v, check_moment!(c, [2013, 2, 20]), "mercredi en huit", "mercredi en 8");
    example!(v, check_moment!(c, [2013, 3, 4]), "lundi en quinze", "lundi en 15");
    // today is Tuesday, so "mardi en huit" is in 2 weeks
    example!(v, check_moment!(c, [2013, 2, 26]), "mardi en quinze", "mardi en 15");
    example!(v, check_moment!(c, [2013, 2, 27]), "mercredi en quinze", "mercredi en 15");
    example!(v, check_moment!(c, [2013, 2, 11]), "lundi cette semaine");
    example!(v, check_moment!(c, [2013, 2, 12]), "mardi cette semaine");
    example!(v, check_moment!(c, [2013, 2, 13]), "mercredi cette semaine");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "cette semaine", "dans la semaine");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "la semaine dernière");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "la semaine prochaine", "la semaine suivante", "la semaine qui suit");
    example!(v, check_moment!(c, [2013, 1]), "le mois dernier");
    example!(v, check_moment!(c, [2013, 3]), "le mois prochain", "le mois suivant");
    example!(v, check_moment!(c, [2012]), "l'année dernière");
    example!(v, check_moment!(c, [2013]), "cette année");
    example!(v, check_moment!(c, [2014]), "l'année prochaine");
    example!(v, check_moment!(c, [2013, 2, 10]), "dimanche dernier", "dimanche de la semaine dernière");
    example!(v, check_moment!(c, [2013, 10, 3]), "3eme jour d'octobre", "le 3eme jour d'octobre");
    example!(v, check_moment!(c, [2014, 10, 6], Grain::Week), "premiere semaine d'octobre 2014", "la premiere semaine d'octobre 2014");
    example!(v, check_moment!(c, [2013, 10, 7], Grain::Week), "la semaine du 6 octobre", "la semaine du 7 octobre");
    example!(v, check_moment!(c, [2015, 10, 31]), "dernier jour d'octobre 2015", "le dernier jour d'octobre 2015");
    example!(v, check_moment!(c, [2014, 9, 22], Grain::Week), "dernière semaine de septembre 2014", "la dernière semaine de septembre 2014");
}

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
