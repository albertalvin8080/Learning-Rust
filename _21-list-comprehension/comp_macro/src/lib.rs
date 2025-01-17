/*
comp: mapping for_if_clause

mapping: expression

for_if_clause:
    | 'for' pattern 'in' sequence ('if' expression)*

pattern: name (, name)*
*/

/*
NOTES:
>> input.parse::<T>()         -> Turbofish.
>> let any: T = input.parse() -> Type inference.
>> input.parse::<T>() actually calls syn::parse::Parse::parse<T>().
>> syn::Token![if] returns the token representation for 'if'.
>> The tokenstream must be parsed following the order of the list comprehension:
- 1. mapping 
- 2. for_if_clause
- 2.1. 'for' pattern
- 2.2. 'in' sequence
- 2.3. ('if' expression)*
*/

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct Mapping(syn::Expr);

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Expr>().map(Self)
        // Ok(Self(input.parse::<syn::Expr>()?))
    }
}

impl quote::ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct Pattern(syn::Pat);

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.call(syn::Pat::parse_single).map(Self)
        // Ok(Self(input.call(syn::Pat::parse_single)?))
    }
}

impl quote::ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct Condition(syn::Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![if]>()?;
        input.parse::<syn::Expr>().map(Self)
    }
}

impl quote::ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct ForIfClause {
    pattern: Pattern,
    sequence: syn::Expr,
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![for]>()?;
        let pattern = input.parse::<Pattern>()?;

        _ = input.parse::<syn::Token![in]>()?;
        let sequence = input.parse::<syn::Expr>()?;

        let conditions = parse_zero_or_many::<Condition>(input);

        Ok(Self {
            pattern,
            sequence,
            conditions,
        })
    }
}

fn parse_zero_or_many<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut vec: Vec<T> = Vec::new();

    while let Ok(t) = input.parse::<T>() {
        vec.push(t);
    }

    vec
}

struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mapping = input.parse::<Mapping>()?;
        let for_if_clause = input.parse::<ForIfClause>()?;
        Ok(Self {
            mapping,
            for_if_clause,
        })
    }
}

impl quote::ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            sequence,
            conditions,
        } = &self.for_if_clause;

        // Another way without implementing quote::ToTokens for Condition struct.
        // let conditions = conditions.iter().map(|c| {
        //     let inner = &c.0;
        //     quote! {#inner}
        // });

        tokens.extend(quote! {
            core::iter::IntoIterator::into_iter(#sequence).filter_map(|#pattern| {
                // .then() is from primitive bool. It returns an Option<>
                /*
                `true` is the default behavior for no if conditions.
                */
                (true #(&& (#conditions))*).then(|| #mapping)
            })
        });
    }
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // parse_macro_input! will internally call syn::parse::Parse::parse() for Comp struct.
    let c = parse_macro_input!(input as Comp);
    // >> quote! internally calls quote::ToTokens on #c
    // >> into() converts proc_macro2::TokenStream to proc_macro::TokenStream
    quote! {#c}.into()
}
