use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(1), "1", "um", "uma");
    example!(v, check_integer(11), "onze");
    example!(v, check_integer(17), "dezessete");
    example!(v, check_integer(21), "vinte e um");
    example!(v, check_integer(23), "vinte e três");
    example!(v, check_integer(70), "setenta");
    example!(v, check_integer(78), "setenta e oito");
    example!(v, check_integer(73), "setenta e três");
    example!(v, check_integer(80), "oitenta");
    example!(v, check_integer(81), "oitenta e um");
    example!(v, check_integer(90), "noventa");
    example!(v, check_integer(91), "noventa e um");
    example!(v, check_integer(99), "noventa e nove");
    example!(v, check_integer(5000), "cinco mil");
    example!(v, check_integer(200000), "duzentos mil");
    example!(v, check_integer(21011), "vinte e um mil e onze");
    example!(v, check_integer(721012), "setecentos e vinte e um mil e doze");
    example!(v, check_integer(31256721), "trinta e um milhões duzentos e cinquenta e seis mil setecentos e vinte e um");
    example!(v, check_integer(33), "33", "trinta e três");
    example!(v, check_integer(100000), "100.000", "100000", "100K", "100k", "cem mil");
    // TODO: Check if want/need support for M = 1000000 ?
    //example!(v, check_integer(3000000), "3M", "3000000", "3.000.000", "três milhões");
    //example!(v, check_integer(1200000), "1.200.000", "1200000", "1,2M", "um milhão e duzentos mil");
    //example!(v, check_integer(-1200000), "- 1.200.000", "-1200000", "menos 1200000", "-1,2M", "menos um milhão e duzentos mil");
    // TODO : float temperature
    //example!(v, check_float(1.1), "1,1", "1,10", "um vígula um", "um vírgula dez");
    example!(v, check_float(0.5), "0,5", "0,50", "zero vírgula cinco", "zero vírgula cinquenta");
    example!(v, check_float(32.75), "32,75", "trinta e dois vírgula setenta e cinco");
    example!(v, check_float(10.08), "10,08", "dez vírgula zero oito");
    // TODO: Check if want/need support for ordinal special character/overscript
    example!(v, check_ordinal(1), "1o", "1a", "primeiro", "primeira", "1º", "1ª");
    example!(v, check_ordinal(3), "3o", "3a", "3º", "3ª", "terceiro", "terceira");
    example!(v, check_ordinal(2), "segundo", "2º", "2o");
    example!(v, check_ordinal(5), "quinto", "5º", "5o");
}

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "0,3%", "zero vírgula três por cento");
    example!(v, check_percentage(15.0), "15%", "quinze por cento");
    example!(v, check_percentage(355.0), "355 %", "355 por cento", "trezentos e cinquenta e cinco por cento");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "três graus", "3 graus");
    example!(v, check_temperature(32.0, Some("celsius")), "trinta e dois graus celsius", "trinta e dois graus centígrados", "32°C", "32°c");
    example!(v, check_temperature(-27.0, Some("degree")), "menos 27 graus", "27 graus abaixo de zero","menos vinte e sete graus");
    example!(v, check_temperature(-27.0, Some("celsius")), "menos 27 graus celsius", "menos 27 graus centígrados", "-27°C", "-27°c");
    example!(v, check_temperature(-5.0, Some("fahrenheit")), "menos cinco graus fahrenheit", "cinco graus fahrenheit abaixo de zero", "-5°F", "-5°f");
    example!(v, check_temperature(168.0, Some("fahrenheit")), "cento e sessenta e oito fahrenheit", "cento e sessenta e oito graus fahrenheit", "168 graus fahrenheit", "168°F", "168°f");
    example!(v, check_temperature(10.0, Some("kelvin")), "dez kelvin", "10K", "10k", "dez graus kelvin");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    // TODO: Ask Drica for US$800
    //example!(v, check_finance(800.0, Some("$"), Precision::Exact), "US$800", "$800", "oitocentos dólares");
    example!(v, check_finance(10.0, Some("$"), Precision::Approximate), "uns dez dólares", "dez dólares mais ou menos");
    // TODO: Ask Drica for US$10
    //example!(v, check_finance(10.0, Some("USD"), Precision::Approximate), "uns dez dólares americanos", "US$10 mais ou menos", "quase $10", "uns 10 USD");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "três dólares australianos");
    example!(v, check_finance(3.5, Some("AUD"), Precision::Exact), "3,5 dólares australianos", "três dólares australianos e cinquenta centavos");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "zero dólares de hong kong");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "125 CAD", "cento e vinte e cinco dólares canadenses");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact), "45€", "quarenta e cinco euros");
    // TODO: Support money amounts with cents dois vírgula cinco euros =/= 2.5
    //example!(v, check_finance(2.05, Some("EUR"), Precision::Exact), "2,05€", "dois vírgula cinco euros");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "2£", "duas libras");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "vinte libras esterlinas");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "38 francos suiços", "trinta e oito francos suiços");
    example!(v, check_finance(447.0, Some("KR"), Precision::Exact), "447 coroas", "quatrocentos e quarenta e sete coroas");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "dez mil DKK", "10.000 coroas dinamarquesas");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact), "100 coroas norueguesas", "cem coroas norueguesas", "100NOK");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "2005 coroas suecas", "duas mil e cinco coroas suecas");
    example!(v, check_finance(96.0, Some("INR"), Precision::Approximate), "aproximadamente 96 rúpias", "umas noventa e seis rúpias");
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

    // Days
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 00]), "agora", "agora mesmo", "neste exato momento", "neste momento");
    example!(v, check_moment!(c, [2013, 2, 12]), "hoje");
    //example!(v, check_moment!(c, [2013, 2, 11]), "ontem", "no dia anterior", "na véspera", "um dia antes");
    //example!(v, check_moment!(c, [2013, 2, 10]), "antes de ontem", "anteontem");
    // ????
    //example!(v, check_moment!(c, [2013, 2, 13]), "amanhã", "no dia seguinte", "um dia depois", "no dia depois");

    // TODO: depois de amanha / after(tomorow)
    //example!(v, check_moment!(c, [2013, 2, 14]), "depois de amanhã");
    // same type of ambiguity as below
    //example!(v, check_moment!(c, [2013, 2, 18]), "segunda-feira", "segunda", "esta segunda", "seg.", "2ª", "2ª feira");
    example!(v, check_moment!(c, [2013, 2, 18]), "segunda-feira 18 de fevereiro", "na segunda-feira dia 18 de fevereiro", "na segunda-feira dia dezoito de fevereiro", "na segunda dezoito de fevereiro");
    // same type of ambiguity as below
    //example!(v, check_moment!(c, [2013, 2, 19]), "terça-feira", "terça", "3ª feira", "ter.", "esta terça");
    example!(v, check_moment!(c, [2013, 2, 13]), "quarta-feira 13 de fevereiro", "na 4ª feira dia 13 de fevereiro", "na quarta dia treze de fevereiro", "na 4ª dia treze de fevereiro");
    // same type of ambiguity as below
    //example!(v, check_moment!(c, [2013, 2, 14]), "quinta-feira", "dois dias depois", "dois dias mais tarde", "quinta", "5ª feira");
    // sexta: Ambiguity sixth(ordinal) and sixth-feira
    //example!(v, check_moment!(c, [2013, 2, 15]), "sexta-feira", "sexta", "6ª feira");
    example!(v, check_moment!(c, [2013, 2, 16]), "sábado", "no sábado", "sáb.");
    example!(v, check_moment!(c, [2013, 2, 17]), "domingo", "no domingo", "dom.");

    example!(v, check_moment!(c, [2013, 3, 1]), "dia primeiro de março", "no dia primeiro de março", "1º de março", "6ª feira dia primeiro de março", "na sexta-feira dia primeiro de março");
    example!(v, check_moment!(c, [2013, 3, 1]), "no dia primeiro de março de 2013", "no dia 1 de março de 2013", "em 1º de março de 2013", "em 1 de março do ano de 2013", "1/3/2013", "01/03/2013", "01/03/13", "01-03-2013");
    example!(v, check_moment!(c, [2013, 3, 2]), "no dia 2 de março", "2 de março", "dois de março", "em 2/3", "no 02/03");
    example!(v, check_moment!(c, [2013, 3, 2]), "no dia 2", "dia 2");
    example!(v, check_moment!(c, [2013, 3, 3]), "no dia 3 de março", "dia três de março", "3 de março", "no 3/3", "em 03/03");
    example!(v, check_moment!(c, [2013, 4, 5]), "no dia 5 de abril", "5 de abril", "dia 5 de abril");
    example!(v, check_moment!(c, [2015, 3, 3]), "no dia 3 de março de 2015", "em 3 de março de 2015", "3 de março de 2015", "3/3/2015", "03/03/2015", "03/03/15", "3-3-2015");
    example!(v, check_moment!(c, [2013, 2, 15]), "no dia 15 de fevereiro", "15 de fevereiro", "quinze de fevereiro");
    // TODO: fev, fev. not supported for fevereiro
    //example!(v, check_moment!(c, [2013, 2, 15]), "15/02/2013", "15/02/13", "15 fev. 2013", "15 fev 2013", "15 de fevereiro de 2013", "quinze de fevereiro de 2013");
    example!(v, check_moment!(c, [2013, 2, 16]), "no dia 16", "dia dezesseis");
    example!(v, check_moment!(c, [2013, 2, 17]), "17 de fevereiro", "no dia 17 de fevereiro", "17/2", "17/02", "no 17/02");
    // TODO: f. for feira not supported
    //example!(v, check_moment!(c, [2013, 2, 13]), "quarta-feira dia 13", "quarta dia 13", "4ª f. dia 13", "quarta-feira dia treze");
    example!(v, check_moment!(c, [2014, 2, 20]), "20/02/2014", "20/2/2014", "20/02/14", "20.02.2014", "20.02.14", "20-02-2014", "20-2-2014", "20-02-14");
    example!(v, check_moment!(c, [2013, 10, 31]), "31 de outubro", "no 31 de outubro");
    example!(v, check_moment!(c, [2014, 2, 20]), "20/02/2014", "20/2/2014", "20/02/14", "20.02.2014", "20.02.14", "20-02-2014", "20-2-2014", "20-02-14");
    example!(v, check_moment!(c, [1974, 10, 31]), "31/10/1974", "31/10/74");
    // when today is Tuesday, "segunda que vem" is a week from now
    example!(v, check_moment!(c, [2013, 2, 18]), "na próxima segunda", "segunda que vem", "na segunda-feira da semana que vem", "2ª feira da semana que vem");

    //example!(v, check_moment!(c, [2013, 2, 19]), "na terça-feira que vem", "na próxima terça", "na 3ª feira que vem", "na terça da semana que vem", "na terça da próxima semana");
    //example!(v, check_moment!(c, [2013, 2, 13]), "na quarta-feira que vem", "na próxima quarta", "na 4ª feira da próxima semana");
    //example!(v, check_moment!(c, [2013, 2, 20]),"na quarta-feira da semana que vem", "na quarta-feira da próxima semana", "4ª feira da próxima semana");
    //example!(v, check_moment!(c, [2013, 2, 11]), "segunda-feira desta semana", "na segunda desta semana", "nesta segunda");
    //example!(v, check_moment!(c, [2013, 2, 12]), "terça-feira desta semana", "na terça desta semana", "na 3ª feira desta semana");
    //example!(v, check_moment!(c, [2013, 2, 13]), "quarta-feira desta semana", "quarta desta semana", "4ª feira desta semana");
    // resolution: date period
    //example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "esta semana");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "a semana passada", "a última semana");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "a próxima semana", "a semana que vem");
    example!(v, check_moment!(c, [2013, 1]), "o mês passado", "o último mês");
    example!(v, check_moment!(c, [2013, 3]), "o mês que vem", "o próximo mês");
    example!(v, check_moment!(c, [2012]), "o ano passado", "o último ano");
    example!(v, check_moment!(c, [2013]), "este ano");
    example!(v, check_moment!(c, [2014]), "o ano que vem", "o próximo ano");

    //example!(v, check_moment!(c, [2013, 2, 10]), "o domingo passado", "o domingo da semana passada", "o último domingo");
    example!(v, check_moment!(c, [2013, 10, 3]), "o terceiro dia de outubro", "o 3º dia de outubro", "o 3º dia de outubro", "dia 3 de outubro");
    //example!(v, check_moment!(c, [2014, 10, 6], Grain::Week), "primeira semana de outubro de 2014", "a primeira semana de outubro de 2014");
    //example!(v, check_moment!(c, [2013, 10, 7], Grain::Week), "a semana do dia 7 de outubro", "a semana de sete de outubro");
    // fix_example!(v, check_moment!(c, [2015, 10, 31]), "último dia de outubro de 2015", "o último dia de outubro de 2015");
    // fix_example!(v, check_moment!(c, [2014, 9, 22], Grain::Week), "última semana de setembro de 2014", "a última semana de setembro de 2014", "a última semana do mês de setembro de 2014", "a última semana de setembro do ano de 2014");

    // Day times
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "às quinze horas", "às três da tarde");
    // Ambiguity Duration // Time
    //example!(v, check_moment!(c, [2013, 2, 12, 15, 0]), "15:00", "15h");
    // Ambiguity Duration // Time
    //example!(v, check_moment!(c, [2013, 2, 13, 00]), "meia-noite", "zero hora");
    // FIXME: PROBLEM 12 horas
    //example!(v, check_moment!(c, [2013, 2, 12, 12]), "meio-dia", "hoje ao meio-dia", "às doze horas");
    example!(v, check_moment!(c, [2013, 2, 12, 12, 15]), "ao meio-dia e quinze", "às doze horas e quinze minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 55]), "cinco para o meio-dia", "às onze e cinquenta e cinco", "às onze horas e cinquenta e cinco minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 12, 30]), "ao meio-dia e meia", "às doze horas e trinta minutos", "às doze e trinta");

    example!(v, check_moment!(c, [2013, 2, 13, 00, 03]), "à zero hora e três minutos");
    //Is today at midnight 13/12 or 12/12 ?
    //example!(v, check_moment!(c, [2013, 2, 13, 00, 03]), "hoje à meia-noite e três");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "às quinze e quinze", "às quinze horas e quinze minutos", "às três e quinze da tarde", "15:15", "15h15");
    example!(v, check_moment!(c, [2013, 2, 13, 15, 15]), "amanhã às três e quinze da tarde");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "às quinze e trinta", "às quinze horas e trinta minutos", "às três e meia da tarde", "15:30", "15h30");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 45]), "às quinze para o meio-dia", "11:45", "às onze horas e quarenta e cinco minutos", "hoje às 11h45");

    // Day + day time
    example!(v, check_moment!(c, [2013, 3, 2, 5, 0]), "2 de março às 5:00");
    example!(v, check_moment!(c, [2013, 3, 2, 5]), "2 de março às 5h", "no dia 2 de março às 5h", "em 02/03 às 5h da manhã", "em 02/03 às 5h da manhã");
    example!(v, check_moment!(c, [2013, 3, 2, 5]), "2 de março lá pelas 5h", "no dia 2 de março por volta das 5h da manhã", "2 de março lá pelas cinco horas mais ou menos");
    example!(v, check_moment!(c, [2013, 3, 2, 5]), "no dia 2 às cinco", "dia dois às cinco horas", "no dia dois às cinco da manhã", "no dia 02 às cinco horas da manhã", "no dia 02 às cinco horas da madrugada");
    example!(v, check_moment!(c, [2013, 2, 16, 6]), "no dia 16 às 6 da manhã","no dia 16 às 6 da madrugada");
    example!(v, check_moment!(c, [2013, 2, 16, 18]), "dia 16 às 18h", "no sábado dia 16 às 6 da tarde");
    example!(v, check_moment!(c, [2013, 2, 13, 11]), "amanhã às 11 horas", "amanhã às onze horas", "amanhã às onze");
    // TODO: Ambiguity ? "depois de amanhã às 11h" -> After((tomorrow, 11h) or -> (the day after tomorrow , 11h) + Is Grain OK ?
    //example!(v, check_moment!(c, [2013, 2, 14, 11]), "quinta-feira às 11h", "depois de amanhã às 11h");
    // Warning! Grain
    example!(v, check_moment!(c, [2013, 2, 14, 11, 0]), "o dia depois de amanhã às 11:00");
    example!(v, check_moment!(c, [2013, 2, 15, 12]), "na sexta-feira às 12h", "sexta-feira ao meio-dia", "ao meio-dia da sexta-feira");
    example!(v, check_moment!(c, [2013, 2, 15, 16]), "sexta-feira dia quinze às dezesseis horas", "na sexta-feira dia quinze às dezesseis horas", "na sexta-feira dia 15 às 16h", "na sexta dia quinze às quatro da tarde");
    example!(v, check_moment!(c, [2013, 2, 15, 16, 0]), "sexta-feira dia 15 às 16h00");
    example!(v, check_moment!(c, [2013, 2, 13, 11]), "quarta-feira às 11h", "quarta-feira às 11 da manhã");

    // In + duration / duration + ago≤
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "dentro de um segundo", "em um segundo");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "dentro de um minuto", "em um minuto");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 0]), "dentro de 2 minutos", "em 2 minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "dentro de 60 minutos", "em 60 minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "dentro de uma hora", "em uma hora");
    example!(v, check_moment!(c, [2013, 2, 13, 4, 30]), "dentro de 24 horas", "dentro de vinte e quatro horas");
    example!(v, check_moment!(c, [2013, 2, 13]), "dentro de um dia", "em um dia", "dentro de um dia");
    example!(v, check_moment!(c, [2013, 2, 19]), "dentro de sete dias", "em sete dias");
    example!(v, check_moment!(c, [2013, 2, 19]), "dentro de uma semana");
    example!(v, check_moment!(c, [2013, 4, 12]), "dentro de dois meses", "em dois meses");
    example!(v, check_moment!(c, [2014, 2]), "dentro de um ano", "em um ano");
    example!(v, check_moment!(c, [2013, 2, 12, 2, 30]), "faz duas horas", "há duas horas");
    example!(v, check_moment!(c, [2013, 1, 22]), "faz três semanas", "há três semanas");
    example!(v, check_moment!(c, [2012, 11, 12]), "há três meses", "faz três meses");
    example!(v, check_moment!(c, [2011, 2]), "faz dois anos", "há dois anos");

    // Seasons
    example!(v, check_moment_span!(c, [2013, 6, 21], [2013, 9, 24]), "este verão");
    example!(v, check_moment_span!(c, [2012, 12, 21], [2013, 3, 21]), "este inverno");

    // Holidays
    example!(v, check_moment!(c, [2013, 12, 25]), "Natal", "dia de natal");
    example!(v, check_moment_span!(c, [2013, 12, 24, 18], [2013, 12, 25, 00]), "véspera de natal", "a noite de natal");
    example!(v, check_moment!(c, [2014, 1, 1]), "primeiro dia do ano", "ano novo", "primeiro de janeiro", "no dia primeiro de janeiro", "no primeiro dia de janeiro");
    example!(v, check_moment!(c, [2013, 11, 1]), "todos os santos", "no dia de todos os santos", "dia de finados", "dia de todos os santos");
    example!(v, check_moment!(c, [2013, 05, 1]), "dia do trabalho", "dia do trabalhador");

    // Part of day (morning, afternoon...)
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 19]), "esta tarde", "pela tarde", "hoje pela tarde");
    example!(v, check_moment_span!(c, [2013, 2, 12, 15], [2013, 2, 12, 17]), "meio da tarde", "em plena tarde", "no meio da tarde");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 9]), "logo de manhã", "no início da manhã", "logo no início da manhã", "primeiras horas da manhã");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 16]), "no meio do dia");
    example!(v, check_moment!(c, [2013, 2, 12, 12]), "ao meio-dia", "às doze horas");
    example!(v, check_moment_span!(c, [2013, 2, 12, 10], [2013, 2, 12, 12]), "no final da manhã", "nas últimas horas da manhã");
    // TODO: ?? Before lunch, after work, etc
    //example!(v, check_moment_span!(c, [2013, 2, 12, 13], [2013, 2, 12, 17]), "depois de comer", "depois da hora de comer");
    //example!(v, check_moment_span!(c, [2013, 2, 12, 10], [2013, 2, 12, 12]), "antes de comer");
    //example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "durante a refeição", "na hora da refeição" );
    //example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 21]), "depois do trabalho", "ao sair do trabalho");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 15]), "no começo da tarde", "logo à tarde", "no início da tarde");
    example!(v, check_moment_span!(c, [2013, 2, 12, 15], [2013, 2, 12, 17]), "no final da tarde", "nas últimas horas da tarde", "no fim da tarde");
    example!(v, check_moment_span!(c, [2013, 2, 12, 6], [2013, 2, 12, 10]), "no início do dia", "no começo do dia");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11], [2013, 2, 12, 16]), "na metade do dia", "no meio do dia");
    example!(v, check_moment_span!(c, [2013, 2, 12, 17], [2013, 2, 12, 21]), "no final do dia", "ao fim do dia", "ao acabar o dia");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 00]), "esta noite", "de noite");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 12, 21]), "no início da noite", "logo à noite");
    // TODO: Ask Drica: tarde a part-of-day (and logo)
    example!(v, check_moment_span!(c, [2013, 2, 12, 21], [2013, 2, 13, 00]), "tarde da noite", "no final da noite", "no fim da noite");
    example!(v, check_moment_span!(c, [2013, 2, 18, 4], [2013, 2, 18, 12]), "segunda-feira de manhã", "na segunda pela manhã");
    example!(v, check_moment_span!(c, [2013, 2, 18, 12], [2013, 2, 18, 19]), "segunda de tarde", "segunda-feira pela tarde");
    example!(v, check_moment_span!(c, [2013, 2, 18, 15], [2013, 2, 18, 17]), "segunda-feira no final da tarde", "no fim da tarde de segunda", "no fim da tarde de segunda-feira");

    example!(v, check_moment_span!(c, [2013, 2, 15, 4], [2013, 2, 15, 12]), "dia quinze de fevereiro pela manhã", "no dia quinze de fevereiro pela manhã");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "hoje às oito da noite", "8 da noite", "às oito da noite");
    // TODO: IMPORTANT:
    //example!(v, check_moment!(c, [2013, 2, 13, 3]), "3 da manhã", "às três da manhã");

    // Part of the week/month
    example!(v, check_moment_span!(c, [2013, 2, 13, 18], [2013, 2, 14, 00]), "amanhã de noite", "na quarta-feira de noite", "na noite de quarta-feira");
    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 00]), "ontem à noite");
    // Ambiguity: end of week // week-end
    //example!(v, check_moment_span!(c, [2013, 2, 15, 18], [2013, 2, 18, 00]), "este fim de semana", "este final de semana", "no próximo final de semana");
    example!(v, check_moment_span!(c, [2013, 2, 11], [2013, 2, 13]), "no começo da semana", "no início desta semana");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 15]), "no meio da semana", "na metade da semana");
    example!(v, check_moment_span!(c, [2013, 2, 14], [2013, 2, 18]), "no final da semana", "no fim de semana");
    example!(v, check_moment_span!(c, [2013, 2, 11], [2013, 2, 16]), "durante a semana");
    example!(v, check_moment_span!(c, [2013, 2, 19], [2013, 3, 01]), "no final do mês", "no final do mês");
    example!(v, check_moment_span!(c, [2013, 9, 6, 18], [2013, 9, 9, 00]), "o primeiro fim de semana de setembro", "o primeiro final de semana de setembro", "o primeiro fim de semana do mês de setembro");
    example!(v, check_moment_span!(c, [2013, 9, 13, 18], [2013, 9, 16, 00]), "o segundo final de semana de setembro");
    example!(v, check_moment_span!(c, [2013, 9, 27, 18], [2013, 9, 30, 00]), "o último fim de semana de setembro");

    // Intervals involving cycles
    //example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "2 últimos segundos", "os dois últimos segundos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "os próximos 3 segundos", "os 3 próximos segundos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "os 2 últimos minutos", "últimos dois minutos");
    // FIXME: this is confused b/ time and interval
    // fix_example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "nos próximos três minutos", "durante os próximos três minutos");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "as 3 próximas horas");
    // FIXME: same as above
    // fix_example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "últimos dois dias");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "próximos 3 dias", "os próximos 3 dias");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11]), "últimas 2 semanas", "as duas últimas semanas");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11]), "as próximas três semanas", "as três próximas semanas");
    example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "últimos dois meses", "os últimos dois meses");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "os três próximos meses");
    example!(v, check_moment_span!(c, [2011], [2013]), "os últimos 2 anos", "os dois anos anteriores");
    example!(v, check_moment_span!(c, [2014], [2017]), "os próximos 3 anos", "os três próximos anos");

    // Explicit intervals
    // FIXME:
    // fix_example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]),  "13 julho - 15 julho",  "de 13 a 15 de julho",  "do dia 13 ao dia 15 de julho",  "de sábado dia 13 a domingo dia 15 de julho",  "de sábado dia 13 até domingo dia 15 de julho",  "do dia 13 a domingo dia 15");

    // FIXME: Resolution error (second date bad resolution) + "13-15 julho" not supported
    // example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "13-15 julho", "entre dia 13 e dia 15 de julho");
    // FIXME: Resolution issue 10 --> 11 , + "de segunda dia primeiro a quarta-feira dia 10 de julho" not parsed
    // fix_example!(v, check_moment_span!(c, [2013, 7, 1], [2013, 7, 10]), "do dia 1 ao dia 10 de julho", "de segunda dia primeiro a quarta-feira dia 10 de julho");
    // FIXME: Resolution eror : 06/18 -> 06/19
    //example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 18]), "do dia 13 ao dia 18", "entre dia 13 e dia 18");
    example!(v, check_moment_span!(c, [2023, 2, 1], [2023, 3, 1]), "entre fevereiro e março de dois mil e vinte e três", "entre o mês de fevereiro e o mês de março de dois mil e vinte e três");
    // FIXME: Resolution error : "primeiro de julho" -> 06/01 but "de 10 de junho a primeiro de julho" -> 06/02
    // example!(v, check_moment_span!(c, [2013, 6, 10], [2013, 7, 2]), "de 10 de junho a primeiro de julho", "entre o dia dez de junho e dia primeiro de julho", "entre o dia 10 de junho e o dia primeiro de julho", "de 10 de junho a 1º de julho");
    // FIXME: Resolution error: start: 2017-04-06T00:00:00+02:00, end: 2017-06-09T00:00:00+02:00, 8 --> 9
    // example!(v, check_moment_span!(c, [2017,4,6], [2017,6,9]), "do dia seis de abril ao dia oito de junho de dois mil e dezessete", "de seis de abril ao dia oito de junho de dois mil e dezessete");
    // TODO: OK, but abbreviation "5ª f." not supported, "09:30 - 11:00" as interval not supported
    // fix_example!(v, check_moment_span!(c, [2013, 2, 14, 9, 30], [2013, 2, 14, 11]), "09:30 - 11:00 na quinta-feira", "das 9h30 às 11h na quinta-feira", "das 9 e meia às 11 de quinta-feira", "na quinta-feira das 9h30 às 11h", "quinta-feira entre 9 e meia e 11", "quinta das 9h30 às 11h", "5ª f. entre as 09:30 e as 11:00");
    example!(v, check_moment_with_direction!(c, [2013, 3, 8], Direction::After), "a partir do dia 08", "desde o dia 8 de março", "do dia 8 de março em diante");
    example!(v, check_moment_with_direction!(c, [2013, 2, 14, 9, 30], Direction::After), "a partir das 09h30 de quinta-feira", "quinta pela manhã das 9 e meia em diante");
    example!(v, check_moment_with_direction!(c, [2013, 11, 1, 16], Direction::After), "depois das 16h do dia um de novembro", "depois das 4 da tarde do dia um de novembro", "no dia um de novembro depois das 16 horas");
    example!(v, check_moment_with_direction!(c, [2013, 11, 1], Direction::After), "depois do dia um de novembro");

    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 16], Direction::Before), "antes das 16h", "até as 4 da tarde");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 16, 0], Direction::Before), "até as 16:00");
    // FIXME: Same as below (Interval(..-..) expected but got Before(..))
    // example!(v, check_moment_span!(c, [2013, 2, 13, 0], [2013, 2, 13, 6]), "amanhã até as seis horas", "amanhã antes das 6h", "até as 6 horas amanhã");
    // FIXME: Same as below (Interval(..-..) expected but got After(..))
    // todo_example!(v, check_moment_with_direction!(c, [2013, 2, 20, 10], Direction::After), "13/02/20 18:00", "no dia 20 a partir das 10 horas", "desde as dez horas no dia vinte");
    example!(v, check_moment_with_direction!(c, [2013, 2, 15, 12], Direction::After), "na sexta-feira a partir do meio-dia", "na sexta-feira do meio-dia em diante", "desde sexta-feira ao meio-dia");
    // FIXME: Resolved as: Before(13/02/20 18:00), but what is expected is: Interval(13/02/20 00:00 - 13/02/20 18:00)
    //example!(v, check_moment_span!(c, [2013, 2, 20], [2013, 2, 20, 18]), "no dia 20 até as 18h");
    // TODO: Do not support 14-20 (dash as time period) and set. (abbreviation of month)
    // FIXME: "14" ---> 2014/01/01 (bad resolution) and 20 de setembro 2014 --> OK
    //example!(v, check_moment_span!(c, [2014, 9, 14], [2014, 9, 21]), "14-20 set. 2014", "de 14 a 20 de setembro 2014");

    example!(v, check_moment!(c, [2013, 2, 26]), "dentro de 2 semanas", "em duas semanas");
    example!(v, check_moment!(c, [2013, 5, 12]), "dentro de 3 meses", "em três meses");
    example!(v, check_moment!(c, [2013, 2, 27]),"em 15 dias","nos próximos quinze dias","dentro de 15 dias");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 7]), "das 5 às 7", "entre 5 e 7 horas", "de 5 à sete horas");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9], [2013, 2, 14, 11]), "quinta-feira das 9 às 11", "na quinta-feira entre as 9 e as 11");

    // FIXME: "entre as doze e as cartorze horas" (bad resolution: doze)
    //example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 14]), "do meio-dia às 14h", "entre o meio-dia e as duas da tarde", "entre as doze e as cartorze horas");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 30]), "das 11h30 à 1h30", "das 11 e meia à uma e meia");
    example!(v, check_moment!(c, [2013, 9, 21, 13, 30]), "às 13:30 de sábado dia 21 de setembro", "às 13h30 de sábado dia 21 de setembro");
    example!(v, check_moment_span!(c, [2013, 3, 25], [2013, 4, 1]), "no final de março", "no fim do mês de março");
    example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 6]), "no começo de abril", "no início do mês de abril");
    example!(v, check_moment_span!(c, [2013, 4, 1], [2013, 4, 15]), "a primeira quinzena de abril", "na primeira quinzena do mês de abril");
    example!(v, check_moment_span!(c, [2013, 4, 15], [2013, 5, 01]), "a segunda quinzena de abril", "na segunda quinzena do mês de abril");
    example!(v, check_moment_span!(c, [2013, 12, 10], [2013, 12, 20]),  "meados de dezembro", "em meados de dezembro", "na metade do mês de dezembro");
    example!(v, check_moment!(c, [2013, 3]), "março", "em março", "durante o mês de março", "o mês de março");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 45, 0]), "dentro de quinze minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 0, 0]), "dentro de meia hora", "dentro de trinta minutos");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 15, 0]), "dentro de quarenta e cinco minutos");
    example!(v, check_moment!(c, [2016, 12, 15]), "15.12.2016", "15.12.16", "15/12/2016");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 2]), "durante duas horas", "por duas horas");
    // TODO: todo a dia
    // example!(v, check_duration!([0, 0, 0, 1]), "durante um dia", "por um dia", "todo o dia");
    example!(v, check_duration!([0, 1, 0]), "durante um mês", "por um mês");
    example!(v, check_duration!([1]), "durante um ano", "por um ano");
    example!(v, check_duration!([0, 0, 0, 0, 0, 1, 3]), "durante um minuto e três segundos", "um minuto e três segundos");
    example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "cerca de uma hora e meia", "uma hora e meia mais ou menos", "uma hora e meia aproximadamente", "por volta de 1h30");
    example!(v, check_duration!([0, 0, 0, 0, 0, 15], Precision::Approximate), "durante um quarto de hora mais o menos", "aproximadamente durante um quarto de hora");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "durante uma hora", "por uma hora");
    example!(v, check_duration!([0, 0, 2]), "durante 2 semanas", "por duas semanas");
}