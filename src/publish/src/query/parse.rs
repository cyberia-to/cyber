/// AST for Logseq simple query expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum QueryExpr {
    /// [[tag]] or (page-tags [[tag]])
    Tag(String),
    /// (and expr1 expr2 ...)
    And(Vec<QueryExpr>),
    /// (or expr1 expr2 ...)
    Or(Vec<QueryExpr>),
    /// (not expr)
    Not(Box<QueryExpr>),
    /// (property :key) or (property :key "value") or (page-property :key "value")
    Property {
        key: String,
        value: Option<String>,
    },
    /// (namespace [[ns]])
    Namespace(String),
    /// (page [[name]]) — matches a specific page
    Page(String),
    /// "text" — full-text search in page content
    TextSearch(String),
}

/// Parse a Logseq simple query string into a QueryExpr.
/// Returns None if the query can't be parsed (falls back to styled display).
pub fn parse_query(input: &str) -> Option<QueryExpr> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Try S-expression parse first
    if let Some((expr, _)) = parse_expr(trimmed) {
        return Some(expr);
    }

    // Fallback: try [[tag]] AND/OR [[tag]] shorthand
    parse_and_or_shorthand(trimmed)
}

fn parse_expr(input: &str) -> Option<(QueryExpr, &str)> {
    let input = input.trim();

    if input.is_empty() {
        return None;
    }

    // S-expression: (operator ...)
    if input.starts_with('(') {
        return parse_sexp(input);
    }

    // [[wikilink]] — treated as tag filter
    if input.starts_with("[[") {
        if let Some(end) = input.find("]]") {
            let tag = &input[2..end];
            let rest = &input[end + 2..];
            return Some((QueryExpr::Tag(tag.to_string()), rest));
        }
    }

    // "quoted string" — text search
    if input.starts_with('"') {
        if let Some(end) = input[1..].find('"') {
            let text = &input[1..1 + end];
            let rest = &input[2 + end..];
            return Some((QueryExpr::TextSearch(text.to_string()), rest));
        }
    }

    // Bare word — could be part of "[[tag]] AND [[tag]]" syntax
    if input.starts_with("AND") || input.starts_with("and") {
        let rest = input[3..].trim();
        return parse_expr(rest);
    }
    if input.starts_with("OR") || input.starts_with("or") {
        let rest = input[2..].trim();
        return parse_expr(rest);
    }

    // Try to parse as bare [[tag1]] AND/OR [[tag2]] pattern
    None
}

fn parse_sexp(input: &str) -> Option<(QueryExpr, &str)> {
    let input = input.trim();
    if !input.starts_with('(') {
        return None;
    }

    let inner = &input[1..]; // skip '('

    // Find the operator
    let inner = inner.trim();
    let (op, rest) = split_token(inner)?;

    match op {
        "and" => {
            let (args, rest) = parse_args(rest)?;
            // Filter out empty/placeholder args
            let args: Vec<_> = args.into_iter().filter(|a| *a != QueryExpr::And(vec![])).collect();
            if args.is_empty() {
                Some((QueryExpr::And(vec![]), rest))
            } else if args.len() == 1 {
                Some((args.into_iter().next().unwrap(), rest))
            } else {
                Some((QueryExpr::And(args), rest))
            }
        }
        "or" => {
            let (args, rest) = parse_args(rest)?;
            if args.len() == 1 {
                Some((args.into_iter().next().unwrap(), rest))
            } else {
                Some((QueryExpr::Or(args), rest))
            }
        }
        "not" => {
            let (arg, rest) = parse_one_arg(rest)?;
            Some((QueryExpr::Not(Box::new(arg)), rest))
        }
        "page-tags" => {
            let (tag, rest) = parse_wikilink_arg(rest)?;
            Some((QueryExpr::Tag(tag), rest))
        }
        "property" | "page-property" => {
            let (key, value, rest) = parse_property_args(rest)?;
            Some((QueryExpr::Property { key, value }, rest))
        }
        "namespace" => {
            let (ns, rest) = parse_wikilink_arg(rest)?;
            Some((QueryExpr::Namespace(ns), rest))
        }
        "page" => {
            let (name, rest) = parse_wikilink_arg(rest)?;
            Some((QueryExpr::Page(name), rest))
        }
        _ => {
            // Unknown operator — try to skip to closing paren
            skip_to_close_paren(input)
                .map(|rest| (QueryExpr::And(vec![]), rest))
        }
    }
}

