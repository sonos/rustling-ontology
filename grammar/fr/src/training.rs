use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "0,3%", "zéro virgule trois pour cent");
    example!(v, check_percentage(15.0), "15%", "quinze pour cent");
    example!(v, check_percentage(355.0), "355 %", "355 pourcent");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "trois degrés", "3 degrés", "3°", "3 °");
    example!(v, check_temperature(32.0, Some("celsius")), "trente deux degrés celsius", "trente deux degrés centigrade", "32°C", "32 °c");
    example!(v, check_temperature(-27.0, Some("celsius")), "moins 27 celsius", "-27C", "- 27 c");
    example!(v, check_temperature(-5.0, Some("fahrenheit")), "moins cinq degrés fahrenheit", "-5 °F", "- 5°f");
    example!(v, check_temperature(168.0, Some("fahrenheit")), "cent soixante-huit fahrenheit", "168 F", "168f");
    example!(v, check_temperature(10.0, Some("kelvin")), "dix degrés kelvin", "10 °K", "10°k");
    example!(v, check_temperature(21.0, Some("kelvin")), "21 kelvin", "21 K", "21k");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(800.0, Some("$"), Precision::Exact), "800 $", "huit cents dollars");
    example!(v, check_finance(10.0, Some("USD"), Precision::Approximate), "environ dix dollars américains", "près de 10 USD", "presque 10US$");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "3 dollars australiens");
    example!(v, check_finance(3.5, Some("AUD"), Precision::Exact), "3 dollars australiens et cinquante cents");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "zéro dollar de hong kong");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "125 CAD");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact), "45€", "quarante-cinq euros", "quarante cinq euros");
    example!(v, check_finance(2.05, Some("EUR"), Precision::Exact), "deux euros et cinq centimes");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "2 £", "deux livres");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "vingt livres sterling");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "38 francs suisses");
    example!(v, check_finance(447.0, Some("KR"), Precision::Exact), "447 couronnes");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "dix mille DKK");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact), "100 couronnes norvégiennes");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "2005 couronnes suedoises");
    example!(v, check_finance(96.0, Some("INR"), Precision::Approximate), "quasiment 96 roupies");
    example!(v, check_finance(5.3, Some("RUB"), Precision::Exact), "cinq roubles trente");
    example!(v, check_finance(89.0, Some("JPY"), Precision::Exact), "exactement 89 JPY");
    example!(v, check_finance(8.0, Some("¥"), Precision::Exact), "8¥");
    example!(v, check_finance(100.0, Some("CNY"), Precision::Exact), "pile cent yuans", "100 yuan pile");
    example!(v, check_finance(7.0, Some("KRW"), Precision::Exact), "7 wons", "7₩");
    example!(v, check_finance(3.0, Some("฿"), Precision::Exact), "3฿", "3 ฿", "trois bitcoins");
    example!(v, check_finance(15.0, Some("$"), Precision::Approximate), "une quinzaine de dollars");
    example!(v, check_finance(3000000.0, Some("EUR"), Precision::Exact), "trois millions d'euros");
}

