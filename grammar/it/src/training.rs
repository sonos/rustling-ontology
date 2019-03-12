use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_datetime(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    // Days
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 00]), "ora", "adesso", "in questo momento esatto", "in questo preciso istante");
    example!(v, check_moment!(c, [2013, 2, 12]), "oggi", "in questo momento", "in questa giornata");
    example!(v, check_moment!(c, [2013, 2, 11]), "ieri", "il giorno prima", "il giorno precedente", "la vigilia");
    example!(v, check_moment!(c, [2013, 2, 10]), "l'altro ieri", "l'altroieri", "ieri l'altro", "avantieri", "avant'ieri");
    example!(v, check_moment!(c, [2013, 2, 13]), "domani", "il giorno seguente", "il giorno dopo", "l'indomani", "il giorno successivo");
    example!(v, check_moment!(c, [2013, 2, 14]), "dopodomani", "dopo domani", "l'indomani del 13 febbraio");
    example!(v, check_moment!(c, [2013, 2, 18]), "lunedi", "lunedì", "lun.", "lun", "questo lunedì");
    example!(v, check_moment!(c, [2013, 2, 18]), "lunedì 18 febbraio", "il lunedì 18 febbraio", "il lunedi diciotto di febbraio");
    example!(v, check_moment!(c, [2013, 2, 19]), "martedi", "martedì");
    example!(v, check_moment!(c, [2013, 2, 13]), "mercoledì 13 febbraio", "il mercoledi 13 febbraio");
    example!(v, check_moment!(c, [2013, 2, 14]), "giovedì", "due giorni dopo", "due giorni più tardi");
    example!(v, check_moment!(c, [2013, 2, 15]), "venerdi", "venerdì");
    example!(v, check_moment!(c, [2013, 2, 16]), "sabato");
    example!(v, check_moment!(c, [2013, 2, 17]), "domenica");
    example!(v, check_moment!(c, [2013, 3, 1]), "primo marzo", "il primo marzo", "l'uno di marzo", "uno marzo", "il 1 marzo", "venerdì 1° marzo");
    example!(v, check_moment!(c, [2013, 3, 1]), "il primo marzo 2013", "1/3/2013", "01/03/2013", "01/03/13", "01-03-2013");
    example!(v, check_moment!(c, [2013, 3, 2]), "il 2 marzo", "2 marzo", "due marzo", "il 2/3", "il 02/03");
    example!(v, check_moment!(c, [2013, 3, 2]), "il 2");
    example!(v, check_moment!(c, [2013, 3, 3]), "il 3 marzo", "il 3 di marzo", "3 marzo", "il 3/3", "il 03/03");
    example!(v, check_moment!(c, [2013, 4, 5]), "il 5 aprile", "5 aprile", "5 d'aprile", "5 di aprile");
    example!(v, check_moment!(c, [2015, 3, 3]), "il 3 marzo 2015", "il 3 marzo del 2015", "3 marzo 2015", "3/3/2015", "03/03/2015", "03/03/15", "3-3-2015");
    example!(v, check_moment!(c, [2013, 2, 15]), "il 15 febbraio", "15 febbraio", "15 di febbraio");
    example!(v, check_moment!(c, [2013, 2, 15]), "15/02/2013", "15/02/13", "15 feb. 2013", "15 feb 2013");
    example!(v, check_moment!(c, [2013, 2, 16]), "il 16", "il sedici");

    example!(v, check_moment!(c, [2013, 2, 17]), "17 febbraio", "il 17 febbraio", "17/2", "17/02", "il 17/02", "17.02", "il 17.02", "il 17-02");
    example!(v, check_moment!(c, [2013, 2, 13]), "mercoledì 13"); // when today is Tuesday 12, "mercoledì 13" should be tomorrow
    example!(v, check_moment!(c, [2014, 2, 20]), "20/02/2014", "20/2/2014", "20/02/14", "20.02.2014", "20.02.14", "20-02-2014", "20-2-2014", "20-02-14");
    example!(v, check_moment!(c, [2013, 10, 31]), "31 ottobre", "il 31 ottobre", "31/10", "il 31/10", "31.10", "il 31.10", "31-10", "il 31-10");
    example!(v, check_moment!(c, [2014, 12, 24]), "24/12/2014", "24/12/14", "il 24/12/14", "24.12.2014", "24.12.14", "il 24-12-2014", "il 24-12-14");
    //  smart two-digit year resolution
    example!(v, check_moment!(c, [1974, 10, 31]), "31/10/1974", "31/10/74");
    //  when today is Tuesday, "lundi prochain" is a week from now
    example!(v, check_moment!(c, [2013, 2, 18]), "lunedì prossimo", "il prossimo lunedì", "lunedì della settimana prossima", "il lunedì della settimana prossima", "lunedì della prossima settimana", "il lunedì della prossima settimana");
    //  when today is Tuesday, "martedì prochain" is a week from now
    example!(v, check_moment!(c, [2013, 2, 19]), "martedì prossimo", "il martedì seguente", "il prossimo martedì", "martedì della settimana prossima", "martedì della prossima settimana", "martedì della settimana che viene", "martedì della settimana seguente", "martedì della settimana successiva", "martedì della settimana dopo");
    //  when today is Tuesday, "mercredi prochain" should be tomorrow
    example!(v, check_moment!(c, [2013, 2, 13]), "mercoledì prossimo", "mercoledi prossimo", "il prossimo mercoledì");
    example!(v, check_moment!(c, [2013, 2, 13]), "il mercoledì seguente", "il mercoledì successivo");
    example!(v, check_moment!(c, [2013, 2, 20]), "il mercoledì della settimana prossima", "il mercoledì della prossima settimana", "il mercoledì della settimana seguente", "il mercoledì della settimana successiva", "il mercoledì della settimana che viene", "mercoledì della settimana dopo");
    // fix_example!(v, check_moment!(c, [2013, 2, 11]), "lunedì di questa settimana", "il lunedì di questa settimana");
    // fix_example!(v, check_moment!(c, [2013, 2, 12]), "martedì di questa settimana", "il martedì di questa settimana");
    // fix_example!(v, check_moment!(c, [2013, 2, 13]), "mercoledì di questa settimana", "il mercoledì di questa settimana");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "questa settimana", "in settimana");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "la settimana scorsa", "la settimana passata", "la scorsa settimana");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "la settimana prossima", "la prossima settimana", "la settimana seguente", "la settimana che viene");
    example!(v, check_moment!(c, [2013, 1]), "il mese scorso", "lo scorso mese", "il mese passato");
    example!(v, check_moment!(c, [2013, 3]), "il prossimo mese", "il mese prossimo", "il mese seguente", "il mese che viene", "il mese successivo");
    example!(v, check_moment!(c, [2012]), "l'anno scorso", "lo scorso anno", "l'anno passato");
    example!(v, check_moment!(c, [2013]), "quest'anno");
    example!(v, check_moment!(c, [2014]), "l'anno prossimo", "il prossimo anno");
    example!(v, check_moment!(c, [2013, 2, 10]), "domenica scorsa", "la scorsa domenica", "domenica della settimana scorsa", "domenica della scorsa settimana", "la domenica della settimana scorsa", "la domenica della scorsa settimana");
    example!(v, check_moment!(c, [2013, 10, 3]), "terzo giorno di ottobre", "il 3° giorno d'ottobre");
    example!(v, check_moment!(c, [2014, 10, 6], Grain::Week), "prima settimana dell'ottobre 2014", "la prima settimana dell'ottobre 2014", "la prima settimana dell'ottobre del 2014", "la prima settimana di ottobre del 2014");
    example!(v, check_moment!(c, [2013, 10, 7], Grain::Week), "la settimana del 6 ottobre", "la settimana del 7 ottobre");
    example!(v, check_moment!(c, [2015, 10, 31]), "ultimo giorno dell'ottobre 2015", "l'ultimo giorno dell'ottobre 2015");
    example!(v, check_moment!(c, [2014, 9, 22], Grain::Week), "ultima settimana del settembre 2014", "l'ultima settimana del settembre 2014", "l'ultima settimana del settembre del 2014", "l'ultima settimana di settembre del 2014");


    // Day times
    // FIXME: integer alone for round hours not working + other issues with hours
    //  "alle 3 del pomeriggio", "alle 3 di pomeriggio", "15h", , "verso le 15", "verso le ore 15", "intorno alle 15", "alle 15:00"
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "alle quindici", "alle ore quindici");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 0]), "15:00", "15.00");
    example!(v, check_moment!(c, [2013, 2, 13, 00]), "mezzanotte");
    example!(v, check_moment!(c, [2013, 2, 12, 12]), "mezzogiorno", "mezzodì", "mezzodí", "mezzodi", "oggi a mezzogiorno");
    // FIXME: "dodici e quindici", "mezzogiorno e quindici", "dodici e quindici minuti"
    example!(v, check_moment!(c, [2013, 2, 12, 12, 15]), "mezzogiorno e un quarto");
    // FIXME: "dodici meno cinque"
    example!(v, check_moment!(c, [2013, 2, 12, 11, 55]), "mezzogiorno meno cinque");
    // FIXME: "dodici e trenta"
    example!(v, check_moment!(c, [2013, 2, 12, 12, 30]), "mezzogiorno e mezza", "mezzogiorno e mezzo", "mezzogiorno e trenta");
    example!(v, check_moment!(c, [2013, 2, 13, 00, 03]), "mezzanotte e tre");
    example!(v, check_moment!(c, [2013, 2, 12, 00, 03]), "oggi a mezzanotte e tre");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "alle quindici e quindici", "alle ore quindici e quindici minuti", "alle quindici e un quarto", "alle tre e un quarto questo pomeriggio", "alle tre e un quarto di pomeriggio", "alle tre e quindici oggi pomeriggio", "15:15", "15h15");
    example!(v, check_moment!(c, [2013, 2, 13, 15, 15]), "alle tre e un quarto domani pomeriggio");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "alle quindici e trenta", "alle quindici e trenta minuti", "alle tre e mezzo del pomeriggio", "alle tre e mezza di pomeriggio", "15:30", "15h30");
    // FIXME: "undici e tre quarti",
    example!(v, check_moment!(c, [2013, 2, 12, 11, 45]), "mezzogiorno meno un quarto", "11:45", "alle undici e quaranta cinque", "oggi alle 11:45");

    // Day + day time
    // TODO: Fix next for "[...] alle 5",
    example!(v, check_moment!(c, [2013, 3, 2, 5, 0]), "2 marzo alle 05:00", "il 2 marzo alle 5:00", "il 02/03 alle 5:00 di mattina", "il 02/03 alle 5:00 del mattino", "il 02/03 alle 5:00 della mattina");
    // FIXME - expressions or resolution issue?
    // fix_example!(v, check_moment!(c, [2013, 3, 2, 5, 0]), "2 marzo verso le 05:00", "il 2 marzo intorno alle 5:00", "2 marzo alle 5 circa");
    // FIXME: Also time-of-the-day issues
    example!(v, check_moment!(c, [2013, 3, 2, 5]), "il 2 alle 5 del mattino");
    // fix_example!(v, check_moment!(c, [2013, 3, 2, 5]), "il 2 alle 5h");
    example!(v, check_moment!(c, [2013, 2, 16, 18]), "il 16 alle 18", "il 16 alle 6 del pomeriggio", "il 16 alle 6 di pomeriggio", "sabato 16 alle 6 della sera");
    // fix_example!(v, check_moment!(c, [2013, 2, 16, 18]), "il 16 verso le 18:00", "il 16 tipo verso le 18", "il 16 intorno alle 18h", "il 16 verso le 6 di sera");
    example!(v, check_moment!(c, [2013, 2, 13, 11]), "domani alle ore 11");
    example!(v, check_moment!(c, [2013, 2, 14, 11]), "giovedì alle 11", "dopodomani alle ore 11");
    // Warning! 'alle ore 11' and '11:00' don't have same grain
    example!(v, check_moment!(c, [2013, 2, 14, 11, 0]), "dopo domani alle 11:00");
    example!(v, check_moment!(c, [2013, 2, 15, 12]), "venerdì a mezzogiorno", "venerdì alle 12");
    // fix_example!(v, check_moment!(c, [2013, 2, 15, 12]), "a mezzogiorno di venerdì");
    example!(v, check_moment!(c, [2013, 2, 15, 16]), "venerdì quindici alle sedici", "il venerdì quindici alle ore sedici", "venerdì quindici alle 16");
    example!(v, check_moment!(c, [2013, 2, 15, 16, 0]), "venerdì 15 alle 16:00");
    example!(v, check_moment!(c, [2013, 2, 13, 11, 0]), "mercoledì alle 11:00");

    // In + duration / duration + ago
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "tra un secondo", "fra un secondo", "tra un sec");
    // fix_example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "a un secondo da ora", "a un secondo da adesso");

    example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "tra un minuto", "fra un minuto");
    // fix_example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "a un minuto da ora", "a un minuto da adesso", "tra un min");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 0]), "tra 2 minuti", "fra due min");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "tra 60 minuti", "fra 60 minuti");
    // fix_example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "a sessanta minuti da adesso");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "tra un'ora", "fra un'ora");
    example!(v, check_moment!(c, [2013, 2, 12, 2, 30]), "due ore fa");
    example!(v, check_moment!(c, [2013, 2, 13, 4, 30]), "tra 24 ore", "fra venti quattro ore");
    example!(v, check_moment!(c, [2013, 2, 13]), "tra un giorno", "fra un giorno");
    example!(v, check_moment!(c, [2013, 2, 19]), "tra 7 giorni", "fra sette giorni");
    example!(v, check_moment!(c, [2013, 2, 19]), "tra 1 settimana", "fra una settimana");
    example!(v, check_moment!(c, [2013, 1, 22]), "tre settimane fa");
    example!(v, check_moment!(c, [2013, 4, 12]), "tra due mesi", "fra due mesi");
    example!(v, check_moment!(c, [2012, 11, 12]), "3 mesi fa");
    example!(v, check_moment!(c, [2014, 2]), "tra un anno", "fra un anno");
    example!(v, check_moment!(c, [2011, 2]), "due anni fa");

    // Seasons
    example!(v, check_moment_span!(c, [2013, 6, 21], [2013, 9, 24]), "quest'estate", "questa estate");
    example!(v, check_moment_span!(c, [2012, 12, 21], [2013, 3, 21]), "quest'inverno", "questo inverno");

    // Holidays
    // TODO
    // holiday_example!(v, check_moment!(c, [2013, 12, 25]), "Natale", "natale", "giorno di natale");
    // holiday_example!(v, check_moment_span!(c, [2013, 12, 24, 18], [2013, 12, 25, 00]), "la sera di natale", "la notte di Natale");
    // "il primo gennaio & co. works already"
    // holiday_example!(v, check_moment!(c, [2014, 1, 1]), "primo giorno dell'anno", "capodanno", "primo gennaio", "il primo gennaio", "il primo giorno di gennaio");
    // holiday_example!(v, check_moment!(c, [2013, 11, 1]), "tutti i santi", "il giorno di tutti i santi", "ognissanti", "il giorno di ognissanti", "il giorno d'ognissanti");
    // "il primo maggio & co. works already"
    // holiday_example!(v, check_moment!(c, [2013, 05, 1]), "festa del lavoro", "la festa dei lavoratori", "il primo maggio");

    // Part of day (morning, afternoon...)
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 19]), "questo pomeriggio", "il pomeriggio", "oggi pomeriggio");
    example!(v, check_moment_span!(c, [2013, 2, 12, 15], [2013, 2, 12, 17]), "nel mezzo del pomeriggio");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 15], [2013, 2, 12, 17]), "a metà pomeriggio", "a metà del pomeriggio", "alla metà del pomeriggio");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 9]), "la mattina presto", "la mattina sul presto", "di prima mattina", "di primo mattino", "nel mattinata sul presto");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 9]), "a inizio mattinata", "all'inizio della mattinata");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 9], [2013, 2, 12, 11]), "a metà mattina", "a metà mattinata", "a metà della mattina", "a metà della mattinata");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 10], [2013, 2, 12, 12]), "a fine mattina", "a fine mattinata", "alla fine della mattina", "alla fine della mattinata", "la mattina sul tardi");
    example!(v, check_moment_span!(c, [2013, 2, 12, 10], [2013, 2, 12, 12]), "nella tarda mattinata", "la seconda mattinata");
    example!(v, check_moment_span!(c, [2013, 2, 12, 13], [2013, 2, 12, 17]), "dopo pranzo");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 13], [2013, 2, 12, 17]), "dopo ora di pranzo");
    example!(v, check_moment_span!(c, [2013, 2, 12, 10], [2013, 2, 12, 12]), "prima di pranzo");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "durante il pranzo");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "all'ora di pranzo");
    example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 21]), "dopo il lavoro", "dopo l'orario di lavoro");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 15]), "nel primo pomeriggio", "il pomeriggio presto", "il pomeriggio sul presto");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 15]), "all'inizio del pomeriggio", "a inizio pomeriggio");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 19]), "nel secondo pomeriggio", "il pomeriggio tardi", "il pomeriggio sul tardi", "a fine pomeriggio", "alla fine del pomeriggio");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 6], [2013, 2, 12, 10]), "a inizio giornata", "all'inizio del giorno", "all'inizio della giornata");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 16]), "a metà giornata", "a metà della giornata");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 16]), "nel mezzo del giorno");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 21]), "a fine giornata", "alla fine della giornata", "alla fine del giorno");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 00]), "questa sera", "stasera", "in serata");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 12, 21]), "a inizio serata", "all'inizio della serata", "la sera presto");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 21], [2013, 2, 13, 00]), "a fine serata", "alla fine della serata", "la sera tardi");
    example!(v, check_moment_span!(c, [2013, 2, 18, 4], [2013, 2, 18, 12]), "lunedì mattina");
    example!(v, check_moment_span!(c, [2013, 2, 18, 12], [2013, 2, 18, 19]), "lunedì pomeriggio", "lunedì nel pomeriggio", "il pomeriggio di lunedi");
    example!(v, check_moment_span!(c, [2013, 2, 18, 17], [2013, 2, 18, 19]), "lunedì nel tardo pomeriggio", "lunedì nel secondo pomeriggio", "nel tardo pomeriggio di lunedì");
    example!(v, check_moment_span!(c, [2013, 2, 15, 4], [2013, 2, 15, 12]), "il 15 febbraio in mattinata", "la mattina del 15 febbraio", "il 15 febbraio di mattina");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "8 di stasera", "8 di sera", "ore otto della sera");
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "3 del mattino", "ore tre della mattina", "alle 3 di mattina");

    // Part of the week/month
    example!(v, check_moment_span!(c, [2013, 2, 13, 18], [2013, 2, 14, 00]), "domani sera", "mercoledì sera", "mercoledì in serata", "nella serata di mercoledì");
    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 00]), "ieri sera", "la sera di ieri");
    example!(v, check_moment_span!(c, [2013, 2, 15, 18], [2013, 2, 18, 00]), "questo week-end", "questo weekend", "questo week end", "questo fine settimana", "questo finesettimana", "questo fine-settimana");
    example!(v, check_moment_span!(c, [2013, 2, 11], [2013, 2, 13]), "a inizio settimana", "all'inizio della settimana", "all'inizio di questa settimana");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 15]), "a metà settimana", "a metà della settimana", "nel mezzo della settimana");
    example!(v, check_moment_span!(c, [2013, 2, 14], [2013, 2, 18]), "alla fine della settimana");
    example!(v, check_moment_span!(c, [2013, 2, 11], [2013, 2, 16]), "in settimana");
    example!(v, check_moment_span!(c, [2013, 2, 19], [2013, 3, 01]), "alla fine del mese", "a fine mese");
    // TODO "en inizio de settimana prochaine" interval (2013, 2, 18) (2013, 2, 20)
    example!(v, check_moment_span!(c, [2013, 9, 6, 18], [2013, 9, 9, 00]), "il primo week-end di settembre", "il primo weekend di settembre", "il primo fine-settimana del mese di settembre");
    example!(v, check_moment_span!(c, [2013, 9, 6, 18], [2013, 9, 9, 00]), "il primo week-end di settembre", "il primo weekend di settembre", "il primo fine-settimana del mese di settembre");
    example!(v, check_moment_span!(c, [2013, 9, 13, 18], [2013, 9, 16, 00]), "il secondo weekend di settembre");
    example!(v, check_moment_span!(c, [2013, 9, 27, 18], [2013, 9, 30, 00]), "l'ultimo fine settimana di settembre");

    // Intervals involving cycles
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "ultimi 2 secondi", "ultimi due secondi");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "i prossimi 3 secondi", "prossimi tre secondi");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "ultimi 2 minuti", "ultimi due minuti");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "nei prossimi 3 minuti", "prossimi tre minuti");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "le prossime 3 ore", "le 3 ore seguenti");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "ultimi 2 giorni", "ultimi due giorni", "negli scorsi due giorni");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "prossimi 3 giorni");
    // fix_example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11]), "ultime 2 settimane", "ultime due settimane", "scorse 2 settimane");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11]), "prossime tre settimane");
    // fix_example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "ultimi 2 mesi", "due mesi scorsi");
    // fix_example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "prossimi 3 mesi", "tre mesi seguenti");
    // fix_example!(v, check_moment_span!(c, [2011], [2013]), "ultimi 2 anni", "scorsi due anni");
    // fix_example!(v, check_moment_span!(c, [2014], [2017]), "prossimi 3 anni");

    // Explicit intervals
    example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "13-15 luglio", "dal 13 al 15 luglio", "dal 13 fino al 15 luglio", "dal 13 luglio al 15 luglio", "tra il 13 e il 15 luglio", "da sabato 13 a domenica 15 luglio");
    // fix_example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "13 luglio - 15 luglio", "dal sabato 13 alla domenica 15 di luglio", "dal 13 alla domenica 15 luglio");
    example!(v, check_moment_span!(c, [2013, 7, 1], [2013, 7, 11]), "dall'1 al 10 luglio", "da lunedì 1 a mercoledì 10 luglio", "dal lunedì 1 al mercoledì 10 luglio", "dal 1° luglio al mercoledì 10 luglio");
    // FIXME: Resolution issue
    // fix_example!(v, check_moment_span!(c, [2013, 7, 1], [2013, 7, 11]), "dal lunedì primo al mercoledì 10 di luglio");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 19]), "tra il 13 e il 18");
    // FIXME: Recognized but as time-of-day interval
    // fix_example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 19]), "dal 13 al 18", "fra il 13 e il 18");
    example!(v, check_moment_span!(c, [2023, 2, 1], [2023, 4, 1]), "tra febbraio e marzo due mila venti tre", "tra il febbraio e il marzo del due mila venti tre");
    example!(v, check_moment_span!(c, [2013, 6, 10], [2013, 7, 2]), "dal 10 giugno al primo luglio", "tra il 10 di giugno e l'uno di luglio", "dal 10 giugno al 1° luglio");
    // fix_example!(v, check_moment_span!(c, [2013, 6, 10], [2013, 7, 2]), "fra il 10 di giugno e il primo luglio");
    example!(v, check_moment_span!(c, [2017,4,6], [2017,6,9]), "dal sei aprile all'otto giugno due mila diciassette", "dal sei di aprile all'otto di giugno del due mila diciassette");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 14, 9, 30], [2013, 2, 14, 11]), "09:30 - 11:00 giovedì", "dalle 9:30 fino alle 11 giovedì", "dalle 9 e mezza alle 11 di giovedì", "giovedì dalle ore 9 e 30 alle ore 11", "giovedì tra le 9 e mezzo e le 11", "giovedì fra le 9h30 e le 11h", "giovedì tipo tra le 09:30 e le 11:00");
    example!(v, check_moment_with_direction!(c, [2013, 3, 8], Direction::After), "a partire dall'8");
    // fix_example!(v, check_moment_with_direction!(c, [2013, 3, 8], Direction::After), "dall'8 marzo", "dall'8 di marzo in poi");
    // todo_example!(v, check_moment_with_direction!(c, [2013, 2, 14, 9, 30], Direction::After), "a partire dalle 09:30 di giovedì", "giovedì dopo le 9 e 30", "giovedì mattina dalle 9 e mezza");
    // todo_example!(v, check_moment_with_direction!(c, [2013, 11, 1, 16], Direction::After), "dopo le 16 il primo novembre", "dopo le 16 del primo novembre", "l'1 novembre dopo le ore 16");
    // todo_example!(v, check_moment_with_direction!(c, [2013, 11, 1], Direction::After), "dopo il primo di novembre");
    // todo_example!(v, check_moment_with_direction!(c, [2013, 2, 12, 16], Direction::Before), "prima delle 16", "entro le 16:00", "fino alle ore 16");
    // todo_example!(v, check_moment_span!(c, [2013, 2, 13, 0], [2013, 2, 13, 16]), "domani fino alle 16", "domani prima delle 16:00", "entro le 16 di domani");
    // todo_example!(v, check_moment_with_direction!(c, [2013, 2, 20, 10], Direction::After), "il 20 a partire dalle 10", "il 20 dalle 10:00", "dalle ore 10 del 20");
    example!(v, check_moment_with_direction!(c, [2013, 2, 15, 12], Direction::After), "venerdì a partire da mezzogiorno");
    // fix_example!(v, check_moment_with_direction!(c, [2013, 2, 15, 12], Direction::After), "venerdì da mezzogiorno in poi", "dal mezzodì di venerdi");
    // todo_example!(v, check_moment_span!(c, [2013, 2, 20], [2013, 2, 20, 18]), "il 20 fino alle 18");
    example!(v, check_moment_span!(c, [2014, 9, 14], [2014, 9, 21]), "14 - 20 set. 2014", "14-20 sett 2014");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 26]), "entro 2 settimane");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 26]), "in due settimane");
    // 15j != 2 settimanas
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 5, 12]), "entro 3 mesi");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 5, 12]), "in tre mesi");
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 27]), "in 15 giorni", "nei prossimi 15 giorni");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 27]), "entro 15 giorni");
    // todo_example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 7]), "dalle 5 alle 7", "tra le 5 e le 7");
    // todo_example!(v, check_moment_span!(c, [2013, 2, 14, 9], [2013, 2, 14, 11]), "giovedì dalle 9 alle 11", "giovedì tra le 9 e le 11");
    // todo_example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "tra mezzogiorno e le 2", "fra mezzogiorno e le 2 di pomeriggio", "tra mezzogiorno e le quattordici");
    // todo_example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 30]), "11:30-13:30", "dalle 11:30 all'1 e trenta", "dalle 11 e mezza fino all'una e mezza");
    example!(v, check_moment!(c, [2013, 9, 21, 13, 30]), "alle 13:30 sabato 21 settembre", "alle 13:30 il sabato 21 settembre", "alle 13:30 del sabato 21 settembre");
    example!(v, check_moment_span!(c, [2013, 3, 25], [2013, 4, 1]), "fine marzo");
    // fix_example!(v, check_moment_span!(c, [2013, 3, 25], [2013, 4, 1]),//"alla fine del mese di marzo",//"a fine marzo",//"alla fine di marzo");
    example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 6]), "inizio aprile");
    // fix_example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 6]), "all'inizio del mese d'aprile", "all'inizio del mese di aprile", "all'inizio d'aprile", "all'inizio di aprile");
    example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 15]), "la prima quindicina d'aprile", "la prima quindicina di aprile");
    example!(v, check_moment_span!(c, [2013, 4, 15], [2013, 5, 01]), "la seconda quindicina d'aprile", "la seconda quindicina di aprile");
    // todo_example!(v, check_moment_span!(c, [2013, 12, 10], [2013, 12, 20]), "metà dicembre", "alla metà di dicembre", "alla metà del mese di dicembre");
    example!(v, check_moment!(c, [2013, 3]), "marzo", "a marzo", "in marzo", "il mese di marzo");
    // fix_example!(v, check_moment!(c, [2013, 3]), "nel mese di marzo",);
    example!(v, check_moment!(c, [2013, 2, 12, 4, 45, 0]), "tra un quarto d'ora", "fra 1/4 d'ora");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 0, 0]), "tra mezzora", "tra una mezzora", "fra 1/2 ora");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 15, 0]), "tra tre quarti d'ora", "fra 3/4 d'ora");
    example!(v, check_moment!(c, [2016, 12, 15]), "15.12.2016", "15.12.16");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 2]), "durante due ore", "per due ore");
    example!(v, check_duration!([0, 0, 0, 1]), "durante un giorno", "per una giornata", "durante una giornata", "per un giorno");
    example!(v, check_duration!([0, 1, 0]), "durante un mese", "per un mese");
    example!(v, check_duration!([1]), "durante un anno", "per un anno");
    example!(v, check_duration!([0, 0, 0, 0, 0, 1, 3]), "durante un minuto e tre secondi", "per un minuto e tre secondi");
    example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "verso un'ora e trenta", "circa un'ora e trenta", "all'incirca 1h30", "un'ora e mezza più o meno", "approssimativamente un'ora e mezzo");
    example!(v, check_duration!([0, 0, 0, 0, 0, 15], Precision::Approximate), "durante un quarto d'ora all'incirca", "durante circa un quarto d'ora", "per più o meno 1/4 d'ora");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "durante un'ora", "per un'ora");
    example!(v, check_duration!([0, 0, 2]), "durante 2 settimane", "per due settimane");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(1), "1", "un", "uno", "una", "un'");
    example!(v, check_integer(11), "undici");
    example!(v, check_integer(17), "diciassette");
    example!(v, check_integer(21), "vent uno");
    example!(v, check_integer(23), "venti tre", "venti tré");
    example!(v, check_integer(70), "settanta");
    example!(v, check_integer(78), "settant otto");
    example!(v, check_integer(73), "settanta tre", "settanta tré");
    example!(v, check_integer(80), "ottanta");
    example!(v, check_integer(81), "ottant uno");
    example!(v, check_integer(90), "novanta");
    example!(v, check_integer(91), "novant uno");
    example!(v, check_integer(99), "novanta nove");
    example!(v, check_integer(5000), "5 mila", "cinque mila");
    example!(v, check_integer(200000), "due cento mila");
    example!(v, check_integer(21011), "vent uno mila undici");
    example!(v, check_integer(721012), "sette cento vent uno mila dodici");
    example!(v, check_integer(31256721), "trent uno milioni due cento cinquanta sei mila sette cento vent uno", "trent uno milioni e due cento cinquanta sei mila sette cento vent uno");
    example!(v, check_integer(33), "33", "trenta tre", "trenta tré");
    example!(v, check_integer(100000), "100.000", "100000", "100K", "100k");
    example!(v, check_integer(3000000), "3M", "3000K", "3000000", "3.000.000");
    example!(v, check_integer(1200000), "1.200.000", "1200000", "1,2M", "1200K", ",0012G");
    example!(v, check_integer(-1200000), "- 1.200.000", "-1200000", "meno 1200000", "-1,2M", "-1200K", "-,0012G");
    example!(v, check_float(1.1), "1,1", "1,10", "01,10", "uno punto uno", "uno virgola uno", "uno punto dieci", "uno virgola dieci");
    example!(v, check_float(0.5), "0,5", "0,50", "zero punto cinque", "zero virgola cinque", "zero punto cinquanta", "zero virgola cinquanta");
    example!(v, check_float(32.75), "32,75", "trenta due punto settanta cinque", "trenta due virgola settanta cinque");
    example!(v, check_float(10.08), "10,08", "dieci punto zero otto");//, "dieci virgola zero otto");
    example!(v, check_ordinal(1), "1o", "1a", "il 1o", "la 1a", "1°");
    example!(v, check_ordinal(3), "3o", "il 3o", "3a", "la 3a", "3°", "terzo", "terza");
}

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "0,3%", "zero virgola tre per cento", "zero virgola tre percento");
    example!(v, check_percentage(15.0), "15%", "quindici per cento", "quindici percento");
    example!(v, check_percentage(355.0), "355 %", "355 per cento", "355 percento", "tre cento cinquanta cinque per cento", "tre cento cinquanta cinque percento");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "tre gradi", "3 gradi", "3°", "3 °");
    example!(v, check_temperature(32.0, Some("celsius")), "trenta due gradi celsius", "trenta due gradi centigradi", "32°C", "32° C", "32° c", "32°c", "32 °c");
    example!(v, check_temperature(-27.0, Some("celsius")), "meno 27 celsius", "meno 27 gradi celsius", "meno venti sette gradi celsius", "-27C", "-27°C", "-27° C", "-27°c", "-27° c", "- 27 c");
    example!(v, check_temperature(-5.0, Some("fahrenheit")), "meno cinque gradi fahrenheit", "-5 °F", "-5°F", "-5°f", "-5° f", "- 5°f");
    example!(v, check_temperature(168.0, Some("fahrenheit")), "cento sessant otto fahrenheit", "cento sessant otto gradi fahrenheit", "168 gradi fahrenheit", "168° F", "168 F", "168f");
    example!(v, check_temperature(10.0, Some("kelvin")), "dieci gradi kelvin", "dieci kelvin", "10°K", "10° K", "10 °K", "10°k");
    example!(v, check_temperature(21.0, Some("kelvin")), "21 kelvin", "vent uno gradi kelvin", "21°K", "21° K", "21 °K", "21°k", "21 K", "21k");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(800.0, Some("$"), Precision::Exact), "800 $", "800$", "otto cento dollari");
    example!(v, check_finance(10.0, Some("$"), Precision::Approximate), "circa dieci dollari", "dieci dollari circa");
    example!(v, check_finance(10.0, Some("USD"), Precision::Approximate), "circa dieci dollari americani", "più o meno 10 USD", "quasi 10US$");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "tre dollari australiani");
    example!(v, check_finance(3.5, Some("AUD"), Precision::Exact), "3,5 dollari australiani", "tre dollari australiani e cinquanta centesimi");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "zero dollari di hong kong");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "125 CAD", "cento venti cinque dollari canadesi");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact), "45€", "45 €", "quaranta cinque euro");
    example!(v, check_finance(2.05, Some("EUR"), Precision::Exact), "2,05 €", "2,05€", "due euro e cinque centesimi");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "2 £", "2£", "due lire");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "venti sterline", "venti sterline britanniche", "venti lire sterline", "venti sterline inglesi");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "38 franchi svizzeri", "trent otto franchi svizzeri");
    example!(v, check_finance(447.0, Some("KR"), Precision::Exact), "447 corone", "quattro cento quaranta sette corone");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "dieci mila DKK", "10.000 corone danesi");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact), "100 corone norvegesi", "cento corone norvegesi");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "2005 corone svedesi", "due mila cinque corone svedesi");
    example!(v, check_finance(96.0, Some("INR"), Precision::Approximate), "approssimativamente 96 rupie", "novanta sei rupie all'incirca");
    example!(v, check_finance(5.3, Some("RUB"), Precision::Exact), "cinque rubli e trenta", "5,3 rubli");
    example!(v, check_finance(89.0, Some("JPY"), Precision::Exact), "esattamente 89 JPY", "89 yen esatti", "precisamente ottanta nove yen giapponesi");
    example!(v, check_finance(8.0, Some("¥"), Precision::Exact), "8¥");
    example!(v, check_finance(100.0, Some("CNY"), Precision::Exact), "cento yuan esatti", "esattamente 100 yuan cinesi", "100 renminbi precisi");
    example!(v, check_finance(7.0, Some("KRW"), Precision::Exact), "7 won", "7₩", "sette won sudcoreani");
    example!(v, check_finance(3.0, Some("฿"), Precision::Exact), "3฿", "3 ฿", "tre bitcoin", "tre bitcoins");
    // TODO: FIXME - add approx. numbers in IT rules
    // todo_example!(v, check_finance(15.0, Some("$"), Precision::Approximate), "una quindicina di dollari", "una 15ina di dollari");
    example!(v, check_finance(3000000.0, Some("EUR"), Precision::Exact), "tre milioni di euro");
}