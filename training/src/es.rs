use super::*;
use rustling_ontology_values::check::*;

pub fn examples_time(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ParsingContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second), 80);
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 00]), "ahora", "ya", "ahorita", "cuanto antes");
    example!(v, check_moment!(c, [2013, 2, 12]), "hoy", "en este momento");
    example!(v, check_moment!(c, [2013, 2, 11]), "ayer");
    example!(v, check_moment!(c, [2013, 2, 10]), "anteayer", "antier");
    example!(v, check_moment!(c, [2013, 2, 13]), "mañana");
    example!(v, check_moment!(c, [2013, 2, 14]), "pasado mañana");
    example!(v, check_moment!(c, [2013, 2, 18]), "lunes", "lu", "lun.", "este lunes");
    example!(v, check_moment!(c, [2013, 2, 18]), "lunes, 18 de febrero");
    example!(v, check_moment!(c, [2013, 2, 19]), "martes", "ma", "ma.");
    example!(v, check_moment!(c, [2013, 2, 13]), "miercoles", "miércoles", "mx", "mié.");
    example!(v, check_moment!(c, [2013, 2, 14]), "jueves");
    example!(v, check_moment!(c, [2013, 2, 15]), "viernes");
    example!(v, check_moment!(c, [2013, 2, 16]), "sabado");
    example!(v, check_moment!(c, [2013, 2, 17]), "domingo");
    example!(v, check_moment!(c, [2013, 5, 5]), "el 5 de mayo", "el cinco de mayo");
    // "mayo 5 del 2013" in part of latin america
    example!(v, check_moment!(c, [2013, 5, 5]), "el cinco de mayo de 2013", "mayo 5 del 2013", "5-5-2013");
    example!(v, check_moment!(c, [2013, 7, 4]), "el 4 de julio", "el 4/7");
    example!(v, check_moment!(c, [2013, 3, 3]), "el 3 de marzo", "3 de marzo", "el 3-3");
    example!(v, check_moment!(c, [2013, 4, 5]), "el 5 de abril", "5 de abril");
    example!(v, check_moment!(c, [2013, 3, 1]), "el 1 de marzo", "1 de marzo", "el primero de marzo", "el uno de marzo", "primero de marzo", "uno de marzo");
    example!(v, check_moment!(c, [2013, 3, 1]), "1-3-2013", "1.3.2013", "1/3/2013");
    example!(v, check_moment!(c, [2013, 2, 16]), "el 16", "16 de febrero");
    example!(v, check_moment!(c, [2013, 2, 17]), "el 17", "17 de febrero", "17-2", "el 17/2");
    example!(v, check_moment!(c, [2013, 2, 20]), "el 20", "20 de febrero", "20/2");
    //"31/10/74"  smart two-digit year resolution
    example!(v, check_moment!(c, [1974, 10, 31]), "31/10/1974", "31/10/74");
    //when today is Tuesday, "mardi prochain" is a week from now
    example!(v, check_moment!(c, [2013, 2, 19]), "el martes que viene");
    //when today is Tuesday, "mercredi prochain" is tomorrow
    example!(v, check_moment!(c, [2013, 2, 20]), "miércoles que viene", "el miércoles de la semana que viene", "miercoles de la próxima semana");
    example!(v, check_moment!(c, [2013, 2, 11]), "el lunes de esta semana");
    example!(v, check_moment!(c, [2013, 2, 12]), "martes de esta semana");
    example!(v, check_moment!(c, [2013, 2, 13]), "el miércoles de esta semana");
    //Cycles
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "esta semana");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "la semana pasada");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "la semana que viene", "la proxima semana");
    example!(v, check_moment!(c, [2013, 1]), "el pasado mes");
    example!(v, check_moment!(c, [2013, 3]), "el mes que viene", "el proximo mes");
    example!(v, check_moment!(c, [2012]), "el año pasado");
    example!(v, check_moment!(c, [2013]), "este ano");
    example!(v, check_moment!(c, [2014]), "el año que viene", "el proximo ano");
    example!(v, check_moment!(c, [2013, 2, 10]), "el domingo pasado", "el domingo de la semana pasada");
    example!(v, check_moment!(c, [2013, 2, 5]), "el martes pasado");
    // Hours
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "a las tres de la tarde", "a las tres", "a las 3 pm", "a las 15 horas");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "a las ocho de la tarde");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 0]), "15:00", "15.00");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 0, 10]), "15:00.10");
    example!(v, check_moment!(c, [2013, 2, 13, 00]), "medianoche");
    example!(v, check_moment!(c, [2013, 2, 12, 12]), "mediodía", "las doce");
    example!(v, check_moment!(c, [2013, 2, 12, 12, 15]), "las doce y cuarto");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 55]), "las doce menos cinco");
    example!(v, check_moment!(c, [2013, 2, 12, 12, 30]), "las doce y media");
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "las tres de la manana", "las tres en la manana");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "a las tres y quince", "a las 3 y cuarto", "a las tres y cuarto de la tarde", "a las tres y cuarto en la tarde", "15:15", "15.15");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "a las tres y media", "a las 3 y treinta", "a las tres y media de la tarde", "a las 3 y treinta del mediodía", "15:30", "15.30");
    //"hoy a 11:45"
    example!(v, check_moment!(c, [2013, 2, 12, 11, 45]), "las doce menos cuarto", "11:45", "las once y cuarenta y cinco", "hoy a las doce menos cuarto", "hoy a las once y cuarenta y cinco");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 15]), "5 y cuarto");
    example!(v, check_moment!(c, [2013, 2, 12, 6]), "6 de la mañana");
    example!(v, check_moment!(c, [2013, 2, 13, 11]), "miércoles a las once de la mañana");
    example!(v, check_moment!(c, [2013, 2, 13, 11]), "mañana a las once", "mañana a 11");
    example!(v, check_moment!(c, [2014, 9, 12]), "viernes, el 12 de septiembre de 2014");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "en un segundo");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "en un minuto", "en 1 min");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 0]), "en 2 minutos", "en dos minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "en 60 minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "en una hora");
    example!(v, check_moment!(c, [2013, 2, 12, 2, 30]), "hace dos horas");
    example!(v, check_moment!(c, [2013, 2, 13, 4, 30]), "en 24 horas", "en veinticuatro horas");
    example!(v, check_moment!(c, [2013, 2, 13, 4]), "en un dia");
    example!(v, check_moment!(c, [2013, 2, 19, 4]), "en 7 dias");
    example!(v, check_moment!(c, [2013, 2, 19]), "en una semana");
    example!(v, check_moment!(c, [2013, 1, 22]), "hace tres semanas");
    example!(v, check_moment!(c, [2013, 4, 12]), "en dos meses");
    example!(v, check_moment!(c, [2012, 11, 12]), "hace tres meses");
    example!(v, check_moment!(c, [2014, 2]), "en un ano", "en 1 año");
    example!(v, check_moment!(c, [2011, 2]), "hace dos años");
    //Seasons
    example!(v, check_moment_span!(c, [2013, 6, 21], [2013, 9, 24]), "este verano");
    example!(v, check_moment_span!(c, [2012, 12, 21], [2013, 3, 21]), "este invierno");
    //ES holidays
    example!(v, check_moment!(c, [2013, 12, 25]), "Navidad", "la Navidad");
    example!(v, check_moment!(c, [2013, 12, 31]), "Nochevieja");
    example!(v, check_moment!(c, [2014, 1, 1]), "ano nuevo", "año nuevo");
    //Part of day (morning, afternoon...)
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 00]), "esta noche");
    //"miércoles por la noche"
    example!(v, check_moment_span!(c, [2013, 2, 13, 18], [2013, 2, 14, 00]), "mañana por la noche");
    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 00]), "ayer por la noche");
    example!(v, check_moment_span!(c, [2013, 2, 15, 18], [2013, 2, 18, 00]), "este weekend", "este fin de semana");
    example!(v, check_moment_span!(c, [2013, 2, 18, 4], [2013, 2, 18, 12]), "lunes por la mañana");
    example!(v, check_moment_span!(c, [2013, 2, 15, 4], [2013, 2, 15, 12]), "el 15 de febrero por la mañana");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "a las 8 de la tarde");
    //Intervals involving cycles
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "pasados 2 segundos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "proximos 3 segundos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "pasados 2 minutos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "proximos 3 minutos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "proximas 3 horas");
    example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "pasados 2 dias");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "proximos 3 dias");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11]), "pasadas dos semanas");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11]), "3 proximas semanas", "3 semanas que vienen");
    example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "pasados 2 meses", "dos pasados meses");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "3 próximos meses", "proximos tres meses", "tres meses que vienen");
    example!(v, check_moment_span!(c, [2011], [2013]), "pasados 2 anos", "dos pasados años");
    example!(v, check_moment_span!(c, [2014], [2017]), "3 próximos años", "proximo tres años", "3 años que vienen");
    //Explicit intervals
    example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "13 a 15 de julio", "13 - 15 de julio de 2013");
    example!(v, check_moment_span!(c, [2013, 2, 12, 9, 30], [2013, 2, 12, 11]), "9:30 - 11:00");
    example!(v, check_moment_span!(c, [2013, 12, 21], [2014, 1, 7]), "21 de Dic. a 6 de Ene");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 7, 30]), "dentro de tres horas");
    example!(v, check_moment!(c, [2016, 12, 15]), "15.12.2016", "15.12.16");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(1), "1", "uno", "una");
    example!(v, check_integer(11), "once");
    example!(v,
             check_integer(16),
             "dieciséis",
             "dieciseis",
             "diesiseis",
             "diez y seis");
    example!(v, check_integer(21), "veintiuno", "veinte y uno");
    example!(v, check_integer(23), "veintitrés", "veinte y tres");
    example!(v, check_integer(70), "setenta");
    example!(v, check_integer(78), "setenta y ocho");
    example!(v, check_integer(80), "ochenta");
    example!(v, check_integer(33), "33", "treinta y tres", "treinta y 3");
    example!(v, check_float(1.1), "1,1", "1,10", "01,10");
    example!(v, check_float(0.77), "0,77", ",77");
    example!(v,
             check_integer(100000),
             "100.000",
             "100000",
             "100K",
             "100k");
    example!(v, check_integer(300), "trescientos");
    example!(v, check_integer(243), "243");
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
             "menos 1.200.000",
             "-1,2M",
             "-1200K",
             "-,0012G");
    example!(v,
             check_float(1.5),
             "1 punto cinco",
             "una punto cinco",
             "1,5");
}
