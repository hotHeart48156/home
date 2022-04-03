pub fn gen_free_stmt_stmi() -> syn::Stmt {
    let cortex_m_ident = syn::Ident::new("cortex_m", proc_macro2::Span::call_site());
    let cortex_m_path_segment = syn::PathSegment {
        ident: cortex_m_ident,
        arguments: syn::PathArguments::None,
    };
    let colon = syn::token::Colon2::default();
    let interrpute_ident = syn::Ident::new("interrupt", proc_macro2::Span::call_site());
    let interrpute_path_segment = syn::PathSegment {
        ident: interrpute_ident,
        arguments: syn::PathArguments::None,
    };
    let free_ident = syn::Ident::new("free", proc_macro2::Span::call_site());
    let free_path_segment = syn::PathSegment {
        ident: free_ident,
        arguments: syn::PathArguments::None,
    };
    let mut segments: syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> =
        syn::punctuated::Punctuated::default();
    segments.push(cortex_m_path_segment);
    segments.push_punct(colon.clone());
    segments.push(interrpute_path_segment);
    segments.push_punct(colon.clone());
    segments.push(free_path_segment);
    segments.push_punct(colon.clone());

    let expr_path = syn::Path {
        leading_colon: None,
        segments,
    };
    let path = syn::Expr::Path(syn::ExprPath {
        attrs: vec![],
        qself: None,
        path: expr_path,
    });
    let args: syn::punctuated::Punctuated<syn::Expr, syn::token::Comma> =
        syn::punctuated::Punctuated::default();
    let expr = syn::Expr::Call(syn::ExprCall {
        attrs: vec![],
        paren_token: syn::token::Paren::default(),
        args,
        func: Box::new(path),
    });
    let stmi = syn::Stmt::Semi(expr, syn::token::Semi::default());
    stmi
}
pub fn gen_closure_block() {
    let input: syn::punctuated::Punctuated<syn::Pat, syn::token::Comma> =
        syn::punctuated::Punctuated::default();
    let stms: Vec<syn::Stmt> = vec![];

    let body = syn::Expr::Block(syn::ExprBlock {
        attrs: vec![],
        label: None,
        block: syn::Block {
            brace_token: syn::token::Brace::default(),
            stmts: stms,
        },
    });
    let closure = syn::Expr::Closure(syn::ExprClosure {
        attrs: vec![],
        movability: None,
        asyncness: None,
        capture: None,
        or1_token: syn::token::Or::default(),
        or2_token: syn::token::Or::default(),
        inputs: input,
        output: syn::ReturnType::Default,
        body: Box::new(body),
    });
}
pub fn gen_single_local_varible(input: Vec<(syn::Ident, syn::token::Colon2)>) -> syn::Stmt {
    let mut input_clone = input.clone();
    let index = input_clone.len() - 1;
    let varible_name_ident = syn::Ident::new(
        input_clone[index].0.to_string().to_lowercase().as_str(),
        input_clone[index].0.span(),
    );
    let pat = syn::Pat::Ident(syn::PatIdent {
        attrs: vec![],
        by_ref: None,
        mutability: Some(syn::token::Mut::default()),
        ident: varible_name_ident,
        subpat: None,
    });
    let init=syn::Expr::MethodCall(syn::ExprMethodCall{
        attrs:vec![],
        method:gen_as_mut_method_call_path_segment()
    });
    let semi = syn::Stmt::Local(syn::Local {
        attrs: vec![],
        let_token: syn::token::Let::default(),
        pat,
        init,
        semi_token: syn::token::Semi::default(),
    });
    return semi;
}

pub fn gen_unwrap_method_call_path_segment(receiver: Box<syn::Expr>) -> syn::Expr {
    let method = syn::Ident::new("unwrap", proc_macro2::Span::call_site());
    let mut args: syn::punctuated::Punctuated<syn::Expr, syn::token::Comma> =
        syn::punctuated::Punctuated::default();
    let init_expr = syn::Expr::MethodCall(syn::ExprMethodCall {
        attrs: vec![], //[]
        receiver,
        dot_token: syn::token::Dot::default(), //.
        turbofish: None,
        method,                                    //ident
        paren_token: syn::token::Paren::default(), //()
        args,                                      //cs
    });
    init_expr
}
pub fn gen_as_mut_method_call_path_segment(receiver: Box<syn::Expr>) -> syn::Expr {
    let method = syn::Ident::new("as_mut", proc_macro2::Span::call_site());
    let mut args: syn::punctuated::Punctuated<syn::Expr, syn::token::Comma> =
        syn::punctuated::Punctuated::default();
    let init_expr = syn::Expr::MethodCall(syn::ExprMethodCall {
        attrs: vec![], //[]
        receiver,
        dot_token: syn::token::Dot::default(), //.
        turbofish: None,
        method,                                    //ident
        paren_token: syn::token::Paren::default(), //()
        args,                                      //cs
    });
    init_expr
}
pub fn gen_borrow_mut_method_call_path_segment(receiver: Box<syn::Expr>) -> syn::Expr {
    let method = syn::Ident::new("borrow_mut", proc_macro2::Span::call_site());
    let mut args: syn::punctuated::Punctuated<syn::Expr, syn::token::Comma> =
        syn::punctuated::Punctuated::default();
    let init_expr = syn::Expr::MethodCall(syn::ExprMethodCall {
        attrs: vec![], //[]
        receiver,
        dot_token: syn::token::Dot::default(), //.
        turbofish: None,
        method,                                    //ident
        paren_token: syn::token::Paren::default(), //()
        args,                                      //cs
    });
    init_expr
}
pub fn gen_borrow_method_call_path_segment(receiver: Box<syn::Expr>) -> syn::Expr {
    let method = syn::Ident::new("borrow", proc_macro2::Span::call_site());

    let arg_ident = syn::Ident::new("cs", proc_macro2::Span::call_site());
    let arg_path_segment = syn::PathSegment {
        ident: arg_ident,
        arguments: syn::PathArguments::None,
    };
    let mut cs: syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> =
        syn::punctuated::Punctuated::default();
    cs.push(arg_path_segment);
    let args_path = syn::Path {
        leading_colon: None,
        segments: cs,
    };
    let mut args: syn::punctuated::Punctuated<syn::Expr, syn::token::Comma> =
        syn::punctuated::Punctuated::default();
    let args_expr = syn::Expr::Path(syn::ExprPath {
        attrs: vec![],
        qself: None,
        path: args_path,
    });
    args.push(args_expr);
    let init_expr = syn::Expr::MethodCall(syn::ExprMethodCall {
        attrs: vec![], //[]
        receiver,
        dot_token: syn::token::Dot::default(), //.
        turbofish: None,
        method,                                    //ident
        paren_token: syn::token::Paren::default(), //()
        args,                                      //cs
    });
    init_expr
}
