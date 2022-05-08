use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn should_not_throw_errors() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic
		// create identity "czareko" for accountId 1
		let identity = "czareko".as_bytes().to_vec();
		assert_ok!(IdentityModule::create_identity(Origin::signed(1), "czareko".as_bytes().to_vec() ));
		// Read pallet storage and assert an expected result.
		assert_eq!(IdentityModule::get_identity(&identity), Some(1));

		let attribute_key = "name".as_bytes().to_vec();
		let attribute_value = "cezary olborski".as_bytes().to_vec();

		// add attribute name => cezary olborski
		assert_ok!(IdentityModule::add_attribute(Origin::signed(1), "czareko".as_bytes().to_vec(), "name".as_bytes().to_vec(), "cezary olborski".as_bytes().to_vec()));
		// check attribute value
		assert_eq!(IdentityModule::get_attribute((&identity, &attribute_key)), attribute_value);

		// Remmove attribute
		assert_ok!(IdentityModule::remove_attribute(Origin::signed(1), "czareko".as_bytes().to_vec(), "name".as_bytes().to_vec()));
		// after removing, attribute value should be blank
		assert_eq!(IdentityModule::get_attribute((identity, attribute_key)), "".as_bytes().to_vec());

	});
}

#[test]
fn check_for_errors() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let identity = "czareko".as_bytes().to_vec();
		assert_ok!(IdentityModule::create_identity(Origin::signed(1), "czareko".as_bytes().to_vec() ));
		// Read pallet storage and assert an expected result.
		assert_eq!(IdentityModule::get_identity(&identity), Some(1));

		// Should throw error as identity "prasad" is already claimed
		let identity = "czareko".as_bytes().to_vec();
		assert_noop!(
			IdentityModule::create_identity(
				Origin::signed(2),
				"czareko".as_bytes().to_vec()
			),
			Error::<Test>::IdentityAlreadyClaimed
		);

		// add_attribute signed by different identity (2)
		// should throw NotAuthorized error
		assert_noop!(
			IdentityModule::add_attribute(
				Origin::signed(2),
				"czareko".as_bytes().to_vec(),
				"name".as_bytes().to_vec(),
				"cezary olborski".as_bytes().to_vec()
			),
			Error::<Test>::NotAuthorized
		);

		// Attribute value should be blank
		assert_eq!(IdentityModule::get_attribute((identity, "name".as_bytes().to_vec())), "".as_bytes().to_vec());
	});
}
