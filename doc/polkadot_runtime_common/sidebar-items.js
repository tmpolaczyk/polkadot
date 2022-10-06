window.SIDEBAR_ITEMS = {"constant":[["AVERAGE_ON_INITIALIZE_RATIO","We assume that an on-initialize consumes 1% of the weight on average, hence a single extrinsic will not be allowed to consume more than `AvailableBlockRatio - 1%`."],["MAXIMUM_BLOCK_WEIGHT","The storage proof size is not limited so far."],["NORMAL_DISPATCH_RATIO","We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by  Operational  extrinsics."]],"enum":[["BalancesCall","Contains one variant per dispatchable that can be called by an extrinsic."],["StakerStatus","Indicates the initial status of the staker."],["TimestampCall","Contains one variant per dispatchable that can be called by an extrinsic."]],"macro":[["impl_elections_weights","Implements the weight types for the elections module and a specific runtime. This macro should not be called directly; use [`impl_runtime_weights`] instead."],["impl_runtime_weights","Implements the weight types for a runtime. It expects the passed runtime constants to contain a `weights` module. The generated weight types were formerly part of the common runtime but are now runtime dependant."],["prod_or_fast","Macro to set a value (e.g. when using the `parameter_types` macro) to either a production value or to an environment variable or testing value (in case the `fast-runtime` feature is selected). Note that the environment variable is evaluated at compile time."]],"mod":[["assigned_slots","This pallet allows to assign permanent (long-lived) or temporary (short-lived) parachain slots to paras, leveraging the existing parachain slot lease mechanism. Temporary slots are given turns in a fair (though best-effort) manner. The dispatchables must be called from the configured origin (typically `Sudo` or a governance origin). This pallet should not be used on a production relay chain, only on a test relay chain (e.g. Rococo)."],["auctions","Auctioning system to determine the set of Parachains in operation. This includes logic for the auctioning mechanism and for reserving balance as part of the “payment”. Unreserving the balance happens elsewhere."],["claims","Pallet to process claims from Ethereum addresses."],["crowdloan","Parachain `Crowdloaning` pallet"],["elections","Code for elections."],["impls","Auxiliary `struct`/`enum`s for polkadot runtime."],["paras_registrar","Pallet to handle parathread/parachain registration and related fund management. In essence this is a simple wrapper around `paras`."],["paras_sudo_wrapper","A simple wrapper allowing `Sudo` to call into `paras` routines."],["purchase","Pallet to process purchase of DOTs."],["slot_range","The `SlotRange` struct which succinctly handles the 36 values that represent all sub ranges between 0 and 7 inclusive."],["slots","Parathread and parachains leasing system. Allows para IDs to be claimed, the code and data to be initialized and parachain slots (i.e. continuous scheduling) to be leased. Also allows for parachains and parathreads to be swapped."],["traits","Traits used across pallets for Polkadot."],["xcm_sender","XCM sender for relay chain."]],"struct":[["AdjustmentVariable","The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to change the fees more rapidly."],["AssignmentSessionKeyPlaceholder","A placeholder since there is currently no provided session key handler for parachain validator keys."],["BalanceToU256","Convert a balance to an unsigned 256-bit number, use in nomination pools."],["BlockHashCount",""],["BlockLength","Maximum length of block. Up to 5MB."],["MaximumMultiplier","The maximum amount of the multiplier."],["MinimumMultiplier","Minimum amount of the multiplier. This value cannot be too low. A test case should ensure that combined with `AdjustmentVariable`, we can recover from the minimum. See `multiplier_can_grow_from_zero`."],["ParachainSessionKeyPlaceholder","A placeholder since there is currently no provided session key handler for parachain validator keys."],["StakingBenchmarkingConfig","A reasonable benchmarking config for staking pallet."],["TargetBlockFullness","The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less than this will decrease the weight and more will increase."],["U256ToBalance","Convert an unsigned 256-bit number to balance, use in nomination pools."]],"trait":[["Bounded","Numbers which have upper and lower bounds"],["BuildStorage","Complex storage builder stuff."]],"type":[["CurrencyToVote","The type used for currency conversion."],["NegativeImbalance",""],["SlowAdjustingFeeUpdate","Parameterized slow adjusting fee updated based on https://research.web3.foundation/en/latest/polkadot/overview/2-token-economics.html#-2.-slow-adjusting-mechanism"]]};