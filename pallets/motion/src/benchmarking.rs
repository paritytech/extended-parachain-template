#![cfg(feature = "runtime-benchmarks")]

use super::*;
#[allow(unused)]
use crate::Pallet as Motion;
use frame_benchmarking::v2::*;
use frame_support::traits::EnsureOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks(
where
	<T as Config>::RuntimeCall: From<frame_system::Call<T>>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn simple_majority() {
		let call: <T as Config>::RuntimeCall = frame_system::Call::remark { remark: vec![] }.into();
		let origin = <T as Config>::SimpleMajorityOrigin::try_successful_origin().unwrap();
		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, Box::new(call));

		assert_last_event::<T>(Event::DispatchSimpleMajority { motion_result: Ok(()) }.into())
	}

	#[benchmark]
	fn super_majority() {
		let call: <T as Config>::RuntimeCall = frame_system::Call::remark { remark: vec![] }.into();
		let origin = <T as Config>::SuperMajorityOrigin::try_successful_origin().unwrap();
		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, Box::new(call));

		assert_last_event::<T>(Event::DispatchSuperMajority { motion_result: Ok(()) }.into())
	}

	#[benchmark]
	fn unanimous() {
		let call: <T as Config>::RuntimeCall = frame_system::Call::remark { remark: vec![] }.into();
		let origin = <T as Config>::UnanimousOrigin::try_successful_origin().unwrap();
		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, Box::new(call));

		assert_last_event::<T>(Event::DispatchUnanimous { motion_result: Ok(()) }.into())
	}
	impl_benchmark_test_suite!(Motion, crate::mock::new_test_ext(), crate::mock::Test);
}
