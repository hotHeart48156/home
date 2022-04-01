pub fn expand_attr(meta: &syn::Meta) -> syn::Result<Vec<Vec<(syn::Ident, syn::token::Colon2)>>> {
    let mut ret: Vec<Vec<(syn::Ident, syn::token::Colon2)>> = vec![];

    if let syn::Meta::List(syn::MetaList {
        ref path,
        ref nested,
        ..
    }) = meta
    {
        if let syn::Path { ref segments, .. } = path {
            let mut path_vec: Vec<(syn::Ident, syn::token::Colon2)> = vec![];
            let _ = segments
                .iter()
                .map(|c| {
                    path_vec.push((c.ident.clone(), syn::token::Colon2::default()));
                    0
                })
                .collect::<Vec<u32>>();

            let _ = nested
                .into_iter()
                .map(|c| {
                    if let syn::NestedMeta::Meta(syn::Meta::Path(s)) = c {
                        // syn::NestedMeta::Meta(syn::NestedMeta::Lit(syn::Lit::Str(s)))
                        // if let syn::NestedMeta::Meta(syn::Meta::Path(s)) = c
                        let mut one_stmi: Vec<(syn::Ident, syn::token::Colon2)> = vec![];
                        one_stmi.extend(path_vec.clone());
                        let name_string = s.get_ident().unwrap().to_string();
                        let name_string_cut_token =
                            name_string.as_str().trim_matches(&['\"', '\''] as &[_]);
                        let init_str = "_init";
                        let name_ident_string = format!("{}{}", name_string_cut_token, init_str);
                        let name_ident_str = name_ident_string.as_str();
                        let name_ident =
                            syn::Ident::new(name_ident_str, proc_macro2::Span::call_site());
                        one_stmi.push((name_ident, syn::token::Colon2::default()));
                        ret.push(one_stmi);
                    } else {
                        eprintln!("{:#?}", c);
                        if let syn::NestedMeta::Lit(syn::Lit::Str(s)) = c {
                            let mut one_stmi: Vec<(syn::Ident, syn::token::Colon2)> = vec![];
                            one_stmi.extend(path_vec.clone());
                            let name_string = s.token().to_string();
                            let name_string_cut_token =
                                name_string.as_str().trim_matches(&['\"', '\''] as &[_]);
                            let init_str = "_init";
                            let name_ident_string =
                                format!("{}{}", name_string_cut_token, init_str);
                            let name_ident_str = name_ident_string.as_str();
                            let name_ident =
                                syn::Ident::new(name_ident_str, proc_macro2::Span::call_site());
                            one_stmi.push((name_ident, syn::token::Colon2::default()));
                            ret.push(one_stmi);
                        }
                    }
                    0
                })
                .collect::<Vec<u32>>();
        } else {
            return syn::Result::Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "not a meta list",
            ));
        }
    } else {
        return syn::Result::Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "not a path",
        ));
    }
    return Ok(ret);
}
pub fn new_stmi(punc: Vec<(syn::Ident, syn::token::Colon2)>) -> syn::Result<syn::Stmt> {
    let attrs: Vec<syn::Attribute> = vec![];
    let attrs_func: Vec<syn::Attribute> = vec![];
    let mut segments: syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> =
        syn::punctuated::Punctuated::new();
    for i in 0..punc.len() - 1 {
        let seg = punc[i].0.clone();
        let col = punc[i].1.clone();
        let arguments = syn::PathArguments::default();
        let path_segment = syn::PathSegment {
            arguments: arguments,
            ident: seg,
        };
        segments.push(path_segment);
        segments.push_punct(col);
    }
    let seg = punc[punc.len() - 1].0.clone();
    let arguments = syn::PathArguments::default();
    let path_segment = syn::PathSegment {
        arguments: arguments,
        ident: seg,
    };
    segments.push(path_segment);

    let func = Box::new(syn::Expr::Path(syn::ExprPath {
        path: syn::Path {
            leading_colon: None,
            segments,
        },
        qself: None,
        attrs: attrs_func,
    }));
    let args = syn::punctuated::Punctuated::default();
    let paren_token = syn::token::Paren::default();
    let call = syn::Expr::Call(syn::ExprCall {
        attrs,
        func,
        paren_token,
        args,
    });
    let semi = syn::Stmt::Semi(call, syn::token::Semi::default());
    return Ok(semi);
}
