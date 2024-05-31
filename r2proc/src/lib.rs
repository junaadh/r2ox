use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Unit)]
pub fn derive_pager(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;

    quote! {
        impl crate::memory::traits::Unit for #name {
            type Addr = x86_64::VirtAddr;

            #[inline]
            fn new(addr: Self::Addr) -> Self {
                Self { index: addr.into() }
            }

            #[inline]
            fn at(index: crate::memory::pager::index::PageIndex) -> Self {
                Self { index }
            }

            #[inline]
            fn addr(self) -> x86_64::VirtAddr {
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
                write!(f, "Page<{:x}>", self.addr().as_u64())
            }
        }
    }
    .into()
}
