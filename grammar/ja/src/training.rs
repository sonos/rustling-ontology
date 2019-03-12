use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "零", "0", "０", "〇");
    example!(v, check_integer(1), "一", "一尾", "一把");
    example!(v, check_integer(2), "二", "二名", "二羽");
    example!(v, check_integer(3), "三", "三話", "三人", "三画");
    example!(v, check_integer(4), "四", "四面", "四冊");
    example!(v, check_integer(5), "五", "五問", "五曲");
    example!(v, check_integer(6), "六", "六拍子", "六種");
    example!(v, check_integer(7), "七", "七段", "七種類");
    example!(v, check_integer(8), "八", "八門", "八カ国");
    example!(v, check_integer(9), "九", "九倍", "九ヶ国");
    example!(v, check_integer(10), "十", "十畳", "十クラス");
    example!(v, check_integer(11), "十一", "十一膳", "十一丁");
    example!(v, check_integer(14), "十四", "十四ページ", "十四口");
    example!(v, check_integer(20), "二十", "二十例", "二十束");
    example!(v, check_integer(30), "三十", "三十字", "三十機");
    example!(v, check_integer(33), "三十三", "三十三文", "三十三行");
    example!(v, check_integer(90), "九十", "九十行", "九十通", "九十通り");
    example!(v, check_integer(340), "三百四十", "三百四十語", "三百四十戸");
    example!(v, check_integer(1096), "千九十六", "千九十六台", "千九十六切れ");
    example!(v, check_integer(40020), "四万二十", "四万二十坪", "四万二十つ", "四万二十カ国", "四万二十字", "四万二十拍子");

    example!(v, check_float(0.8), "0.8", "0点８", "零点八", "〇点８", "〇.８");

    example!(v, check_ordinal(1), "最初", "一番目", "一行目", "一錠目", "一匹目");
    example!(v, check_ordinal(7), "七番目", "七体目", "七問目", "七拍子目", "七種目", "七種類目");
    example!(v, check_ordinal(11), "十一番目", "十一句目", "十一面目", "十一ページ目", "十一口目");
    example!(v, check_ordinal(91), "九十一番目", "九十一滴目", "九十一段目", "九十一拍子目", "九十一切れ目");
    example!(v, check_ordinal(40020), "四万二十坪目", "四万二十つ目", "四万二十カ国目", "四万二十字目", "四万二十拍子目");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(800.0, Some("$"), Precision::Exact), "800$", "800ドル", "八百ドル");
    example!(v, check_finance(10.0, Some("USD"), Precision::Exact), "10アメリカドル", "十米ドル");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "3豪ドル", "三オーストラリアドル");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "0香港ドル", "零香港ドル");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "125カナダドル", "百二十五カナダドル");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Approximate), "45€ほど", "四十五ユーロくらい", "45ユーロ程");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "2ポンド");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "ちょうど二十イギリスポンド", "まさに20英ポンド");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "38スイスフラン");
    example!(v, check_finance(447.0, Some("KR"), Precision::Approximate), "四百四十七クローネ位");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "一万デンマーククローネ");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact), "100ノルウェークローネ");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "二千五スウェーデンクローナ");
    example!(v, check_finance(96.0, Some("INR"), Precision::Exact), "96ルピー", "九十六インドルピー");
    example!(v, check_finance(5.0, Some("RUB"), Precision::Approximate), "約5ルーブル", "だいたい5ルーブリ");
    example!(v, check_finance(89.0, Some("JPY"), Precision::Exact), "八十九円", "89日本円");
    example!(v, check_finance(100.0, Some("CNY"), Precision::Exact), "100元", "百人民元");
    example!(v, check_finance(7.0, Some("KRW"), Precision::Exact), "七ウォン", "7韓国ウォン");
    example!(v, check_finance(3.0, Some("฿"), Precision::Exact), "3ビット", "三ビットコイン");
    }

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "0,3 %", "0,3%", "0,3％");
    example!(v, check_percentage(25.0), "25%", "25％", "25パーセント", "二十五パーセント");
    example!(v, check_percentage(10.0), "割");
    example!(v, check_percentage(1.0), "分");
    example!(v, check_percentage(0.1), "厘");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(0.0, Some("degree")), "零度", "0度", "零ど", "0ど", "0 °", "0°");
    example!(v, check_temperature(5.0, Some("degree")), "五度", "5度", "5 °", "5°");
    example!(v, check_temperature(6.0, Some("degree")), "六ど", "6ど");
    example!(v, check_temperature(14.0, Some("degree")), "14度");
    example!(v, check_temperature(25.0, Some("degree")), "二十五度");
    example!(v, check_temperature(0.0, Some("celsius")), "摂氏0度", "摂氏零ど", "0°C", "0℃");
    example!(v, check_temperature(50.0, Some("celsius")), "摂氏50度", "摂氏五十ど");
    example!(v, check_temperature(7.0, Some("celsius")), "七°C", "七℃", "7°C", "7℃");
    example!(v, check_temperature(0.0, Some("fahrenheit")), "華氏0度", "華氏零ど", "カ氏0度", "カ氏零ど", "0°F", "零℉");
    example!(v, check_temperature(5.0, Some("fahrenheit")), "華氏五度", "華氏5ど", "カ氏5ど");
    example!(v, check_temperature(3.0, Some("fahrenheit")), "三°F", "三℉", "3 °F", "3 ℉");
    example!(v, check_temperature(0.0, Some("kelvin")), "0ケルビン", "零ケルビン");
    example!(v, check_temperature(12.0, Some("kelvin")), "12ケルビン", "十二ケルビン");
    example!(v, check_temperature(25.0, Some("kelvin")), "25°K");
    example!(v, check_temperature(-10.0, Some("degree")), "マイナス十度", "零下十度", "れいか十ど",  "れいか10ど", "-10°");
    example!(v, check_temperature(-7.0, Some("celsius")), "マイナス七°C", "零下7℃", "摂氏マイナス七度", "摂氏零下7ど", "摂氏れいか7度");
    example!(v, check_temperature(-3.0, Some("fahrenheit")), "マイナス3°F", "零下三℉", "華氏マイナス3度", "華氏零下三度", "カ氏れいか三ど");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_duration!([0, 0, 0, 0, 0, 0, 1]), "一秒間");
    example!(v, check_duration!([0, 0, 0, 0, 0, 1]), "一分間");
    example!(v, check_duration!([0, 0, 0, 0, 1]), "一時間");
    example!(v, check_duration!([0, 0, 0, 5]), "五日間");
    example!(v, check_duration!([0, 10]), "十ヶ月間", "十カ月間");
    example!(v, check_duration!([0, 0, 0, 0, 2], Precision::Approximate), "およそ二時間", "二時間程", "二時間位");

}

