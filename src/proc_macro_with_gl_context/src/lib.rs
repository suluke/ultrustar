use std::{iter::Peekable, str::FromStr};
type Result<T> = std::result::Result<T, &'static str>;

use proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree};

fn read_attrs<T>(iter: &mut Peekable<T>) -> Result<TokenStream>
where
    T: Iterator<Item = TokenTree>,
{
    let mut output = TokenStream::new();
    loop {
        if let Some(TokenTree::Punct(punct)) = iter.peek() {
            if punct.as_char() != '#' {
                break;
            }
        } else {
            break;
        }
        output.extend(iter.next());
        if let Some(TokenTree::Group(group)) = iter.peek() {
            if group.delimiter() != Delimiter::Bracket {
                return Err("Group belonging to attr should be delimited by brackets");
            }
            output.extend(iter.next());
        } else {
            return Err("Expected group while reading attribute");
        }
    }
    Ok(output)
}

fn read_visibility<T>(iter: &mut Peekable<T>) -> Result<TokenStream>
where
    T: Iterator<Item = TokenTree>,
{
    let mut output = TokenStream::new();
    if let Some(TokenTree::Ident(ident)) = iter.peek() {
        if "pub" == ident.to_string() {
            output.extend(iter.next());
        }
    }
    Ok(output)
}

fn read_type<T>(iter: &mut Peekable<T>) -> Result<TokenStream>
where
    T: Iterator<Item = TokenTree>,
{
    let mut output = TokenStream::new();
    if let Some(TokenTree::Ident(_)) = iter.peek() {
        output.extend(iter.next());
    } else {
        return Err("Expected type identifier");
    }
    if let Some(TokenTree::Punct(punct)) = iter.peek() {
        if punct.as_char() == '<' {
            output.extend(iter.next());
            if let Some(TokenTree::Ident(_)) = iter.peek() {
                output.extend(iter.next());
            } else {
                return Err("Expected type argument identifier");
            }
            if let Some(TokenTree::Punct(punct)) = iter.peek() {
                if punct.as_char() == '>' {
                    output.extend(iter.next());
                } else {
                    return Err("Expected type argument list end");
                }
            } else {
                return Err("Expected type argument list end");
            }
        }
    }
    Ok(output)
}

fn read_func_sig<T>(iter: &mut Peekable<T>) -> Result<TokenStream>
where
    T: Iterator<Item = TokenTree>,
{
    let mut output = TokenStream::new();
    let mut popped = false;
    if let Ok(vis) = read_visibility(iter) {
        popped = !vis.is_empty();
        output.extend(vis);
    }
    loop {
        if let Some(TokenTree::Ident(ident)) = iter.peek() {
            if ["unsafe", "const", "extern", "async"].contains(&ident.to_string().as_str()) {
                output.extend(iter.next());
                popped = true;
                continue;
            }
        }
        break;
    }
    if let Some(TokenTree::Ident(ident)) = iter.peek() {
        if ident.to_string().as_str() == "fn" {
            output.extend(iter.next());
        } else if popped {
            return Err("Expected fn");
        } else {
            return Ok(output);
        }
    }
    if let Some(TokenTree::Ident(_)) = iter.peek() {
        output.extend(iter.next());
    } else {
        return Err("Expected function name");
    }
    if let Some(TokenTree::Group(group)) = iter.peek() {
        if group.delimiter() != Delimiter::Parenthesis {
            return Err("Expected function parameters");
        }
        output.extend(iter.next());
    }
    if let Some(TokenTree::Punct(punct)) = iter.peek() {
        if punct.as_char() == '-' {
            output.extend(iter.next());
            if let Some(TokenTree::Punct(punct)) = iter.peek() {
                if punct.as_char() != '>' {
                    return Err("Expected >");
                }
                output.extend(iter.next());
            } else {
                return Err("Expected >");
            }
            output.extend(read_type(iter)?);
        }
    }
    Ok(output)
}

fn parse_macro_params<T>(iter: &mut T) -> Result<(Ident, Ident)>
where
    T: Iterator<Item = TokenTree>,
{
    let thread_local = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        _ => {
            return Err("Expected identifier");
        }
    };
    match iter.next() {
        Some(TokenTree::Ident(ident)) => {
            if &ident.to_string() != "as" {
                return Err("Expected 'as'");
            }
        }
        _ => {
            return Err("Expected 'as'");
        }
    }
    let binding = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        _ => {
            return Err("Expected identifier");
        }
    };
    Ok((thread_local, binding))
}

fn patch(group: Group, thread_local: Ident, binding: Ident) -> Group {
    let delim = group.delimiter();
    let mut stream = TokenStream::new();
    stream.extend(Some(TokenTree::Ident(thread_local)));
    stream.extend(TokenStream::from_str(&format!(
        ".with(|{binding}| {{
            let scope = {binding}.borrow();
            let {binding} = scope
            .as_ref()
            .expect(\"WebGlRenderingContext not set for current thread\");
            {group}
        }})",
        binding = binding.to_string(),
        group = group.to_string()
    )));
    Group::new(delim, stream)
}

#[proc_macro_attribute]
pub fn with_gl_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    (|| -> Result<TokenStream> {
        let (thread_local, binding) = parse_macro_params(&mut attr.into_iter())?;

        let mut output = TokenStream::new();
        let mut tokens = item.into_iter().peekable();
        output.extend(read_attrs(&mut tokens)?);
        output.extend(read_func_sig(&mut tokens)?);
        if let Some(TokenTree::Group(group)) = tokens.peek() {
            if group.delimiter() != Delimiter::Brace {
                return Err("Expected function body in braces");
            }
        } else {
            return Err("Expected function body");
        }
        let group = match tokens.next() {
            Some(TokenTree::Group(group)) => group,
            _ => {
                return Err("Failed next after successful peel");
            }
        };
        let group = TokenTree::Group(patch(group, thread_local, binding));
        output.extend(TokenStream::from(group));
        let remaining = TokenStream::from_iter(tokens);
        output.extend(remaining);

        Ok(output)
    })()
    .unwrap()
}
