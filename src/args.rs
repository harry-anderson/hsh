pub fn parse_input(input: &str) -> Result<Vec<String>, &str> {
    let mut parsed = vec![];
    let args = &mut input.chars();

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
                let rest: String = args.take_while(|ch| ch != &' ').collect();
                let rest = format!("{ch}{rest}");
                parsed.push(rest);
            }
        }
    }

    Ok(parsed)
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
        let mut res = parse_input("print 'paired'").unwrap().into_iter();
        let cmd = res.next().unwrap();
        let args = res.collect::<Vec<String>>();
        assert_eq!(
            (cmd, args),
            (String::from("print"), vec![String::from("paired")])
        );

        let mut res = parse_input(r#"print 'paired'"#).unwrap().into_iter();
        let cmd = res.next().unwrap();
        let args = res.collect::<Vec<String>>();
        assert_eq!(
            (cmd, args),
            (String::from("print"), vec![String::from("paired")])
        );

        // from the task
        let mut res = parse_input(r#"/usr/bin/printf "The cat's name is %s.\n" 'Theodore Roosevelt'"#)
            .unwrap()
            .into_iter();
        let cmd = res.next().unwrap();
        let args = res.collect::<Vec<String>>();
        assert_eq!(
            (cmd, args),
            (
                String::from("/usr/bin/printf"),
                vec![
                    String::from("The cat's name is %s.\\n"),
                    String::from("Theodore Roosevelt")
                ]
            )
        );
        assert_eq!(
            parse_input(r#"/usr/bin/printf "Missing quote"#),
            Err("error: mismatched quotes")
        );
    }
}
