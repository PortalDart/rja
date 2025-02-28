use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Napaka" => "Err",
        "Vredu" => "Ok",
        "NizZnakov" => "String",
        "KartaPovzetkov" => "HashMap",
        "Privzeto" => "Default",
        "Napaka" => "Error",
        "Opcijsko" => "Option",
        "Nekaj" => "Some",
        "Noben" => "None",
        "Izid" => "Result",
        "Bistvo" => "Self",
        "izpiši" => "println",
        "prekini" => "break",
        "asinhron" => "async",
        "počakaj" => "await",
        "zanka" => "loop",
        "selitev" => "move",
        "skrinja" => "crate",
        "nedosegljiva_koda" => "unreachable_code",
        "kot" => "as",
        "stalno" => "const",
        "lastnost" => "trait",
        "nevaren" => "unsafe",
        "v" => "in",
        "iz" => "from",
        "dinamican" => "dyn",
        "odvij" => "unwrap",
        "privzeto" => "default",
        "kot_ref" => "as_ref",
        "vhodizhod" => "io", // kot vhod/izhod
        "zunanji" => "extern",
        "lažno" => "false",
        "funkcija" => "fn",
        "nad" => "super",
        "vstavi" => "insert",
        "pridobi" => "get",
        "dovoli" => "allow",
        "panika" | "razpad" | "izpad" => "panic",
        "modul" => "mod",
        "spremenljiv" => "mut",
        "nov" => "new",
        "kjer" => "where",
        "za" => "for",
        "pridobi_ali_vstavi_z" => "get_or_insert_with",
        "glavni" => "main",
        "javni" => "pub",
        "kaj" => None?,
        "vrni" => "return",
        "izvedi" => "impl",
        "sklic" => "ref",
        "ujem" => "match",
        "če" => "if",
        "sicer" => "else",
        "sebi" => "self",
        "pusti" => "let",
        "statično" => "static",
        "struktura" => "struct",
        "pričakaj" => "expect",
        "dokler" => "while",
        "uporabi" => "use",
        "v_obliko" => "into",
        "resnično" => "true",
        "enumeracija" => "enum",
        "Skupina" => "Group",
        "Oznaka" => "Ident",
        "TokŽetonov" => "TokenStream",
        "DrevoŽetonov" => "TokenTree",
        "v_niz_znakov" => "to_string",
        "kot_niz" => "as_str",
        "obseg" => "span",
        "Vektor" => "Vec",
        "pretok" => "stream",
        "potisni" => "push",
        "razširi" => "extend",
        "ločilo" => "delimiter",
        "Pika" => "Punct",
        "Doslovni" => "Literal",
        "makro_postopek" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn hrđa(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
