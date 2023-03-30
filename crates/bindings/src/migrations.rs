pub use migrations::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod migrations {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"name\":\"newAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastCompletedMigration\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"name\":\"completed\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCompleted\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static MIGRATIONS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        160,
        96,
        2,
        10,
        3,
        25,
        22,
        51,
        23,
        144,
        85,
        97,
        2,
        4,
        128,
        97,
        0,
        50,
        96,
        0,
        57,
        96,
        0,
        243,
        0,
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        97,
        0,
        69,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        99,
        255,
        255,
        255,
        255,
        22,
        128,
        99,
        9,
        0,
        240,
        16,
        20,
        97,
        0,
        74,
        87,
        128,
        99,
        141,
        165,
        203,
        91,
        20,
        97,
        0,
        122,
        87,
        128,
        99,
        251,
        219,
        173,
        60,
        20,
        97,
        0,
        184,
        87,
        128,
        99,
        253,
        172,
        213,
        118,
        20,
        97,
        0,
        223,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        52,
        128,
        21,
        97,
        0,
        86,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        120,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        96,
        4,
        53,
        22,
        97,
        0,
        247,
        86,
        91,
        0,
        91,
        52,
        128,
        21,
        97,
        0,
        134,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        143,
        97,
        1,
        141,
        86,
        91,
        96,
        64,
        128,
        81,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        144,
        146,
        22,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        52,
        128,
        21,
        97,
        0,
        196,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        205,
        97,
        1,
        169,
        86,
        91,
        96,
        64,
        128,
        81,
        145,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        52,
        128,
        21,
        97,
        0,
        235,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        120,
        96,
        4,
        53,
        97,
        1,
        175,
        86,
        91,
        96,
        0,
        128,
        84,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        51,
        20,
        21,
        97,
        1,
        137,
        87,
        129,
        144,
        80,
        128,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        99,
        253,
        172,
        213,
        118,
        96,
        1,
        84,
        96,
        64,
        81,
        130,
        99,
        255,
        255,
        255,
        255,
        22,
        96,
        224,
        27,
        129,
        82,
        96,
        4,
        1,
        128,
        130,
        129,
        82,
        96,
        32,
        1,
        145,
        80,
        80,
        96,
        0,
        96,
        64,
        81,
        128,
        131,
        3,
        129,
        96,
        0,
        135,
        128,
        59,
        21,
        128,
        21,
        97,
        1,
        112,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        90,
        241,
        21,
        128,
        21,
        97,
        1,
        132,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        80,
        80,
        80,
        91,
        80,
        80,
        86,
        91,
        96,
        0,
        84,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        129,
        86,
        91,
        96,
        1,
        84,
        129,
        86,
        91,
        96,
        0,
        84,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        51,
        20,
        21,
        97,
        1,
        213,
        87,
        96,
        1,
        129,
        144,
        85,
        91,
        80,
        86,
        0,
        161,
        101,
        98,
        122,
        122,
        114,
        48,
        88,
        32,
        50,
        220,
        76,
        186,
        251,
        175,
        5,
        174,
        157,
        188,
        254,
        213,
        75,
        99,
        180,
        40,
        148,
        49,
        10,
        106,
        45,
        139,
        197,
        32,
        27,
        150,
        34,
        149,
        188,
        125,
        141,
        1,
        0,
        41,
    ];
    ///The bytecode of the contract.
    pub static MIGRATIONS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        97,
        0,
        69,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        99,
        255,
        255,
        255,
        255,
        22,
        128,
        99,
        9,
        0,
        240,
        16,
        20,
        97,
        0,
        74,
        87,
        128,
        99,
        141,
        165,
        203,
        91,
        20,
        97,
        0,
        122,
        87,
        128,
        99,
        251,
        219,
        173,
        60,
        20,
        97,
        0,
        184,
        87,
        128,
        99,
        253,
        172,
        213,
        118,
        20,
        97,
        0,
        223,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        52,
        128,
        21,
        97,
        0,
        86,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        120,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        96,
        4,
        53,
        22,
        97,
        0,
        247,
        86,
        91,
        0,
        91,
        52,
        128,
        21,
        97,
        0,
        134,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        143,
        97,
        1,
        141,
        86,
        91,
        96,
        64,
        128,
        81,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        144,
        146,
        22,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        52,
        128,
        21,
        97,
        0,
        196,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        205,
        97,
        1,
        169,
        86,
        91,
        96,
        64,
        128,
        81,
        145,
        130,
        82,
        81,
        144,
        129,
        144,
        3,
        96,
        32,
        1,
        144,
        243,
        91,
        52,
        128,
        21,
        97,
        0,
        235,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        0,
        120,
        96,
        4,
        53,
        97,
        1,
        175,
        86,
        91,
        96,
        0,
        128,
        84,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        51,
        20,
        21,
        97,
        1,
        137,
        87,
        129,
        144,
        80,
        128,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        99,
        253,
        172,
        213,
        118,
        96,
        1,
        84,
        96,
        64,
        81,
        130,
        99,
        255,
        255,
        255,
        255,
        22,
        96,
        224,
        27,
        129,
        82,
        96,
        4,
        1,
        128,
        130,
        129,
        82,
        96,
        32,
        1,
        145,
        80,
        80,
        96,
        0,
        96,
        64,
        81,
        128,
        131,
        3,
        129,
        96,
        0,
        135,
        128,
        59,
        21,
        128,
        21,
        97,
        1,
        112,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        90,
        241,
        21,
        128,
        21,
        97,
        1,
        132,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        80,
        80,
        80,
        91,
        80,
        80,
        86,
        91,
        96,
        0,
        84,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        129,
        86,
        91,
        96,
        1,
        84,
        129,
        86,
        91,
        96,
        0,
        84,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        22,
        51,
        20,
        21,
        97,
        1,
        213,
        87,
        96,
        1,
        129,
        144,
        85,
        91,
        80,
        86,
        0,
        161,
        101,
        98,
        122,
        122,
        114,
        48,
        88,
        32,
        50,
        220,
        76,
        186,
        251,
        175,
        5,
        174,
        157,
        188,
        254,
        213,
        75,
        99,
        180,
        40,
        148,
        49,
        10,
        106,
        45,
        139,
        197,
        32,
        27,
        150,
        34,
        149,
        188,
        125,
        141,
        1,
        0,
        41,
    ];
    ///The deployed bytecode of the contract.
    pub static MIGRATIONS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Migrations<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Migrations<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Migrations<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Migrations<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Migrations<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Migrations))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Migrations<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MIGRATIONS_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MIGRATIONS_ABI.clone(),
                MIGRATIONS_BYTECODE.clone(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `lastCompletedMigration` (0xfbdbad3c) function
        pub fn last_completed_migration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([251, 219, 173, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCompleted` (0xfdacd576) function
        pub fn set_completed(
            &self,
            completed: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 172, 213, 118], completed)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgrade` (0x0900f010) function
        pub fn upgrade(
            &self,
            new_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 0, 240, 16], new_address)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Migrations<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `lastCompletedMigration` function with signature `lastCompletedMigration()` and selector `0xfbdbad3c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lastCompletedMigration", abi = "lastCompletedMigration()")]
    pub struct LastCompletedMigrationCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `setCompleted` function with signature `setCompleted(uint256)` and selector `0xfdacd576`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setCompleted", abi = "setCompleted(uint256)")]
    pub struct SetCompletedCall {
        pub completed: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(address)` and selector `0x0900f010`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "upgrade", abi = "upgrade(address)")]
    pub struct UpgradeCall {
        pub new_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MigrationsCalls {
        LastCompletedMigration(LastCompletedMigrationCall),
        Owner(OwnerCall),
        SetCompleted(SetCompletedCall),
        Upgrade(UpgradeCall),
    }
    impl ::ethers::core::abi::AbiDecode for MigrationsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <LastCompletedMigrationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastCompletedMigration(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <SetCompletedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetCompleted(decoded));
            }
            if let Ok(decoded) = <UpgradeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Upgrade(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MigrationsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::LastCompletedMigration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetCompleted(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Upgrade(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MigrationsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LastCompletedMigration(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCompleted(element) => ::core::fmt::Display::fmt(element, f),
                Self::Upgrade(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LastCompletedMigrationCall> for MigrationsCalls {
        fn from(value: LastCompletedMigrationCall) -> Self {
            Self::LastCompletedMigration(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MigrationsCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<SetCompletedCall> for MigrationsCalls {
        fn from(value: SetCompletedCall) -> Self {
            Self::SetCompleted(value)
        }
    }
    impl ::core::convert::From<UpgradeCall> for MigrationsCalls {
        fn from(value: UpgradeCall) -> Self {
            Self::Upgrade(value)
        }
    }
    ///Container type for all return fields from the `lastCompletedMigration` function with signature `lastCompletedMigration()` and selector `0xfbdbad3c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LastCompletedMigrationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
