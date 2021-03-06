// 🧭📦 nwl-protobufs-rust: Rust crate for the generated protobufs for Noelware Analytics
// Copyright (c) 2022 Noelware
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! This is the Rust crate for using Noelware's Analytics API
//! which includes a client implementation under the [`protos/`][protos] module.
//!
//! This was meant to be in use with the [analytics-server](https://github.com/Noelware/analytics-server),
//! but you can use the client to request to your instances.
//!
//! ## Example
//! ```rs
//! use nwl_protobufs_rust::protos::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    // [::1]:50051 is the server that uses Noelware Analytics.
//!    // The server must implement a gRPC endpoint.
//!    //
//!    // Supported products: charted-server, Arisu
//!    // Using official instances will not work.
//!    let mut client = AnalyticsClient::connect("http://[::1]:50051").await?;
//!
//!    let request = tonic::Request::new(ConnectionAckRequest {});
//!    let res = client.connection_ack(request).await?;
//!
//!    Ok(())
//! }
//! ```

use std::env;

pub use prost_types;
pub mod protos;

include_protobufs!();

/// Returns the commit SHA of the commit hash that was used to build
/// the protocol buffers.
pub const PROTOBUF_COMMIT: &str = env!("PROTOBUF_COMMIT_SHA");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
