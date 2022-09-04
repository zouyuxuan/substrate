use crate::{mock::*, Proofs, Config,Error};
use frame_support::{ assert_ok, BoundedVec, assert_noop};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));

		let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1,frame_system::Pallet::<Test>::block_number()))
		);
	
	});
}

#[test]
fn delete_claim_works(){
	new_test_ext().execute_with(||{
		// 创建claim
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_ok!( PoeModule::create_claim(Origin::signed(1),claim.clone()));
		assert_ok!(PoeModule::delete_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&bounded_claim), None)
	})
}

// 删除一个不存在的存证
#[test]
fn delete_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_noop!(PoeModule::delete_claim(Origin::signed(1), claim.clone()) ,
            Error::<Test>::ClaimNotExist);
    })
}
// 转移存证
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

		let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2));
        assert_eq!(Proofs::<Test>::get(&bounded_claim), Some((2, frame_system::Pallet::<Test>::block_number())));
    })
}