//! A set of constant values used in selendra parachain runtime.

/// Money matters.
pub mod currency {
	use selendra_primitives::Balance;

	pub const UNIT: Balance = 1_000_000_000_000;
	pub const MILLIUNIT: Balance = 1_000_000_000;
	pub const MICROUNIT: Balance = 1_000_000;
}

/// Time.
pub mod time {
	use selendra_primitives::{BlockNumber, Moment};

	/// This determines the average expected block time that we are targeting.
	/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
	/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
	/// up by `pallet_aura` to implement `fn slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLISECS_PER_BLOCK: Moment = 12000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
}

/// runtime common
pub mod common {
	use crate::MILLIUNIT;
	use frame_support::{
		parameter_types,
		weights::{
			constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_PER_SECOND},
			DispatchClass, Weight, WeightToFeePolynomial,
		},
	};
	use frame_system::limits::{BlockLength, BlockWeights};
	use selendra_primitives::Balance;
	use smallvec::smallvec;
	use sp_runtime::Perbill;

	/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
	pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

	/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
	/// used to limit the maximal weight of a single extrinsic.
	pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

	/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
	/// `Operational` extrinsics.
	pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

	/// We allow for 0.5 of a second of compute with a 12 second average block time.
	pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;

	parameter_types! {
		pub RuntimeBlockLength: BlockLength =
			BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);

		pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
			.base_block(BlockExecutionWeight::get())
			.for_class(DispatchClass::all(), |weights| {
				weights.base_extrinsic = ExtrinsicBaseWeight::get();
			})
			.for_class(DispatchClass::Normal, |weights| {
				weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
			})
			.for_class(DispatchClass::Operational, |weights| {
				weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
				// Operational transactions have some extra reserved space, so that they
				// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
				weights.reserved = Some(
					MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
				);
			})
			.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
			.build_or_panic();
	}

	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
	/// node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
	///   - `[Balance::min, Balance::max]`
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> frame_support::weights::WeightToFeeCoefficients<Self::Balance> {
			// in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
			// in our template, we map to 1/10 of that, or 1/10 MILLIUNIT
			let p = MILLIUNIT / 10;
			let q = 100 * Balance::from(ExtrinsicBaseWeight::get());
			smallvec![frame_support::weights::WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}
}
