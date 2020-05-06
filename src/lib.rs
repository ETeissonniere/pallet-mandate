#![cfg_attr(not(feature = "std"), no_std)]

//! Any entity (such as the committee) with access to the "root mandate" (this module)
//! can use the `apply` function to dispatch calls as root. Think of this module as an
//! other `sudo` module controlled by another module (ex: a multisig or collective).

use frame_support::{
    decl_event, decl_module,
    traits::EnsureOrigin,
    weights::{FunctionOf, GetDispatchInfo, Pays},
    Parameter,
};
use frame_system as system;
use sp_runtime::{traits::Dispatchable, DispatchResult};
use sp_std::prelude::Box;

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
    type Call: Parameter + Dispatchable<Origin = Self::Origin> + GetDispatchInfo;

    /// Origin that can call this module and execute sudo actions. Typically
    /// the `collective` module.
    type ExternalOrigin: EnsureOrigin<Self::Origin>;
}

decl_event!(
    pub enum Event {
        /// A root operation was executed, show result
        RootOp(DispatchResult),
    }
);

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Let the configured origin dispatch a call as root
        #[weight = FunctionOf(
            |args: (&Box<<T as Trait>::Call>,)| args.0.get_dispatch_info().weight + 10_000,
            |args: (&Box<<T as Trait>::Call>,)| args.0.get_dispatch_info().class,
            Pays::Yes,
        )]
        pub fn apply(origin, call: Box<<T as Trait>::Call>) {
            T::ExternalOrigin::ensure_origin(origin)?;

            // Shamelessly stollen from the `sudo` module
            let res = call.dispatch(frame_system::RawOrigin::Root.into());

            Self::deposit_event(Event::RootOp(res.map(|_| ()).map_err(|e| e.error)));
        }
    }
}
