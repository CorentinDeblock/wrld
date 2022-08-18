use proc_macro2::{Ident, Punct, Span, Spacing};
use quote::TokenStreamExt;

use crate::converter::VertexFormat;

pub struct TokenVertexFormat {
    pub attribute: VertexFormat
}

impl quote::ToTokens for TokenVertexFormat {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        //tokens.append_separated(&["wgpu", "VertexFormat", "Float32x2"], "::");
        tokens.append(Ident::new("wgpu", Span::call_site()));
        tokens.append(Punct::new(':', Spacing::Joint));
        tokens.append(Punct::new(':', Spacing::Alone));
        tokens.append(Ident::new("VertexFormat", Span::call_site()));
        tokens.append(Punct::new(':', Spacing::Joint));
        tokens.append(Punct::new(':', Spacing::Alone));
        tokens.append(Ident::new(format!("{:?}", self.attribute).as_str(), Span::call_site()));
    }
}
