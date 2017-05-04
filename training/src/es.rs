use super::*;

pub fn examples_numbers() -> Vec<::rustling::train::Example<Dimension>> {
    let mut v = vec![];
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
    v
}
