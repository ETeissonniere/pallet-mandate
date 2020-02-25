#![cfg_attr(not(feature = "std"), no_std)]

//! Any entity (such as the committee) with access to the "root mandate" (this module)
//! can use the `apply` function to dispatch calls as root. Think of this module as an
//! other `sudo` module controlled by another module (ex: a multisig or collective).

use frame_support::{decl_event, decl_module, dispatch::DispatchResult, Parameter};
use sp_runtime::{
    traits::{Dispatchable, EnsureOrigin},
    DispatchError,
};
use sp_std::prelude::Box;

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
    type Proposal: Parameter + Dispatchable<Origin = Self::Origin>;

    /// Origin that can call this module and execute sudo actions. Typically
    /// the `collective` module.
    type ExternalOrigin: EnsureOrigin<Self::Origin>;
}

decl_event!(
    pub enum Event {
        /// A root operation was executed, show result
        RootOp(bool),
    }
);

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        pub fn apply(origin, proposal: Box<T::Proposal>) -> DispatchResult {
            T::ExternalOrigin::ensure_origin(origin)?;

            // Shamelessly stollen from the `sudo` module
            let res = match proposal.dispatch(system::RawOrigin::Root.into()) {
                Ok(_) => true,
                Err(e) => {
                    let e: DispatchError = e.into();
                    sp_runtime::print(e);
                    false
                }
            };

            Self::deposit_event(Event::RootOp(res));
            Ok(())
        }
    }
}
