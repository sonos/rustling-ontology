use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "0.3%", "zero point three per cent");
    example!(v, check_percentage(15.0), "15%", "15 %", "fifteen percent");
    example!(v, check_percentage(202.0), "202%", "202 p.c.", "202percent");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "three degrees", "3 degrees", "3°", "3 °");
    example!(v, check_temperature(32.0, Some("celsius")), "thirty two degrees celsius", "thirty two degrees centigrade", "32°C", "32 °c");
    example!(v, check_temperature(-27.0, Some("celsius")), "minus 27 celsius", "-27C", "- 27 c");
    example!(v, check_temperature(-5.0, Some("fahrenheit")), "minus five degrees fahrenheit", "-5 °F", "- 5°f");
    example!(v, check_temperature(168.0, Some("fahrenheit")), "one hundred and sixty-eight fahrenheit", "168 F", "168f");
    example!(v, check_temperature(10.0, Some("kelvin")), "ten degrees kelvin", "10 °K", "10°k");
    example!(v, check_temperature(21.0, Some("kelvin")), "21 kelvin", "21 K", "21k");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(800.0, Some("$"), Precision::Exact), "800$", "eight hundred dollars", "eight hundred dollar");
    example!(v, check_finance(10.0, Some("USD"), Precision::Approximate), "around ten us dollars", "almost 10US$");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "exactly 3 australian dollar", "precisely 3 AUD");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "zero hk dollar");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "125 cad");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact), "forty five euros", "45 €", "45EUR");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "2 pounds", "two £");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "twenty british pounds", "20 sterlings", "20 GBP");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "38 swiss francs");
    example!(v, check_finance(447.0, Some("KR"), Precision::Exact), "447 kroner", "447 crowns");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "10,000 DKK");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact), "one hundred norwegian crowns", "100 norwegian kroner");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "2005 SEK");
    example!(v, check_finance(96.0, Some("INR"), Precision::Exact), "96 indian rupees");
    example!(v, check_finance(5.0, Some("RUB"), Precision::Exact), "five rubles");
    example!(v, check_finance(89.0, Some("JPY"), Precision::Approximate), "about 89 yen");
    example!(v, check_finance(200.0, Some("CNY"), Precision::Exact), "two hundred yuan");
    example!(v, check_finance(7.0, Some("KRW"), Precision::Exact), "7 wons", "7₩");
    example!(v, check_finance(3.0, Some("฿"), Precision::Exact), "3฿", "3 ฿", "three bitcoins");
}


