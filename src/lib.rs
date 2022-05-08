#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as TemplateModule {
		pub Identity get(fn get_identity): map hasher(blake2_128_concat) Vec<u8> => Option<T::AccountId>;

		// ( identity, attribute_key ) => attribute_value
		pub Attribute get(fn get_attribute): map hasher(blake2_128_concat) (Vec<u8>, Vec<u8>) => Vec<u8>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		IdentityCreated(Vec<u8>, AccountId),
		AttributeAdded(Vec<u8>, Vec<u8>, Vec<u8>),
		AttributeRemoved(Vec<u8>, Vec<u8>),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		IdentityAlreadyClaimed,
		IdentityNotFound,
		NotAuthorized,
		AttributeNotFound,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1, 1)]
		pub fn create_identity(origin,identity: Vec<u8>) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			match <Identity<T>>::get(&identity) {
				None => {
					<Identity<T>>::insert(&identity, &who);
					Self::deposit_event(RawEvent::IdentityCreated(identity, who));
					Ok(())
				},
				Some(_) => Err(Error::<T>::IdentityAlreadyClaimed)?
			}

		}
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn add_attribute(origin,identity: Vec<u8>,attribute_key: Vec<u8>,
					attribute_value: Vec<u8>
				) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			match <Identity<T>>::get(&identity) {
				None => Err(Error::<T>::IdentityNotFound)?,
				Some(address) => {
					if address != who {
						return Err(Error::<T>::NotAuthorized)?
					} else{
						Attribute::insert((&identity, &attribute_key), &attribute_value);
						Self::deposit_event(RawEvent::AttributeAdded(identity, attribute_key, attribute_value));
						Ok(())
					}
				},
			}
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn remove_attribute(origin,identity: Vec<u8>,attribute_key: Vec<u8>,
				) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			match <Identity<T>>::get(&identity) {
				None => Err(Error::<T>::IdentityNotFound)?,
				Some(address) => {
					if address != who {
						return Err(Error::<T>::NotAuthorized)?
					} else{
						Attribute::remove((&identity, &attribute_key));
						Self::deposit_event(RawEvent::AttributeRemoved(identity, attribute_key));
						Ok(())
					}
				},
			}
		}

		

	}
}
