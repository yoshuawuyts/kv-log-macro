use log::Level;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Token};

#[derive(Default, Debug)]
struct Log {
    exprs: Vec<Expr>,
}

impl Parse for Log {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut log = Log::default();

        while !input.is_empty() {
            log.exprs.push(input.parse::<Expr>()?);

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(log)
    }
}

/// The `join!` macro.
pub(crate) fn log_internal(level: Option<Level>, input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as Log);
    let mut exprs = parsed.exprs.iter();

    // Get the log level. If it's not passed directly it should be the first arg
    // of the macro.
    let level = level.unwrap_or_else(|| {
        let expr = exprs.next().unwrap();
        expr_to_level(expr)
    });
    dbg!(level);

    // TODO: Get the format string. This is the base we'll use to later map all
    // of our arguments on.

    // TODO: Get the `key = value` arguments and the standalone expression
    // arguments. We pass all of these as args into the formatter as well.

    // TODO: The last item in the iterator is special and may contain the `{
    // key: value }` pairs. We should parse this and extract the pairs into a
    // visitor.

    for expr in parsed.exprs {
        println!("{:?}", expr);
    }
    TokenStream::from(quote! {
        println!("hello");
    })
}

fn expr_to_level(expr: &syn::Expr) -> Level {
    let path = match expr {
        syn::Expr::Path(path) => path,
        _ => panic!("Expected an instance of `log::Level`"),
    };
    let id = &path.path.segments.iter().last().unwrap();
    match format!("{}", &id.ident).as_ref() {
        "Error" => Level::Error,
        "Warn" => Level::Warn,
        "Info" => Level::Info,
        "Debug" => Level::Debug,
        "Trace" => Level::Trace,
        _ => panic!("Unknown log level"),
    }
}
