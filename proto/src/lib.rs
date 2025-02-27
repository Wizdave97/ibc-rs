//! ibc-proto library gives the developer access to the Cosmos SDK IBC proto-defined structs.

// Todo: automate the creation of this module setup based on the dots in the filenames.
//  This module setup is necessary because the generated code contains "super::" calls for dependencies.

#![no_std]
#![deny(warnings, trivial_casts, trivial_numeric_casts, unused_import_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::bare_urls)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/ibc-proto/0.15.0")]

extern crate alloc;
extern crate core as std;

// re-export format! macro from alloc::format to allow its use
// in generated code
#[macro_export]
macro_rules! format {
    ($($args:tt)*) => {
        ::alloc::format!($( $args )*)
    }
}
#[cfg(feature = "std")]
macro_rules! include_proto {
    ($path:literal) => {
        include!(concat!("prost/std/", $path));
    };
}

#[cfg(not(feature = "std"))]
macro_rules! include_proto {
    ($path:literal) => {
        include!(concat!("prost/no_std/", $path));
    };
}

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("prost/COSMOS_SDK_COMMIT");

pub mod cosmos {
    pub mod auth {
        pub mod v1beta1 {
            include_proto!("cosmos.auth.v1beta1.rs");
            /// EthAccount defines an Ethermint account.
            /// TODO: remove when/if a canonical `EthAccount`
            /// lands in the next Cosmos SDK release
            /// (note https://github.com/cosmos/cosmos-sdk/pull/9981
            /// only adds the PubKey type)
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct EthAccount {
                #[prost(message, optional, tag = "1")]
                pub base_account: ::core::option::Option<BaseAccount>,
                #[prost(bytes = "vec", tag = "2")]
                pub code_hash: ::prost::alloc::vec::Vec<u8>,
            }
        }
    }
    pub mod staking {
        pub mod v1beta1 {
            include_proto!("cosmos.staking.v1beta1.rs");
        }
    }
    pub mod base {
        pub mod abci {
            pub mod v1beta1 {
                include_proto!("cosmos.base.abci.v1beta1.rs");
            }
        }
        pub mod kv {
            pub mod v1beta1 {
                include_proto!("cosmos.base.kv.v1beta1.rs");
            }
        }
        pub mod query {
            pub mod v1beta1 {
                include_proto!("cosmos.base.query.v1beta1.rs");
            }

            pub mod pagination {
                use super::v1beta1::PageRequest;

                pub fn all() -> Option<PageRequest> {
                    Some(PageRequest {
                        limit: u64::MAX,
                        ..Default::default()
                    })
                }
            }
        }
        pub mod reflection {
            pub mod v1beta1 {
                include_proto!("cosmos.base.reflection.v1beta1.rs");
            }
        }
        pub mod store {
            pub mod v1beta1 {
                include_proto!("cosmos.base.store.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include_proto!("cosmos.base.v1beta1.rs");
        }
        pub mod tendermint {
            pub mod v1beta1 {
                include_proto!("cosmos.base.tendermint.v1beta1.rs");
            }
        }
    }
    pub mod crypto {
        pub mod multisig {
            pub mod v1beta1 {
                include_proto!("cosmos.crypto.multisig.v1beta1.rs");
            }
        }
    }
    pub mod tx {
        pub mod signing {
            pub mod v1beta1 {
                include_proto!("cosmos.tx.signing.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include_proto!("cosmos.tx.v1beta1.rs");
        }
    }
    pub mod upgrade {
        pub mod v1beta1 {
            include_proto!("cosmos.upgrade.v1beta1.rs");
        }
    }
    pub mod gov {
        pub mod v1beta1 {
            include_proto!("cosmos.gov.v1beta1.rs");
        }
    }
}

pub mod ibc {
    pub mod apps {
        pub mod transfer {
            pub mod v1 {
                include_proto!("ibc.applications.transfer.v1.rs");
            }
        }
    }
    pub mod core {
        pub mod channel {
            pub mod v1 {
                include_proto!("ibc.core.channel.v1.rs");
            }
        }
        pub mod client {
            pub mod v1 {
                include_proto!("ibc.core.client.v1.rs");
            }
        }
        pub mod commitment {
            pub mod v1 {
                include_proto!("ibc.core.commitment.v1.rs");
            }
        }
        pub mod connection {
            pub mod v1 {
                include_proto!("ibc.core.connection.v1.rs");
            }
        }
        pub mod types {
            pub mod v1 {
                include_proto!("ibc.core.types.v1.rs");
            }
        }
        pub mod port {
            pub mod v1 {
                include_proto!("ibc.core.port.v1.rs");
            }
        }
    }
    pub mod lightclients {
        pub mod localhost {
            pub mod v1 {
                include_proto!("ibc.lightclients.localhost.v1.rs");
            }
        }
        pub mod solomachine {
            pub mod v1 {
                include_proto!("ibc.lightclients.solomachine.v1.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include_proto!("ibc.lightclients.tendermint.v1.rs");
            }
        }
    }
    pub mod mock {
        include_proto!("ibc.mock.rs");
    }
}

pub mod ics23 {
    include_proto!("ics23.rs");
}