pub fn examples_datetime(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
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
    //Hours
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "à quinze heures", "à 15 heures", "à 3 heures cet après-midi", "15h", "15H", "vers 15 heures", "à environ 15 heures");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 0]), "15:00", "15h00", "15H00");
    example!(v, check_moment!(c, [2013, 2, 13, 00]), "minuit");
    example!(v, check_moment!(c, [2013, 2, 12, 12]), "midi", "aujourd'hui à midi");
    example!(v, check_moment!(c, [2013, 2, 12, 12, 15]), "midi et quart", "midi quinze");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 55]), "midi moins cinq");
    example!(v, check_moment!(c, [2013, 2, 12, 12, 30]), "midi et demi", "midi trente");
    example!(v, check_moment!(c, [2013, 2, 13, 00, 03]), "minuit trois");
    example!(v, check_moment!(c, [2013, 2, 12, 00, 03]), "aujourd'hui à minuit trois");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "à quinze heures quinze", "à quinze heures et quinze minutes", "15h passé de 15 minutes", "à trois heures et quart cet après-midi", "15:15", "15h15");
    example!(v, check_moment!(c, [2013, 2, 13, 15, 15]), "à trois heures et quart demain après-midi");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "à quinze heures trente", "à quinze heures passé de trente minutes", "à trois heures et demi cet après-midi", "15:30", "15h30");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 45]), "midi moins le quart", "11h45", "onze heures trois quarts", "aujourd'hui à 11h45");
    example!(v, check_moment!(c, [2013, 2, 13, 11]), "mercredi à 11h");
    example!(v, check_moment!(c, [2013, 2, 13, 11]), "demain à 11 heures", "demain à 11H");
    example!(v, check_moment!(c, [2013, 2, 14, 11]), "jeudi à 11h", "après-demain à 11 heures", "après-demain à 11H");
    example!(v, check_moment!(c, [2013, 2, 15, 12]), "vendredi à midi", "vendredi à 12h");
    example!(v, check_moment!(c, [2013, 2, 15, 16]), "vendredi quinze à seize heures", "vendredi 15 à 16h", "vendredi quinze à 16h");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "dans une seconde");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "dans une minute", "dans 1 min");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 0]), "dans 2 minutes", "dans deux min");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "dans 60 minutes");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "dans une heure");
    example!(v, check_moment!(c, [2013, 2, 12, 2, 30]), "il y a deux heures");
    example!(v, check_moment!(c, [2013, 2, 13, 4, 30]), "dans 24 heures", "dans vingt quatre heures");
    example!(v, check_moment!(c, [2013, 2, 13]), "dans un jour");
    example!(v, check_moment!(c, [2013, 2, 19]), "dans 7 jours");
    example!(v, check_moment!(c, [2013, 2, 19]), "dans 1 semaine", "dans une semaine");
    example!(v, check_moment!(c, [2013, 1, 22]), "il y a trois semaines");
    example!(v, check_moment!(c, [2013, 4, 12]), "dans deux mois");
    example!(v, check_moment!(c, [2012, 11, 12]), "il y a trois mois");
    example!(v, check_moment!(c, [2014, 2]), "dans une année", "dans 1 an");
    example!(v, check_moment!(c, [2011, 2]), "il y a deux ans");
    //Seasons
    example!(v, check_moment_span!(c, [2013, 6, 21], [2013, 9, 24]), "cet été");
    example!(v, check_moment_span!(c, [2012, 12, 21], [2013, 3, 21]), "cet hiver");
    example!(v, check_moment!(c, [2013, 12, 25]), "Noel", "noël", "jour de noel");
    example!(v, check_moment_span!(c, [2013, 12, 24, 18], [2013, 12, 25, 00]), "le soir de noël");
    example!(v, check_moment!(c, [2014, 1, 1]), "jour de l'an", "nouvel an", "premier janvier");
    example!(v, check_moment!(c, [2013, 11, 1]), "la toussaint", "le jour de la toussaint", "la journée de la toussaint", "toussaint", "le jour des morts");
    example!(v, check_moment!(c, [2013, 05, 1]), "fête du travail");
    //Part of day (morning, afternoon...)
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 19]), "cet après-midi", "l'après-midi");
    example!(v, check_moment_span!(c, [2013, 2, 12, 15], [2013, 2, 12, 17]), "en milieu d'après-midi");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 9]), "en début de matinée", "très tôt le matin", "tôt le matin", "le matin tôt", "le matin très tôt");
    example!(v, check_moment_span!(c, [2013, 2, 12, 9], [2013, 2, 12, 11]), "milieu de matinée");
    example!(v, check_moment_span!(c, [2013, 2, 12, 10], [2013, 2, 12, 12]), "en fin de matinée");
    example!(v, check_moment_span!(c, [2013, 2, 12, 13], [2013, 2, 12, 17]), "après déjeuner");
    example!(v, check_moment_span!(c, [2013, 2, 12, 10], [2013, 2, 12, 12]), "avant déjeuner");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "pendant le déjeuner", "à l'heure du déjeuner");
    example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 21]), "après le travail");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 12]), "dès le matin", "dès la matinée");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 15]), "en début d'après-midi", "en début d'aprem");
    example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 19]), "en fin d'après-midi", "en fin d'aprem");
    example!(v, check_moment_span!(c, [2013, 2, 12, 6], [2013, 2, 12, 10]), "en début de journée");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 16]), "milieu de journée");
    example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 21]), "en fin de journée");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 00]), "ce soir");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 12, 21]), "en début de soirée");
    example!(v, check_moment_span!(c, [2013, 2, 12, 21], [2013, 2, 13, 00]), "en fin de soirée");
    example!(v, check_moment_span!(c, [2013, 2, 13, 18], [2013, 2, 14, 00]), "demain soir", "mercredi soir", "mercredi en soirée");
    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 00]), "hier soir", "la veille au soir");
    example!(v, check_moment_span!(c, [2013, 2, 15, 18], [2013, 2, 18, 00]), "ce week-end");
    example!(v, check_moment_span!(c, [2013, 2, 11], [2013, 2, 13]), "en début de semaine");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 15]), "en milieu de semaine");
    example!(v, check_moment_span!(c, [2013, 2, 14], [2013, 2, 18]), "en fin de semaine");
    example!(v, check_moment_span!(c, [2013, 2, 11], [2013, 2, 16]), "en semaine");
    example!(v, check_moment_span!(c, [2013, 2, 19], [2013, 3, 01]), "à la fin du mois");
    //TODO "en début de semaine prochaine" interval (2013, 2, 18) (2013, 2, 20)
    example!(v, check_moment_span!(c, [2013, 9, 6, 18], [2013, 9, 9, 00]), "le premier week-end de septembre");
    example!(v, check_moment_span!(c, [2013, 9, 13, 18], [2013, 9, 16, 00]), "le deuxième week-end de septembre");
    example!(v, check_moment_span!(c, [2013, 9, 27, 18], [2013, 9, 30, 00]), "le dernier week-end de septembre");
    example!(v, check_moment_span!(c, [2013, 2, 18, 4], [2013, 2, 18, 12]), "lundi matin");
    example!(v, check_moment_span!(c, [2013, 2, 18, 12], [2013, 2, 18, 19]), "lundi après-midi", "lundi dans l'après-midi");
    example!(v, check_moment_span!(c, [2013, 2, 18, 17], [2013, 2, 18, 19]), "lundi fin d'après-midi", "lundi en fin d'après-midi");
    example!(v, check_moment_span!(c, [2013, 2, 15, 4], [2013, 2, 15, 12]), "le 15 février dans la matinée", "matinée du 15 février", "le 15 février le matin");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "8 heures ce soir", "8h du soir");
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "3 heures du matin", "3h du mat");
    //Intervals involving cycles
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "2 dernières secondes", "deux dernieres secondes");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "3 prochaines secondes", "trois prochaines secondes");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "2 dernieres minutes", "deux dernières minutes");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "3 prochaines minutes", "trois prochaines minutes");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "3 prochaines heures", "3 heures suivantes");
    example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "2 dernier jours", "deux derniers jour");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "3 prochains jours");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11]), "2 dernieres semaines", "2 semaines passées");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11]), "3 prochaines semaines");
    example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "2 derniers mois");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "3 prochains mois", "3 mois suivant");
    example!(v, check_moment_span!(c, [2011], [2013]), "2 dernieres annees", "2 années passées");
    example!(v, check_moment_span!(c, [2014], [2017]), "3 prochaines années");
    //Explicit intervals
    example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "13-15 juillet", "13 au 15 juillet", "13 jusqu'au 15 juillet", "13 juillet au 15 juillet", "13 juillet - 15 juillet", "entre le 13 et le 15 juillet", "samedi 13 au dimanche 15 juillet", "du samedi 13 au dimanche 15 juillet", "du 13 au dimanche 15 juillet");
    example!(v, check_moment_span!(c, [2013, 7, 1], [2013, 7, 11]), "1er au 10 juillet", "lundi 1er au mercredi 10 juillet", "lundi 1 au mercredi 10 juillet", "du lundi 1er au mercredi 10 juillet", "du 1er au mercredi 10 juillet");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 19]), "du 13 au 18", "entre le 13 et le 18");
    example!(v, check_moment_span!(c, [2023, 2, 1], [2023, 4, 1]), "entre février et mars deux mille vingt-trois");
    example!(v, check_moment_span!(c, [2013, 6, 10], [2013, 7, 2]), "10 juin au 1er juillet", "entre le 10 juin et le 1er juillet", "du 10 juin au 1er juillet");
    example!(v, check_moment_span!(c, [2017,4,6], [2017,6,9]), "du six avril au huit juin deux mille dix-sept");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9, 30], [2013, 2, 14, 11]), "9h30 - 11h00 Jeudi", "de 9h30 jusqu'à 11h jeudi", "de 9 heures 30 à 11h jeudi", "de 9 heures 30 a 11h jeudi", "entre 9h30 et 11h jeudi", "jeudi mais entre 9h30 et 11h", "jeudi par exemple entre 9h30 et 11h");
    example!(v, check_moment_with_direction!(c, [2013, 3, 8], Direction::After), "à partir du 8", "à partir du 8 mars");
    example!(v, check_moment_with_direction!(c, [2013, 2, 14, 9, 30], Direction::After), "à partir de 9h30 jeudi", "jeudi après 9h30", "jeudi matin à partir de 9 heures 30");
    example!(v, check_moment_with_direction!(c, [2013, 11, 1, 16], Direction::After), "après 16h le 1er novembre");
    example!(v, check_moment_with_direction!(c, [2013, 11, 1], Direction::After), "après le 1er novembre");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 16], Direction::Before), "avant 16h", "n'importe quand avant 16h", "jusqu'à 16h");
    example!(v, check_moment_span!(c, [2013, 2, 13, 0], [2013, 2, 13, 16]), "demain jusqu'à 16h", "demain avant 16h");
    example!(v, check_moment_with_direction!(c, [2013, 2, 20, 10], Direction::After), "le 20 à partir de 10h");
    example!(v, check_moment_with_direction!(c, [2013, 2, 15, 12], Direction::After), "vendredi à partir de midi");
    example!(v, check_moment_span!(c, [2013, 2, 20], [2013, 2, 20, 18]), "le 20 jusqu'à 18h");
    example!(v, check_moment_span!(c, [2014, 9, 14], [2014, 9, 21]), "14 - 20 sept. 2014");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 26]), "d'ici 2 semaines");
    //15j != 2 semaines
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 5, 12]), "d'ici 3 mois");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 27]), "dans les 15 jours");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 7]), "de 5 à 7");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9], [2013, 2, 14, 11]), "jeudi de 9h à 11h");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "entre midi et 2");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 30]), "11h30-1h30", "de 11h30 à 1h30", "de 11h30 jusqu'à 1h30");
    example!(v, check_moment!(c, [2013, 9, 21, 13, 30]), "13h30 samedi 21 septembre");
    example!(v, check_moment_span!(c, [2013, 3, 25], [2013, 4, 1]), "fin mars", "fin du mois de mars");
    example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 6]), "début avril", "début du mois d'avril");
    example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 15]), "la première quinzaine d'avril");
    example!(v, check_moment_span!(c, [2013, 4, 15], [2013, 5, 01]), "la deuxième quinzaine d'avril");
    example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 6]), "début avril", "début du mois d'avril");
    example!(v, check_moment_span!(c, [2013, 12, 10], [2013, 12, 20]), "mi-décembre");
    example!(v, check_moment!(c, [2013, 3]), "mars", "en mars", "au mois de mars", "le mois de mars");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 45, 0]), "dans un quart d'heure", "dans 1/4h", "dans 1/4 h", "dans 1/4 heure");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 0, 0]), "dans une demi heure", "dans 1/2h", "dans 1/2 h", "dans 1/2 heure");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 15, 0]), "dans trois quarts d'heure", "dans 3/4h", "dans 3/4 h", "dans 3/4 heure");
    example!(v, check_moment!(c, [2016, 12, 15]), "15.12.2016", "15.12.16");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 2]), "pendant deux heures", "durant deux heures");
    example!(v, check_duration!([0, 0, 0, 1]), "pendant un jour", "une journée");
    example!(v, check_duration!([0, 1, 0]), "durant un mois");
    example!(v, check_duration!([1]), "durant une année");
    example!(v, check_duration!([0, 0, 0, 0, 0, 1, 3]), "pendant une minute et trois secondes");
    example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "environ une heure trente", "environ 1h30");
    example!(v, check_duration!([0, 0, 0, 0, 0, 15], Precision::Approximate), "pendant environ un quart d'heure", "environ 1/4h");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "durant une heure");
    example!(v, check_duration!([0, 0, 2]), "pendant 2 semaines");
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
    example!(v, check_integer(5000), "5 milles", "cinq milles");
    example!(v, check_integer(200000), "deux cents milles");
    example!(v, check_integer(21011), "vingt-et-un mille onze");
    example!(v,
             check_integer(721012),
             "sept cents vingt et un milles douze",
             "sept cents vingt-et-un milles douze");
    example!(v,
             check_integer(31256721),
             "trente et un millions deux cents cinquante six milles sept cents vingt et un"
             );


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
    example!(v, check_float(1.1), "1,1", "1,10", "un virgule dix");
    example!(v, check_float(0.5), "0,5", "0,50", "zéro virgule cinq", "zero point cinq");
    example!(v, check_float(32.75), "32,75", "trente-deux virgule soixante-quinze");
    example!(v, check_float(10.08), "10,08", "dix virgule zéro huit", "dix point zéro huit");
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
