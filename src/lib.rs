#![cfg_attr(not(feature = "std"), no_std)]

//! Any entity (such as the committee) with access to the "root mandate" (this module)
//! can use the `apply` function to dispatch calls as root. Think of this module as an
//! other `sudo` module controlled by another module (ex: a multisig or collective).

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::Dispatchable, pallet_prelude::*, weights::GetDispatchInfo};
    use frame_system::pallet_prelude::*;
    use sp_runtime::DispatchResult;
    use sp_std::prelude::Box;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Call: Parameter + Dispatchable<Origin = Self::Origin> + GetDispatchInfo;

        /// Origin that can call this module and execute sudo actions. Typically
        /// the `collective` module.
        type ExternalOrigin: EnsureOrigin<Self::Origin>;
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A root operation was executed, show result
        RootOp(DispatchResult),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Let the configured origin dispatch a call as root
        #[pallet::weight(call.get_dispatch_info().weight + 10_000)]
        pub fn apply(
            origin: OriginFor<T>,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo {
            T::ExternalOrigin::ensure_origin(origin)?;

            // Shamelessly stollen from the `sudo` module
            let res = call.dispatch(frame_system::RawOrigin::Root.into());

            Self::deposit_event(Event::RootOp(res.map(|_| ()).map_err(|e| e.error)));

            Ok(().into())
        }
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
}
