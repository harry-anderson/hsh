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
                let Some(pair_idx) = args.clone().position(|ch| ch == '"') else {
                    return Err("error: mismatched quotes")
                };
                let quoted = args.take(pair_idx).collect::<String>();
                let _closing = args.next();
                parsed.push(quoted)
            }
            '\'' => {
                let Some(pair_idx) = args.clone().position(|ch| ch == '\'') else {
                    return Err("error: mismatched quotes")
                };
                let quoted = args.take(pair_idx).collect::<String>();
                let _closing = args.next();
                parsed.push(quoted)
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
    fn quotes() {
        // missing quote
        assert_eq!(
            parse_input("print 'missing"),
            Err("error: mismatched quotes")
        );
        assert_eq!(
            parse_input(r#"print "missing"#),
            Err("error: mismatched quotes")
        );

        // paired quote
        assert_eq!(
            parse_input("print 'paired'"),
            Ok(("print", vec![String::from("paired")]))
        );
        assert_eq!(
            parse_input(r#"print 'paired'"#),
            Ok(("print", vec![String::from("paired")]))
        );
    }
}
