extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Token};

struct OneType(syn::Ident);
impl syn::parse::Parse for OneType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type1 = input.parse()?;
        Ok(OneType(type1))
    }
}

struct TwoTypes(syn::Ident, syn::Ident);
impl syn::parse::Parse for TwoTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type1 = input.parse()?;
        input.parse::<Token![,]>()?;
        let type2 = input.parse()?;
        Ok(TwoTypes(type1, type2))
    }
}

#[proc_macro_derive(Unit, attributes(unit_type))]
pub fn derive_unit(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let attr = match input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("unit_type"))
        .map(|attr| {
            // syn::parse2::<OneType>(attr.clone().into_token_stream())
            attr.parse_args::<OneType>()
                .expect("Failed to parse attribute")
        }) {
        Some(s) => s.0,
        None => panic!("Expected #[framer_type(/*type*/)]"),
    };

    quote! {
        impl crate::memory::traits::Unit for #name {
            type Addr = #attr;

            #[inline]
            fn new(addr: Self::Addr) -> Self {
                Self { index: addr.into() }
            }

            #[inline]
            fn at(index: crate::memory::pager::index::PageIndex) -> Self {
                Self { index }
            }

            #[inline]
            fn addr(self) -> Self::Addr {
                Self::Addr::new((self.index.0 * crate::memory::PAGE_SIZE) as u64)
            }

            #[inline]
            fn inclusive_addr(self) -> Self::Addr {
                self.index.into()
            }

            #[inline]
            fn index(self) -> crate::memory::pager::index::PageIndex {
                self.index
            }
        }

        impl core::ops::Add<usize> for #name {
            type Output = Self;
            fn add(self, rhs: usize) -> Self::Output {
                Self::at(self.index + rhs)
            }
        }

        impl core::ops::Sub<usize> for #name {
            type Output = Self;
            fn sub(self, rhs: usize) -> Self::Output {
                Self::at(self.index - rhs)
            }
        }

        impl core::ops::AddAssign<usize> for #name {
            fn add_assign(&mut self, rhs: usize) {
                self.index += rhs;
            }
        }

        impl core::ops::SubAssign<usize> for #name {
            fn sub_assign(&mut self, rhs: usize) {
                self.index -= rhs;
            }
        }

        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}<{:x}>", stringify!(#name), self.addr().as_u64())
            }
        }
    }
    .into()
}

#[proc_macro_derive(Range, attributes(range_types))]
pub fn derive_range(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let (basic, iter) = match input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("range_types"))
        .map(|a| {
            a.parse_args::<TwoTypes>()
                .expect("Failed to parse arguments")
        }) {
        Some(a) => (a.0, a.1),
        None => panic!("Expected #[range_types(/*type1: Unit*//*type2: Iterator*/)]"),
    };

    quote! {
        impl crate::memory::traits::Range for #name {
            type Basic = #basic;
            type Iter = #iter;

            #[inline]
            fn new(start: Self::Basic, end: Self::Basic) -> Self {
                Self {start, end}
            }

            #[inline]
            fn empty() -> Self {
                Self {
                    start: Self::Basic::at(PageIndex(1)),
                    end: Self::Basic::at(PageIndex(0))
                }
            }

            #[inline]
            fn start(self) -> Self::Basic {
                self.start
            }

            #[inline]
            fn end(self) -> Self::Basic {
                self.end
            }

            #[inline]
            fn as_page_size(self) -> usize {
                (self.end.index.0 - self.start.index.0) + 1
            }

            #[inline]
            fn as_bytes(self) -> usize {
                self.as_page_size() * crate::memory::PAGE_SIZE
            }

            #[inline]
            fn is_empty(self) -> bool {
                self.start > self.end
            }

            #[inline]
            fn start_addr(self) -> <Self::Basic as Unit>::Addr {
                self.start.addr()
            }

            #[inline]
            fn inclusive_addr(self) -> <Self::Basic as Unit>::Addr {
                self.end.inclusive_addr()
            }

            #[inline]
            fn merge(&mut self, other: Self) -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
                if other.is_empty() {
                    return Ok(());
                }
                if other.start != self.end + 1 && other.end + 1 != self.start {
                    return Err("Error merging pages".into());
                }
                if other.start < self.start {
                    self.start = other.start;
                }
                if other.end > self.end {
                    self.end = other.end;
                }

                Ok(())
            }

            #[inline]
            fn overlaps(self, other: Self) -> bool {
                if self.is_empty() ||other.is_empty() {
                    return false;
                }

                self.start <= other.end && other.start <= self.end
            }

            #[inline]
            fn consumes(self, other: Self) -> bool {
                if self.is_empty() || other.is_empty() {
                    return false;
                }
                self.start <= other.start && self.end >= other.end
            }

            #[inline]
            fn contains(self, unit: Self::Basic) -> bool {
                if self.is_empty() {
                    return false;
                }
                self.start <= unit && self.end >= unit
            }

            #[inline]
            fn iter(&self) -> Self::Iter {
                Self::Iter::new(self)
            }

        }
    }
    .into()
}
