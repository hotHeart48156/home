pub fn parse_attr(meta: &syn::Meta) -> syn::Result<Vec<Vec<(syn::Ident, syn::token::Colon2)>>> {
    let mut ret: Vec<Vec<(syn::Ident, syn::token::Colon2)>> = vec![];

    if let syn::Meta::List(syn::MetaList {
        ref path,
        ref nested,
        ..
    }) = meta
    {
        let  segments = path.segments.clone();

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
                    let name_string = s.get_ident().unwrap().to_string().to_uppercase();
                    let name_string_cut_token =
                        name_string.as_str().trim_matches(&['\"', '\''] as &[_]);
                    let init_str = "";
                    let name_ident_string = format!("{}{}", name_string_cut_token, init_str);
                    let name_ident_str = name_ident_string.as_str();
                    let name_ident =
                        syn::Ident::new(name_ident_str, proc_macro2::Span::call_site());
                    one_stmi.push((name_ident, syn::token::Colon2::default()));
                    ret.push(one_stmi);
                } else {
                    if let syn::NestedMeta::Lit(syn::Lit::Str(s)) = c {
                        let mut one_stmi: Vec<(syn::Ident, syn::token::Colon2)> = vec![];
                        one_stmi.extend(path_vec.clone());
                        let name_string = s.token().to_string().to_uppercase();
                        let name_string_cut_token =
                            name_string.as_str().trim_matches(&['\"', '\''] as &[_]);
                        let init_str = "";
                        let name_ident_string = format!("{}{}", name_string_cut_token, init_str);
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
            "not a path",
        ));
    }
    return Ok(ret);
}
