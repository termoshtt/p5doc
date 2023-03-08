//! p5doc proc-macro

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::*;
use quote::quote;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn p5doc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    p5doc2(item.into()).into()
}

fn p5doc2(item: TokenStream2) -> TokenStream2 {
    if let Ok(mut item) = syn::parse2(item.clone()) {
        match item {
            // Items may have `#[doc]` attributes
            syn::Item::Const(syn::ItemConst { ref mut attrs, .. })
            | syn::Item::Enum(syn::ItemEnum { ref mut attrs, .. })
            | syn::Item::Fn(syn::ItemFn { ref mut attrs, .. })
            | syn::Item::ForeignMod(syn::ItemForeignMod { ref mut attrs, .. })
            | syn::Item::Macro(syn::ItemMacro { ref mut attrs, .. })
            | syn::Item::Macro2(syn::ItemMacro2 { ref mut attrs, .. })
            | syn::Item::Mod(syn::ItemMod { ref mut attrs, .. })
            | syn::Item::Static(syn::ItemStatic { ref mut attrs, .. })
            | syn::Item::Struct(syn::ItemStruct { ref mut attrs, .. })
            | syn::Item::Trait(syn::ItemTrait { ref mut attrs, .. })
            | syn::Item::TraitAlias(syn::ItemTraitAlias { ref mut attrs, .. })
            | syn::Item::Type(syn::ItemType { ref mut attrs, .. })
            | syn::Item::Union(syn::ItemUnion { ref mut attrs, .. }) => {
                convert(attrs);
                return quote! { #item };
            }
            _ => {}
        }
    }

    item
}

fn convert(attrs: &mut Vec<syn::Attribute>) {
    // True if `attr` between quote start(```p5doc) and end (```)
    let mut in_quote = false;
    for attr in attrs {
        let path = &attr.path;
        // if attr is in `#[doc = literal]` form
        if path.segments.len() != 1 || path.segments[0].ident != "doc" {
            continue;
        }

        let mut iter = attr.tokens.clone().into_iter();
        let doc = match (iter.next(), iter.next(), iter.next()) {
            (Some(_eq), Some(lit), None) => {
                // literal includes `"`
                lit.to_string().trim_matches('"').trim().to_string()
            }
            _ => continue,
        };

        let start_tag = "```p5doc";
        if doc.starts_with(start_tag) {
            let re = regex::Regex::new(r"(\d*)x(\d*)").unwrap();
            let (width, height) = re
                .captures(&doc)
                .and_then(|cap| {
                    let width = cap.get(1)?.as_str();
                    let height = cap.get(2)?.as_str();
                    Some((width.parse().ok()?, height.parse().ok()?))
                })
                .expect("p5doc must have width and height like `200x100` form");
            let lit = format!(
                r#"<div id={CANVAS_ID}></div> {CDN_P5JS} <script> {} function draw() {{"#,
                setup(width, height)
            );
            attr.tokens = quote! { = #lit };
            in_quote = true;
        }

        if in_quote && doc.ends_with("```") {
            attr.tokens = quote! { = "} </script>"};
            return;
        }
    }
}

const CDN_P5JS: &str = r#"<script src="https://cdn.jsdelivr.net/npm/p5@1.6.0/lib/p5.js"></script>"#;
const CANVAS_ID: &str = "p5doc";

// Create `setup()` for p5.js global mode
fn setup(width: u64, height: u64) -> String {
    format!(
        r#"function setup() {{ var canvas = createCanvas({width}, {height}); canvas.parent("{CANVAS_ID}"); }}"#
    )
}