/// Parse multiple arguments until we hit a closing ')'.
fn parse_args(input: &str) -> Option<(Vec<QueryExpr>, &str)> {
    let mut args = Vec::new();
    let mut rest = input.trim();

    while !rest.is_empty() && !rest.starts_with(')') {
        if let Some((expr, new_rest)) = parse_expr(rest) {
            args.push(expr);
            rest = new_rest.trim();
        } else {
            // Skip unrecognized token
            let (_, new_rest) = skip_one_token(rest);
            rest = new_rest.trim();
        }
    }

    // Consume closing ')'
    if rest.starts_with(')') {
        rest = &rest[1..];
    }

    Some((args, rest))
}

/// Parse a single argument and consume closing ')'.
fn parse_one_arg(input: &str) -> Option<(QueryExpr, &str)> {
    let rest = input.trim();
    let (expr, rest) = parse_expr(rest)?;
    let rest = rest.trim();
    let rest = if rest.starts_with(')') {
        &rest[1..]
    } else {
        rest
    };
    Some((expr, rest))
}

/// Parse a [[wikilink]] argument and consume closing ')'.
fn parse_wikilink_arg(input: &str) -> Option<(String, &str)> {
    let rest = input.trim();

    if rest.starts_with("[[") {
        if let Some(end) = rest.find("]]") {
            let name = rest[2..end].to_string();
            let rest = rest[end + 2..].trim();
            let rest = if rest.starts_with(')') {
                &rest[1..]
            } else {
                rest
            };
            return Some((name, rest));
        }
    }

    // Try without [[ ]]
    let (token, rest) = split_token(rest)?;
    let rest = rest.trim();
    let rest = if rest.starts_with(')') {
        &rest[1..]
    } else {
        rest
    };
    Some((token.to_string(), rest))
}

/// Parse property arguments: :key or :key "value" or :key value
fn parse_property_args(input: &str) -> Option<(String, Option<String>, &str)> {
    let rest = input.trim();

    // Parse key (with or without leading :)
    let (key_token, rest) = split_token(rest)?;
    let key = key_token.trim_start_matches(':').to_string();
    let rest = rest.trim();

    // Check for value
    if rest.starts_with(')') {
        // No value — existence check
        return Some((key, None, &rest[1..]));
    }

    // Parse value
    if rest.starts_with('"') {
        // Quoted value
        if let Some(end) = rest[1..].find('"') {
            let value = rest[1..1 + end].to_string();
            let rest = rest[2 + end..].trim();
            let rest = if rest.starts_with(')') {
                &rest[1..]
            } else {
                rest
            };
            return Some((key, Some(value), rest));
        }
    }

    // Unquoted value
    let (val_token, rest) = split_token(rest)?;
    let rest = rest.trim();
    let rest = if rest.starts_with(')') {
        &rest[1..]
    } else {
        rest
    };
    Some((key, Some(val_token.to_string()), rest))
}

/// Split off the first whitespace-delimited token.
fn split_token(input: &str) -> Option<(&str, &str)> {
    let input = input.trim();
    if input.is_empty() || input.starts_with(')') {
        return None;
    }

    // Handle special cases
    if input.starts_with('(') || input.starts_with('[') || input.starts_with('"') {
        return None;
    }

    let end = input
        .find(|c: char| c.is_whitespace() || c == ')' || c == '(' || c == '[')
        .unwrap_or(input.len());
    if end == 0 {
        return None;
    }
    Some((&input[..end], &input[end..]))
}

/// Skip one unrecognized token or balanced s-expression.
fn skip_one_token(input: &str) -> (&str, &str) {
    let input = input.trim();
    if input.is_empty() || input.starts_with(')') {
        return ("", input);
    }

    if input.starts_with('(') {
        // Skip balanced parens
        if let Some(rest) = skip_to_close_paren(input) {
            return (&input[..input.len() - rest.len()], rest);
        }
    }

    if input.starts_with("[[") {
        if let Some(end) = input.find("]]") {
            return (&input[..end + 2], &input[end + 2..]);
        }
    }

    if input.starts_with('"') {
        if let Some(end) = input[1..].find('"') {
            return (&input[..end + 2], &input[end + 2..]);
        }
    }

    // Skip to next whitespace or special char
    let end = input
        .find(|c: char| c.is_whitespace() || c == ')' || c == '(')
        .unwrap_or(input.len());
    (&input[..end], &input[end..])
}

