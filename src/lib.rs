// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Workaround for <https://github.com/tokio-rs/tracing/issues/2082>

use proc_macro::TokenStream;

/// A dummy implementation of the [`tracing::instrument`] proc macro
/// This should be used as a replacement for the [`tracing::instrument`] proc
/// macro when the code coverage is needed.
///
/// # Example
/// ```ignore
/// #[cfg(not(coverage))]
/// pub use tracing::instrument;
/// #[cfg(coverage)]
/// pub use tracing_instrument_mock::instrument;
///
/// #[instrument]
/// fn my_function() {
/// 	println!("Hello, world!");
/// }
/// ```
#[proc_macro_attribute]
pub fn instrument(_args: TokenStream, item: TokenStream) -> TokenStream {
	item
}
