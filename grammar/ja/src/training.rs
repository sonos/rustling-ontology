use rustling_ontology_values::check::*;
//use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
//use rustling_ontology_values::ResolverContext;

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(5.0, Some("celcius")), "五度");
    example!(v, check_temperature(25.0, Some("celcius")), "二十五度");
    example!(v, check_temperature(-10.0, Some("celcius")), "マイナス十度");
    example!(v, check_temperature(-10.0, Some("celcius")), "零下十度");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_integer(0), "零");
    example!(v, check_integer(1), "一");
    example!(v, check_integer(2), "二");
    example!(v, check_integer(3), "三");
    example!(v, check_integer(4), "四");
    example!(v, check_integer(5), "五");
    example!(v, check_integer(6), "六");
    example!(v, check_integer(7), "七");
    example!(v, check_integer(8), "八");
    example!(v, check_integer(9), "九");
    example!(v, check_integer(10), "十");
    example!(v, check_integer(20), "二十");
    example!(v, check_integer(30), "三十");
    example!(v, check_integer(40), "四十");
    example!(v, check_integer(90), "九十");
    //example!(v, check_integer(3000000), "3M", "3000K", "3000000", "3,000,000");
    //example!(v, check_integer(1200000), "1200K", "1200.0K", "1.2M", ".0012G", "1,200,000");
    example!(v, check_integer(33), "三十三");
    example!(v, check_integer(96), "九十六");
    example!(v, check_integer(14), "十四");
    example!(v, check_integer(11), "十一");
    example!(v, check_ordinal(7), "七番目");
    example!(v, check_ordinal(11), "十一番目");
    example!(v, check_ordinal(91), "九十一番目");
}