pub fn examples_datetime(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 10]), "一昨日", "二千十三年二月十日", "前の日曜日", "先週の日曜日");
    example!(v, check_moment!(c, [2013, 2, 11]), "昨日", "前の日", "前日");
    example!(v, check_moment!(c, [2013, 2, 13]), "明日", "次の日", "二千十三年二月十三日", "今週の水曜日", "バレンタインデーの前の日"); 
    example!(v, check_moment!(c, [2013, 2, 20]), "次の水曜日");
    example!(v, check_moment!(c, [2013, 2, 14]), "二千十三年二月十四日", "バレンタインデーの日", "今週の木曜日");
    example!(v, check_moment!(c, [2013, 2, 14]), "次の木曜日");
    example!(v, check_moment!(c, [2013, 2, 15]), "二千十三年二月十五日", "今週の金曜日");
    example!(v, check_moment!(c, [2013, 2, 15]), "次の金曜日");
    example!(v, check_moment!(c, [2013, 2, 16]), "今週の土曜日", "二千十三年二月十六日", "二月十六日");
    example!(v, check_moment!(c, [2013, 2, 16]), "次の土曜日");
    example!(v, check_moment!(c, [2013, 2, 17]), "二千十三年二月十七日", "二月十七日", "今週の日曜日");
    example!(v, check_moment!(c, [2013, 2, 17]), "次の日曜日");
    example!(v, check_moment!(c, [2013, 2, 18]), "二千十三年二月十八日", "二月十八日", "来週の月曜日");
    example!(v, check_moment!(c, [2013, 2, 18]), "次の月曜日");
    example!(v, check_moment!(c, [2013, 2, 19]), "二千十三年二月十九日", "二月十九日", "一週間後", "次の火曜日", "来週の火曜日");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "午後三時十五分", "十五時十五分");
    example!(v, check_moment!(c, [2013, 2, 12, 13, 30]), "十三時三十分", "十三時半", "午後一時半", "午後一時三十分");
    example!(v, check_moment!(c, [2014, 1, 1]), "元旦", "元日", "二千十四年一月一日");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "現在", "今", "今すぐ", "今すぐに", "只今", "ただいま");
    example!(v, check_moment!(c, [2013, 12, 23]), "二千十三年十二月二十三日", "十二月二十三日", "天皇誕生日", "クリスマスイブの前の日", "クリスマスイブの前日");
    example!(v, check_moment!(c, [2013, 2, 12, 14, 15]), "十四時十五分", "午後二時十五分");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 45]), "午前五時四十五分");
    example!(v, check_moment_span!(c, [2013, 2, 16, 00], [2013, 2, 18, 00]), "週末", "今週末");
    example!(v, check_moment!(c, [2012]), "去年", "昨年", "前年", "前の年");
    example!(v, check_moment!(c, [2013]), "今年", "当年");
    example!(v, check_moment!(c, [2014]), "来年", "新年");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 12]), "朝", "午前", "今朝");
    example!(v, check_moment!(c, [2013, 12, 25]), "クリスマス", "次のクリスマス",  "二千十三年十二月二十五日", "十二月二十五日");
    example!(v, check_moment_span!(c, [2013, 2, 11, 18], [2013, 2, 12, 3]), "昨晚");
    example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 3]), "今晚", "今夜");
    example!(v, check_moment!(c, [2013, 8, 1]), "二千十三年八月一日", "八月一日");
    example!(v, check_moment!(c, [2013, 6, 1]), "二千十三年六月一日", "六月一日");
    example!(v, check_moment!(c, [2013, 8, 11]), "山の日");
    example!(v, check_moment!(c, [2017, 8, 22]), "二千十七年八月二十二日");
    example!(v, check_moment!(c, [2013, 2, 12]), "今日", "当日");
    example!(v, check_moment_span!(c, [2013, 2, 18, 4], [2013, 2, 18, 12]), "月曜日の朝", "次の月曜日の朝", "来週の月曜日の朝", "月曜日の午前中", "次の月曜日の午前中", "来週の月曜日の午前中");
    example!(v, check_moment!(c, [2013, 10, 7]), "二千十三年十月七日");
    example!(v, check_moment!(c, [2013, 2, 5]), "二千十三年二月五日", "一週間前", "前の火曜日", "先週の火曜日");
    example!(v, check_moment!(c, [2013, 3, 1]), "二千十三年三月一日");
    example!(v, check_moment!(c, [2015, 3, 3]), "二千十五年三月三日");
    example!(v, check_moment!(c, [2013, 2, 15]), "二千十三年二月十五日" , "バレンタインデーの次の日");
    example!(v, check_moment_span!(c, [2013, 2, 15, 4], [2013, 2, 15, 12]), "二千十三年二月十五日四時から十二時まで", "三日後の朝");
    example!(v, check_moment!(c, [2013, 2, 19]), "二千十三年二月十九日");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "今週");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "先週", "前の週");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "来週", "次の週");
    example!(v, check_moment!(c, [2013, 1]), "二千十三年一月", "先月", "前の月");
    example!(v, check_moment!(c, [2013, 3]), "二千十三年三月", "来月", "次の月");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 0]), "過去二秒間");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 1], [2013, 2, 12, 4, 30, 4]),  "次の三秒間");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "過去二分間");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "次の三分間");
    example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 8]), "早朝", "明け方");
    example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "過去二日間");
    example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "明日から三日間", "次の三日間");
    example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11]), "過去二週間");
    example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11]), "次の月曜日から三週間", "次の三週間");
    example!(v, check_moment_span!(c, [2013, 12, 1], [2014, 3, 1]), "今年の十二月から来年の二月まで");
    example!(v, check_moment_span!(c, [2013, 3, 1], [2013, 7, 1]), "今年の三月から六月まで", "三月から六月まで" );
    example!(v, check_moment_span!(c, [2011], [2013]), "過去二年間");
    example!(v, check_moment_span!(c, [2014], [2017]), "次の三年間");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "二千十三年二月十二日午後三時", "今日の十五時");
    example!(v, check_moment!(c, [2015, 4, 14]), "二千十五年四月十四日");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "今晚八時" , "今夜八時", "今日二十時", "今日の午後八時");
    example!(v, check_moment!(c, [2013, 3, 8]), "女性の日");
    example!(v, check_moment!(c, [2013, 2, 15]), "三日後");
    example!(v, check_moment!(c, [2013, 2, 9]), "三日前");
    example!(v, check_moment!(c, [2018, 1, 23]), "2018.1.23");
    // TODO: date as "2018年1月23日"

    example!(v, check_moment_span!(c, [2013, 8, 1], [2013, 8, 11]), "八月の上旬");
    example!(v, check_moment_span!(c, [2013, 10, 1], [2013, 10, 11]), "十月の上旬");
    example!(v, check_moment_span!(c, [2013, 3, 1], [2013, 3, 4]), "来月の始め");
    example!(v, check_moment_span!(c, [2013, 1, 10], [2013, 1, 21]), "先月の半ば");
    example!(v, check_moment_span!(c, [2013, 1, 1], [2013, 1, 11]), "先月の上旬");
    example!(v, check_moment_span!(c, [2013, 3, 1], [2013, 3, 4]), "来月の始め");
    example!(v, check_moment_span!(c, [2013, 3, 10], [2013, 3, 21]), "来月の半ば");
    example!(v, check_moment_span!(c, [2013, 8, 29], [2013, 9, 1]), "八月末");
    example!(v, check_moment_span!(c, [2013, 1, 29], [2013, 2, 1]), "先月末");
    example!(v, check_moment_span!(c, [2013, 3, 29], [2013, 4, 1]), "来月末");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "午後八時", "夜の八時");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "午後三時");
    example!(v, check_moment!(c, [2013, 2, 13, 10]), "明日の朝十時");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "昼の三時");
    example!(v, check_moment!(c, [2013, 2, 13, 10]), "明朝十時");
    example!(v, check_moment!(c,  [2013, 2, 12, 17]), "午後五時");
    example!(v, check_moment_span!(c, [2013, 2, 15], [2013, 2, 21]), "十五日から二十日まで");
    example!(v, check_moment_span!(c, [2013, 3, 1], [2013, 3, 6]), "一日から五日", "一日から五日まで");
    example!(v, check_moment_span!(c, [2013, 2, 28], [2013, 3, 4]), "二十八日から三日", "二十八日から三日まで");
    example!(v, check_moment!(c, [2013, 2, 10, 14, 57]), "一昨日の午後三時三分前");
}