// TODO: Sort out and split by datetime subtype
pub fn examples_datetime(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "now", "right now", "just now");
    example!(v, check_moment!(c, [2013, 2, 12]), "today", "at this time");
    example!(v, check_moment!(c, [2013, 2, 11]), "yesterday");
    example!(v, check_moment!(c, [2013, 2, 13]), "tomorrow");
    example!(v, check_moment!(c, [2013, 2, 18]), "monday", "mon.", "this monday");
    example!(v, check_moment!(c, [2013, 2, 18]), "Monday, Feb 18", "Mon, February 18");
    example!(v, check_moment!(c, [2013, 2, 19]), "tuesday", "Tuesday the 19th", "Tuesday 19th");
    example!(v, check_moment!(c, [2013, 2, 14]), "thursday", "thu", "thu.");
    example!(v, check_moment!(c, [2013, 2, 15]), "friday", "fri", "fri.");
    example!(v, check_moment!(c, [2013, 2, 16]), "saturday", "sat", "sat.");
    example!(v, check_moment!(c, [2013, 2, 17]), "sunday", "sun", "sun.");
    example!(v, check_moment!(c, [2013, 3, 1]), "the 1st of march", "first of march", "march first");
    example!(v, check_moment!(c, [2013, 3, 3]), "march 3");
    example!(v, check_moment!(c, [2013, 3, 15]), "the ides of march");
    example!(v, check_moment!(c, [2015, 3, 3]), "march 3 2015", "march 3rd 2015", "march third 2015");
    example!(v, check_moment!(c, [2015, 3, 3]), "3/3/2015", "3/3/15", "2015-3-3", "2015-03-03");
    example!(v, check_moment!(c, [2015, 8, 31]), "31/08/2015", "31/08/15", "2015-08-31", "2015-08-31");
    example!(v, check_moment!(c, [2015, 8, 31]), "08/31/2015", "08/31/15", "2015-08-31", "2015-08-31");
    example!(v, check_moment!(c, [2013, 3, 3]), "3/3");
    example!(v, check_moment!(c, [2013, 8, 31]), "31/08");
    example!(v, check_moment!(c, [2013, 8, 31]), "08/31");
    example!(v, check_moment!(c, [2013, 2, 15]), "on the 15", "on the 15th");
    example!(v, check_moment!(c, [2013, 2, 15]), "the 15th of february", "15 of february", "february the 15th", "february 15", "15th february", "2/15", "on 2/15", "February 15");
    example!(v, check_moment!(c, [2013, 8, 8]), "Aug 8");
    example!(v, check_moment!(c, [2014, 10]), "October 2014");
    example!(v, check_moment!(c, [2015, 4, 14]), "14 april 2015", "April 14, 2015", "14th April 15");
    example!(v, check_moment!(c, [2013, 2, 19]), "next tuesday");
    example!(v, check_moment!(c, [2013, 2, 22]), "friday after next");
    example!(v, check_moment!(c, [2013, 3]), "next March");
    example!(v, check_moment!(c, [2014, 3]), "March after next");
    example!(v, check_moment!(c, [2013, 2, 10]), "Sunday, Feb 10");
    example!(v, check_moment!(c, [2013, 2, 13]), "Wed, Feb13");
    example!(v, check_moment!(c, [2013, 2, 18]), "Monday, Feb 18", "Mon, February 18");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "this week", "current week", "coming week");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "last week", "past week", "previous week");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "next week", "the following week");
    example!(v, check_moment!(c, [2013, 1]), "last month");
    example!(v, check_moment!(c, [2013, 3]), "next month");
    example!(v, check_moment!(c, [2013, 1, 1], Grain::Quarter), "this quarter", "this qtr");
    example!(v, check_moment!(c, [2013, 4, 1], Grain::Quarter), "next quarter", "next qtr");
    example!(v, check_moment!(c, [2013, 7, 1], Grain::Quarter), "third quarter", "3rd quarter", "third qtr", "3rd qtr", "the 3rd qtr");
    example!(v, check_moment!(c, [2018, 10, 1], Grain::Quarter), "4th quarter 2018", "4th qtr 2018", "the 4th qtr of 2018");
    example!(v, check_moment!(c, [2012]), "last year", "last yr");
    example!(v, check_moment!(c, [2013]), "this year", "current year", "this yr");
    example!(v, check_moment!(c, [2014]), "next year", "next yr");
    example!(v, check_moment!(c, [2013, 2, 10]), "last sunday", "sunday from last week", "last week's sunday");
    example!(v, check_moment!(c, [2013, 2, 5]), "last tuesday");
    example!(v, check_moment!(c, [2013, 2, 13]), "next wednesday");
    example!(v, check_moment!(c, [2013, 2, 20]), "wednesday of next week", "wednesday next week", "wednesday after next");
    example!(v, check_moment!(c, [2013, 2, 22]), "friday after next");
    example!(v, check_moment!(c, [2013, 2, 11]), "monday of this week");
    example!(v, check_moment!(c, [2013, 2, 12]), "tuesday of this week");
    example!(v, check_moment!(c, [2013, 2, 13]), "wednesday of this week");
    example!(v, check_moment!(c, [2013, 2, 14]), "the day after tomorrow");
    example!(v, check_moment!(c, [2013, 2, 14, 17]), "day after tomorrow 5pm");
    example!(v, check_moment!(c, [2013, 2, 10]), "the day before yesterday");
    example!(v, check_moment!(c, [2013, 2, 10, 8]), "day before yesterday 8am");
    example!(v, check_moment!(c, [2013, 3, 25]), "last Monday of March");
    example!(v, check_moment!(c, [2014, 3, 30]), "last Sunday of March 2014");
    example!(v, check_moment!(c, [2013, 10, 3]), "third day of october");
    example!(v, check_moment!(c, [2014, 10, 6], Grain::Week), "first week of october 2014");
    example!(v, check_moment!(c, [2013, 10, 7], Grain::Week), "the week of october 6th", "the week of october 7th");
    example!(v, check_moment!(c, [2015, 10, 31]), "last day of october 2015", "last day in october 2015");
    example!(v, check_moment!(c, [2014, 9, 22], Grain::Week), "last week of september 2014");
    //nth of
    example!(v, check_moment!(c, [2013, 10, 1]), "first tuesday of october", "first tuesday in october");
    example!(v, check_moment!(c, [2014, 9, 16]), "third tuesday of september 2014");
    example!(v, check_moment!(c, [2014, 10, 1]), "first wednesday of october 2014");
    example!(v, check_moment!(c, [2014, 10, 8]), "second wednesday of october 2014");
    example!(v, check_moment!(c, [2015, 1, 13]), "third tuesday after christmas 2014");
    example!(v, check_moment!(c, [2017, 4, 6]), "in twenty seventeen on thursday the sixth of april");
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "at 3am", "3 in the AM", "at 3 AM", "3 oclock am", "at three am");
    example!(v, check_moment!(c, [2013, 2, 13, 3, 18]), "3:18am", "3:18a");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 10]), "ten minutes after eleven", "fifty minutes before noon", "50 minutes before 12");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 10]), "exactly ten minutes after eleven", "fifty minutes before noon precisely");
    example!(v, check_moment_with_precision!(c, [2013, 2, 12, 11, 10], Precision::Approximate), "about 50 minutes before 12", "50 minutes before 12 approximately");
    example!(v, check_moment!(c, [2013, 2, 12, 6, 15]), "fifteen past six am", "quarter past six am", "quarter past six in the morning");
    example!(v, check_moment!(c, [2013, 2, 10, 6, 15]), "two days ago at fifteen past six am", "the day before yesterday at quarter past six am", "quarter past six in the morning two days ago");
    example!(v, check_moment!(c, [2013, 2, 12, 18, 15]), "fifteen past six pm", "quarter past six pm", "quarter past six in the evening");
    example!(v, check_moment!(c, [2013, 2, 12, 18, 15]), "precisely fifteen past six pm", "quarter past six pm sharp");
    example!(v, check_moment_with_precision!(c, [2013, 2, 12, 18, 15], Precision::Approximate), "approximately quarter past six in the evening", "15 past 6 in the afternoon approximately");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "at 3pm", "@ 3pm", "3PM", "3pm", "3 oclock pm", "3 o'clock in the afternoon");
    example!(v, check_moment_with_precision!(c, [2013, 2, 12, 15], Precision::Approximate), "3ish pm", "3pm approximately", "at about 3pm");
    example!(v, check_moment!(c, [2013, 2, 12, 14, 50]), "at ten to three", "at 10 to 3 pm", "10 to 3 in the afternoon");
    example!(v, check_moment!(c, [2013, 2, 13, 3, 15]), "at 15 past 3am", "a quarter past 3am", "3:15 in the morning", "3:15am", "3:15AM", "3:15a");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "at 15 past 3pm", "a quarter past 3pm", "3:15 in the afternon", "15:15", "3:15pm", "3:15PM", "3:15p");
    example!(v, check_moment!(c, [2013, 2, 12, 18, 45]), "at 15 to 7pm", "a quarter to 7pm", "6:45 in the afternon", "18:45", "6:45pm", "6:45PM", "6:45p");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 20]), "at 20 past 3pm", "3:20 in the afternoon", "3:20 in afternoon", "twenty after 3pm", "3:20p");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "at half past three pm", "half past 3 pm", "15:30", "3:30pm", "3:30PM", "330 p.m.", "3:30 p m");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 30]), "3:30", "half three");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 23, 24]), "15:23:24");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 45]), "a quarter to noon", "11:45am", "15 to noon"); // Ambiguous with interval
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "8 tonight", "eight tonight", "8 this evening");
    //Mixing date and time
    example!(v, check_moment!(c, [2013, 9, 20, 19, 30]), "at 7:30 PM on Fri, Sep 20");
    example!(v, check_moment!(c, [2013, 2, 16, 9]), "at 9am on Saturday", "on Saturday for 9am");
    example!(v, check_moment!(c, [2014, 7, 18, 19, 0]), "Fri, Jul 18, 2014 07:00 PM");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "in a sec", "one second from now");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "in a minute", "in one minute");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 0]), "in 2 minutes", "in 2 more minutes", "2 minutes from now");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "in 60 minutes");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 0, 0]), "in half an hour", "in 1/2h", "in 1/2 h", "in 1/2 hour");
    example!(v, check_moment!(c, [2013, 2, 12, 7, 0, 0]), "in 2.5 hours", "in 2 and an half hours");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "in one hour", "in 1h");
    example!(v, check_moment!(c, [2013, 2, 12, 6, 30]), "in a couple hours", "in a couple of hours");
    example!(v, check_moment!(c, [2013, 2, 12, 7, 30]), "in a few hours", "in few hours");
    example!(v, check_moment!(c, [2013, 2, 13, 4, 30]), "in 24 hours", "in 24hrs", "in 24 hrs");
    example!(v, check_moment!(c, [2013, 2, 13]), "in a day", "a day from now");
    example!(v, check_moment!(c, [2016, 2]), "3 years from today");
    example!(v, check_moment!(c, [2013, 2, 19]), "in 7 days");
    example!(v, check_moment!(c, [2013, 2, 19]), "in 1 week", "in a week");
    example!(v, check_moment!(c, [2013, 2, 5]), "7 days ago");
    example!(v, check_moment!(c, [2013, 1, 29]), "14 days ago", "a fortnight ago");
    example!(v, check_moment!(c, [2013, 2, 5]), "a week ago", "one week ago", "1 week ago");
    example!(v, check_moment!(c, [2013, 1, 22]), "three weeks ago");
    example!(v, check_moment!(c, [2012, 11, 12]), "three months ago");
    example!(v, check_moment!(c, [2011, 2]), "two years ago");
    example!(v, check_moment!(c, [2001]), "2001");
    example!(v, check_moment!(c, [2013, 2, 19]), "7 days hence");
    example!(v, check_moment!(c, [2013, 2, 26]), "14 days hence", "a fortnight hence");
    example!(v, check_moment!(c, [2013, 2, 19]), "a week hence", "one week hence", "1 week hence");
    example!(v, check_moment!(c, [2013, 3, 5]), "three weeks hence");
    example!(v, check_moment!(c, [2013, 5, 12]), "three months hence");
    example!(v, check_moment!(c, [2015, 2]), "two years hence");
    example!(v, check_moment!(c, [2013, 12]), "one year after christmas");
    example!(v, check_moment_span!(c, [2013, 6, 21], [2013, 9, 24]), "this summer", "current summer");
    example!(v, check_moment_span!(c, [2012, 12, 21], [2013, 3, 21]), "this winter");
    example!(v, check_moment!(c, [2013, 12, 25]), "xmas", "christmas", "christmas day");
    example!(v, check_moment!(c, [2013, 12, 31]), "new year's eve", "new years eve");
    example!(v, check_moment!(c, [2014, 1, 1]), "new year's day", "new years day");
    example!(v, check_moment!(c, [2013, 2, 14]), "valentine's day", "valentine day");
    example!(v, check_moment!(c, [2013, 5, 27]), "memorial day");
    example!(v, check_moment!(c, [2013, 5, 12]), "Mother's Day");
    example!(v, check_moment!(c, [2013, 6, 16]), "Father's Day");
    example!(v, check_moment_span!(c, [2013, 5, 24, 18], [2013, 5, 28, 0]), "memorial day week-end");
    example!(v, check_moment!(c, [2013, 7, 4]), "independence day", "4th of July", "4 of july");
    example!(v, check_moment!(c, [2013, 9, 2]), "labor day");
    example!(v, check_moment_span!(c, [2013, 8, 30, 18], [2013, 9, 3, 0]), "labor day weekend");
    example!(v, check_moment!(c, [2013, 10, 31]), "halloween");
    example!(v, check_moment!(c, [2013, 11, 28]), "thanksgiving day", "thanksgiving");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 00]), "this evening", "today evening", "tonight");
    example!(v, check_moment_span!(c, [2013, 2, 12, 00], [2013, 2, 12, 05]), "this night");
    example!(v, check_moment_span!(c, [2013, 2, 8, 18], [2013, 2, 11, 00]), "this past weekend");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "last 2 seconds", "last two seconds");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "next 3 seconds", "next three seconds");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "last 2 minutes", "last two minutes");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "next 3 minutes", "next three minutes");
    example!(v, check_moment_span!(c, [2013, 2, 12, 3], [2013, 2, 12, 4]), "last 1 hour", "last 1 hr", "last one hour");
    example!(v, check_moment_span!(c, [2013, 2, 11, 4], [2013, 2, 12, 4]), "last 24 hours", "last twenty four hours", "last twenty four hrs", "last 24 hrs", "last 24hrs");
    example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "next 3 hours", "next three hours");
    example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "last 2 days", "last two days", "past 2 days");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "next 3 days", "next three days");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "next few days");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11], Grain::Week), "last 2 weeks", "last two weeks", "past 2 weeks");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11], Grain::Week), "next 3 weeks", "next three weeks");
    example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "last 2 months", "last two months");
    example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "next 3 months", "next three months");
    example!(v, check_moment_span!(c, [2011], [2013]), "last 2 years", "last two years");
    example!(v, check_moment_span!(c, [2014], [2017]), "next 3 years", "next three years");
    example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "July 13-15", "July 13 to 15", "July 13 thru 15", "July 13 through 15", "July 13 - July 15");
    example!(v, check_moment_span!(c, [2013, 8, 8], [2013, 8, 13]), "Aug 8 - Aug 12");
    example!(v, check_moment_span!(c, [2013, 2, 12, 9, 30], [2013, 2, 12, 11, 0]), "9:30 - 11:00");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9, 30], [2013, 2, 14, 11, 0]), "from 9:30 - 11:00 on Thursday", "between 9:30 and 11:00 on thursday", "9:30 - 11:00 on Thursday", "later than 9:30 but before 11:00 on Thursday", "Thursday from 9:30 to 11:00","from 9:30 untill 11:00 on thursday", "Thursday from 9:30 untill 11:00", "9:30 till 11:00 on Thursday");
    example!(v, check_moment_span!(c, [2013, 2, 14, 9], [2013, 2, 14, 11]), "Thursday from 9a to 11a");
    example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 30]), "11:30-1:30", "11:30-1:30", "11:30-1:30", "11:30-1:30", "11:30-1:30", "11:30-1:30", "11:30-1:30");
    example!(v, check_moment!(c, [2013, 9, 21, 13, 30]), "1:30 PM on Sat, Sep 21");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 26]), "within 2 weeks");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14, 0], Direction::Before), "until 2:00pm", "through 2:00pm");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 14]), "by 2:00pm");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 13, 0]), "by EOD");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 3, 1, 0]), "by EOM");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 4, 1, 0]), "by the end of next month");
    example!(v, check_moment!(c, [2013, 2, 12, 14]), "today at 2pm", "at 2pm");
    example!(v, check_moment!(c, [2013, 4, 25, 16, 0]), "4/25 at 4:00pm");
    example!(v, check_moment!(c, [2013, 2, 13, 15]), "3pm tomorrow");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14], Direction::After), "after 2 pm");
    example!(v, check_moment_with_direction!(c, [2013, 2, 17], Direction::After), "after 5 days");
    example!(v, check_moment_with_direction!(c, [2013, 2, 12, 11], Direction::Before), "before 11 am");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 13, 11]), "tomorrow before 11 am", "13th feb. 2013 until 11am");
    example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 19]), "in the afternoon");
    example!(v, check_moment!(c, [2013, 2, 12, 13, 30]), "at 1:30pm", "1:30pm", "at thirteen thirty");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 45, 0]), "in 15 minutes");
    example!(v, check_moment_span!(c, [2013, 2, 12, 13], [2013, 2, 12, 17]), "after lunch");
    example!(v, check_moment!(c, [2013, 2, 12, 10, 30]), "10:30");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 12]), "morning"); //how should we deal with fb morning
    example!(v, check_moment!(c, [2013, 2, 18]), "next monday");
    example!(v, check_moment!(c, [2013, 2, 12, 12]), "at 12pm", "at noon");
    example!(v, check_moment!(c, [2013, 2, 13, 0]), "at 12am", "at midnight");
    example!(v, check_moment!(c, [2013, 3]), "March", "in March");
    example!(v, check_moment!(c, [2016, 12, 15]), "12.15.2016", "12.15.16");
    example!(v, check_moment!(c, [2017, 05, 10]), "wednesday the 10th of may");
    example!(v, check_moment!(c, [2013, 2, 12, 9, 9]), "at nine nine", "at nine nine am", "at nine nine in the morning");
    example!(v, check_moment_span!(c, [2017, 05, 12, 10, 32], [2017, 06, 7, 18, 23]), "friday the 12th of may, 10:32 am to wednesday the 7th of june, 6:22 pm");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 2]), "during two hours", "for 2 hours");
    example!(v, check_duration!([0, 0, 0, 1], Precision::Approximate), "about one day", "approximately 1 day");
    example!(v, check_duration!([0, 2, 0]), "during two months", "for 2 months");
    example!(v, check_duration!([1]), "during a year");
    example!(v, check_duration!([0, 0, 0, 0, 0, 1, 3]), "during one minute and three seconds", "for 1 minute and 3 seconds", "for 1min3sec");
    example!(v, check_duration!([0, 0, 0, 0, 7, 15], Precision::Approximate), "during about seven hours and a quarter");
    example!(v, check_duration!([0, 0, 0, 0, 3, 30], Precision::Approximate), "about three and a half hours", "around 3 hours and a half");
    example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "about one hour and a half");
    example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "about 1h30", "for around one hour and thirty minutes");
    example!(v, check_duration!([0, 0, 0, 0, 0, 15], Precision::Approximate), "during about a quarter of an hour", "for approximately 1/4hour", "around a quarter-hour");
    example!(v, check_duration!([0, 0, 0, 0, 0, 45]), "for three-quarters of an hour", "for 3/4h", "for 3/4 h", "for 3/4 hour");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "during one hour", "for 1h");
    example!(v, check_duration!([0, 0, 2]), "for 2 weeks");
    example!(v, check_duration!([0, 0, 0, 2], Precision::Approximate), "around two days");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "0", "naught", "nought", "zero", "nil");
    example!(v, check_integer(1), "1", "one", "single");
    example!(v, check_integer(2), "2", "two", "a pair");
    example!(v, check_integer(33), "33", "thirty three", "0033");
    example!(v, check_integer(14), "14", "fourteen");
    example!(v, check_integer(16), "16", "sixteen");
    example!(v, check_integer(17), "17", "seventeen");
    example!(v, check_integer(18), "18", "eighteen");
    example!(v, check_float(1.1), "1.1", "1.10", "01.10", "one point ten", "one point one");
    example!(v, check_float(0.5), "0.5", "0.50", "zero point five");
    example!(v, check_float(32.75), "32.75", "thirty-two point seventy-five");
    example!(v, check_float(10.08), "10.08", "ten point zero eight");
    example!(v,
             check_integer(100000),
             "100,000",
             "100000",
             "100K",
             "100k");
    example!(v,
             check_integer(3000000),
             "3M",
             "3000K",
             "3000000",
             "3,000,000");
    example!(v,
             check_integer(1200000),
             "1,200,000",
             "1200000",
             "1.2M",
             "1200K",
             ".0012G");
    example!(v,
             check_integer(-1200000),
             "- 1,200,000",
             "-1200000",
             "minus 1,200,000",
             "negative 1200000",
             "-1.2M",
             "-1200K",
             "-.0012G");
    example!(v, check_integer(5000), "5 thousand", "five thousand");
    example!(v, check_integer(122), "one twenty two");
    example!(v, check_integer(200000), "two hundred thousand");
    example!(v, check_integer(21011), "twenty-one thousand eleven");
    example!(v,
             check_integer(721012),
             "seven hundred twenty-one thousand twelve",
             "seven hundred twenty-one thousand and twelve");
    example!(v,
             check_integer(31256721),
             "thirty-one million two hundred fifty-six thousand seven hundred twenty-one");
    example!(v, check_ordinal(4), "the 4th", "4th", "fourth");
    example!(v, check_ordinal(3), "the 3rd", "3rd", "third");
    example!(v, check_ordinal(2), "the 2nd", "2nd", "second");
    example!(v, check_ordinal(21), "the twenty first");
}