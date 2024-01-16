#[derive(Debug, Clone, PartialEq)]
enum Keyword {
    NUM_CHAR(char),

    NUM(f64),

    OP(char),
    IF(String),
    ELSE(String),
    NULL(usize),
}

fn raw_kw(s: &str) -> &str {
    if [
        "if", "else", "+", "-", "*", "/",
    ].contains(&s) {
        println!("{} is a keyword", s);
        s
    } else if is_num(s.to_string()) {
        "num"
    } else {
        "null"
    }
}

fn kw_from_str(s: String) -> Keyword {
    let kw_name = raw_kw(&s);
    match kw_name {
        "if" => Keyword::IF(String::from("if")),
        "else" => Keyword::ELSE(String::from("else")),
        "num" => Keyword::NUM_CHAR(s.chars().nth(0).unwrap()),
        "+" | "-" | "*" | "/" => Keyword::OP(kw_name.chars().nth(0).unwrap()),
        _ => Keyword::NULL(0),
    }
}

fn is_keyword(s: String) -> (bool, Keyword) {
    let x = kw_from_str(s);
    match x {
        Keyword::NULL(_) => (false, x),
        _ => (true, x),
    }
}

fn is_num(s: String) -> bool {
    s.parse::<f64>().is_ok() || s == "."
}

fn action_with_cur(cur: &mut String, list: &mut Vec<Keyword>) {
    if !cur.is_empty() {
        let kw_result = is_keyword(cur.clone());
        if kw_result.0 {
            list.push(kw_result.1);
        } else {
            list.push(Keyword::NULL(0));
        }
        cur.clear();
    }
}

fn parse(inp: String) -> Vec<Keyword> {
    let mut cur = String::new();
    let mut list: Vec<Keyword> = Vec::new();

    let input = inp.replace(" ", "");

    println!("input: {}", input);

    let mut iter = input.chars().peekable();
    while let Some(c) = iter.next() {
        cur.push(c);
        action_with_cur(&mut cur, &mut list);
        
    }

    action_with_cur(&mut cur, &mut list);

    let mut new_list: Vec<Keyword> = Vec::new();


    let mut cur_num_list: Vec<Keyword> = Vec::new();

    for kw in list {
        match kw {
            Keyword::NUM_CHAR(c) => {
                cur_num_list.push(Keyword::NUM_CHAR(c));
            },
            _ => {
                if !cur_num_list.is_empty() {
                    println!("cur_num_list: {:?}", cur_num_list);

                    let mut num_str = String::new();
                    for num_char in cur_num_list.clone() {
                        match num_char {
                            Keyword::NUM_CHAR(c) => {
                                num_str.push(c);
                            },
                            _ => {},
                        }
                    }
                    new_list.push(Keyword::NUM(num_str.parse::<f64>().unwrap()));
                    cur_num_list.clear();
                }
                if kw != Keyword::NULL(0) {
                    new_list.push(kw);
                }
            },
        }
    }

    if !cur_num_list.is_empty() {
        println!("cur_num_list: {:?}", cur_num_list);

        let mut num_str = String::new();
        for num_char in cur_num_list.clone() {
            match num_char {
                Keyword::NUM_CHAR(c) => {
                    num_str.push(c);
                },
                _ => {},
            }
        }
        new_list.push(Keyword::NUM(num_str.parse::<f64>().unwrap()));
        cur_num_list.clear();
    }

    new_list
}

fn main() {
    let input = String::from("12* 3.4 + 5.6 / 7.8");
    let list = parse(input);

    println!("{:?}", list);
}