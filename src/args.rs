pub fn parse_input(input: &str) -> Result<(&str, Vec<String>), &str> {
    if input.is_empty() {
        return Err("");
    };
    // trim leading and trailing whitespace
    let input = input.trim();
    // command is 0 to first space or end of string
    let idx = input.find(' ').unwrap_or(input.len());
    let command = &input[0..idx];
    // if the command is the total string, we are done
    if idx == input.len() {
        return Ok((command, vec![]));
    }
    // we have args
    let mut parsed = vec![];
    let args = &mut input[idx + 1..].chars();

    while let Some(ch) = args.next() {
        match ch {
            '"' => {
                let rest: Vec<char> = args.take_while(|ch| ch != &'"').collect();
                let _cl = args.next();
                let rest: String = String::from_iter(rest);
                parsed.push(rest);
            }
            '\'' => {
                let rest: Vec<char> = args.take_while(|ch| ch != &'\'').collect();
                let _cl = args.next();
                let rest: String = String::from_iter(rest);
                parsed.push(rest);
            }
            ' ' => {}
            ch => {
                let rest: Vec<char> = args.take_while(|ch| ch != &' ').collect();
                let rest: String = String::from_iter(rest);
                let rest = format!("{ch}{rest}");
                parsed.push(rest);
            }
        }
    }

    Ok((command, parsed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![
            "    ls     ",
            "cd     /abc",
            "echo extra    spaces    will    be    removed",
            r#"printf "The cat's name is %s.\n" 'Theodore Roosevelt'"#,
        ];
        for i in input {
            let x = parse_input(i);
            println!("{:?}", x);
        }
    }
    #[test]
    fn s() {
        let x = parse_input("print 'sdfs");
        println!("{:?}", x);
    }
}
