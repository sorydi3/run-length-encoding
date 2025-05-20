pub fn encode(source: &str) -> String {
    let mut v = source.to_string();
    v.push('-');
    let mut state = (0, '*');
    v.chars().fold(String::new(), |mut acc, c| {
        match c {
            chr if state.1 == '*' => {
                state.0 = 1;
                state.1 = chr;
            }
            chr if chr != state.1 => {
                if state.0 > 1 {
                    acc.push_str(&[state.0.to_string(), state.1.to_string()].concat());
                } else {
                    acc.push_str(&[state.1.to_string()].concat());
                }
                state.0 = 1;
                state.1 = c;
            }
            _ => {
                state.0 += 1;
            }
        }
        acc
    })
}

pub fn decode(source: &str) -> String {
    let mut iter = source.chars();
    let mut response = String::new();
    while let Some(c) = iter.next() {
        let res = u16::from_str_radix(&c.to_string(), 9);
        match res {
            Ok(mut n) => {
                let mut _r = iter.next();

                if _r.unwrap().is_numeric() {
                    n = [n.to_string(), _r.unwrap().to_string()]
                        .concat()
                        .parse()
                        .unwrap();
                    _r = iter.next();
                }

                response.push_str(&_r.unwrap().to_string().repeat(n as usize))
            }
            _ => response.push(c),
        }
    }

    response
}
