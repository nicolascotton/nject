use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr, ExprClosure, Pat, PatType,
};

pub struct FactoryExpr {
    pub inputs: Vec<PatType>,
    pub body: Box<Expr>,
}

impl Parse for FactoryExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr: ExprClosure = input.parse()?;
        let mut inputs = Vec::with_capacity(expr.inputs.len());
        let span = expr.span();
        for input in expr.inputs {
            if let Pat::Type(pat_type) = input {
                inputs.push(pat_type);
            } else {
                return Err(syn::Error::new(span, format!("Invalid input: {input:?}")));
            }
        }
        Ok(FactoryExpr {
            inputs,
            body: expr.body,
        })
    }
}