fn skip_to_close_paren(input: &str) -> Option<&str> {
    let mut depth = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&input[i + 1..]);
                }
            }
            _ => {}
        }
    }
    None
}

/// Parse "[[tag1]] AND [[tag2]]" shorthand syntax.
/// Used as fallback when S-expression parse fails.
fn parse_and_or_shorthand(input: &str) -> Option<QueryExpr> {
    let mut exprs = Vec::new();
    let mut is_or = false;

    let full = input.trim();

    // Simple check: does it look like [[a]] AND [[b]] ?
    if full.contains(" AND ") || full.contains(" and ") {
        for part in full.split(|c: char| c == ' ') {
            let part = part.trim();
            if part.eq_ignore_ascii_case("AND") {
                continue;
            }
            if part.starts_with("[[") && part.ends_with("]]") {
                let tag = &part[2..part.len() - 2];
                exprs.push(QueryExpr::Tag(tag.to_string()));
            }
        }
        if !exprs.is_empty() {
            return Some(QueryExpr::And(exprs));
        }
    }

    if full.contains(" OR ") || full.contains(" or ") {
        for part in full.split(|c: char| c == ' ') {
            let part = part.trim();
            if part.eq_ignore_ascii_case("OR") {
                is_or = true;
                continue;
            }
            if part.starts_with("[[") && part.ends_with("]]") {
                let tag = &part[2..part.len() - 2];
                exprs.push(QueryExpr::Tag(tag.to_string()));
            }
        }
        if !exprs.is_empty() {
            return Some(if is_or {
                QueryExpr::Or(exprs)
            } else {
                QueryExpr::And(exprs)
            });
        }
    }

    // Single [[tag]]
    if full.starts_with("[[") && full.ends_with("]]") && !full[2..full.len() - 2].contains("]]") {
        let tag = &full[2..full.len() - 2];
        return Some(QueryExpr::Tag(tag.to_string()));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_tag() {
        let expr = parse_query("(page-tags [[research]])").unwrap();
        assert_eq!(expr, QueryExpr::Tag("research".to_string()));
    }

    #[test]
    fn test_parse_property_existence() {
        let expr = parse_query("(property :nitrogener)").unwrap();
        assert_eq!(
            expr,
            QueryExpr::Property {
                key: "nitrogener".to_string(),
                value: None
            }
        );
    }

    #[test]
    fn test_parse_property_value() {
        let expr = parse_query("(property :supply \"yes\")").unwrap();
        assert_eq!(
            expr,
            QueryExpr::Property {
                key: "supply".to_string(),
                value: Some("yes".to_string())
            }
        );
    }

    #[test]
    fn test_parse_and() {
        let expr = parse_query("(and (page-tags [[major]]) (page-tags [[research]]))").unwrap();
        match expr {
            QueryExpr::And(args) => {
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], QueryExpr::Tag("major".to_string()));
                assert_eq!(args[1], QueryExpr::Tag("research".to_string()));
            }
            _ => panic!("Expected And"),
        }
    }

    #[test]
    fn test_parse_and_with_not() {
        let expr = parse_query("(and (page-tags [[major]]) (not (page-tags [[research]])))").unwrap();
        match expr {
            QueryExpr::And(args) => {
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], QueryExpr::Tag("major".to_string()));
                match &args[1] {
                    QueryExpr::Not(inner) => {
                        assert_eq!(**inner, QueryExpr::Tag("research".to_string()));
                    }
                    _ => panic!("Expected Not"),
                }
            }
            _ => panic!("Expected And"),
        }
    }

    #[test]
    fn test_parse_or_namespace() {
        let expr = parse_query("(or (page-tags [[edem]]) (namespace [[edem]]))").unwrap();
        match expr {
            QueryExpr::Or(args) => {
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], QueryExpr::Tag("edem".to_string()));
                assert_eq!(args[1], QueryExpr::Namespace("edem".to_string()));
            }
            _ => panic!("Expected Or"),
        }
    }

    #[test]
    fn test_parse_page_property() {
        let expr = parse_query("(page-property :type \"public\")").unwrap();
        assert_eq!(
            expr,
            QueryExpr::Property {
                key: "type".to_string(),
                value: Some("public".to_string())
            }
        );
    }
}
