use super::*;
use rustling_ontology_values::check::*;


pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "0", "〇", "零", "零个", "0个");
    example!(v, check_integer(1), "1", "一", "一个", "1个");
    example!(v, check_integer(2), "2", "二", "两", "兩", "二個", "二个");
    example!(v, check_integer(3), "3", "三");
    example!(v, check_integer(4), "4", "四");
    example!(v, check_integer(5), "5", "五");
    example!(v, check_integer(6), "6", "六");
    example!(v, check_integer(7), "7", "七");
    example!(v, check_integer(8), "8", "八");
    example!(v, check_integer(9), "9", "九");
    example!(v, check_integer(10), "10", "十");
    example!(v, check_float(1.1), "1.1", "1.10", "01.10");
    example!(v, check_float(0.77), "0.77", ".77");
    example!(v, check_integer(20), "20", "二十");
    example!(v, check_integer(30), "30", "三十");
    example!(v, check_integer(40), "40", "四十");
    example!(v, check_integer(90), "90", "九十");
    example!(v, check_integer(3000000), "3M", "3000K", "3000000", "3,000,000");
    example!(v, check_integer(1200000), "1200K", "1200.0K", "1.2M", ".0012G", "1,200,000");
    example!(v, check_float(1200000.0), "1,200,000.00");
    example!(v, check_integer(33), "33", "三十三");
    example!(v, check_integer(96), "96", "九十六");
    example!(v, check_integer(14), "14", "十四");
    example!(v, check_integer(11), "11", "十一");
    example!(v,
             check_integer(-1200000),
             "- 1,200,000",
             "-1200000",
             "负1,200,000",
             "负 1,200,000",
             "負 1,200,000",
             "负1200000",
             "负 1200000",
             "-1.2M",
             "-1200K",
             "-.0012G");
}
