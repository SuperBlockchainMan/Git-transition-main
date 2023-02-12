use darling::FromMeta;
use syn::{Lit, NestedMeta};

#[derive(Debug)]
pub struct Versions(pub Vec<Version>);

#[derive(Debug, PartialEq, Eq)]
pub struct Version {
    pub version: u64
}

impl Version {
    pub fn to_ident(&self, ident: &syn::Ident) -> syn::Ident {
        syn::Ident::new(
            &format!(
                "{}V{}",
                ident,
                self.version
            ),
            ident.span(),
        )
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromMeta for Version {
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Version { version: value.parse::<u64>().unwrap() })
    }
}

impl FromMeta for Versions {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut versions = Vec::new();
        for item in items {
            match item {
                NestedMeta::Lit(Lit::Str(str)) => {
                    versions.push(Version::from_string(&str.value())?);
                }
                _ => {}
            }
        }
        Ok(Versions(versions))
    }
}