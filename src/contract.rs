use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
// use near_sdk::serde::de::DeserializeOwned;
use near_sdk::{env, near_bindgen, AccountId, Balance, BlockHeight, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    /// The account ID of the owner who's running the staking validator node.
    /// NOTE: This is different from the current account ID which is used as a validator account.
    /// The owner of the staking pool can change staking public key and adjust reward fees.
    pub owner_id: AccountId,
    pub last_total_balance: Balance,
    pub last_block_height: BlockHeight,
    pub last_withdraw:  BlockHeight,
    pub last_unstake: BlockHeight,
    pub num_blocks_to_withdraw: BlockHeight,
    pub num_blocks_to_unstake: BlockHeight,
}

impl Default for Contract {
    fn default() -> Self {
        let last_height = env::block_height();
        Self {
            owner_id: env::predecessor_account_id(),
            last_block_height: env::block_height(),
            last_total_balance: env::account_balance(),
            last_withdraw:  last_height,
            last_unstake: last_height,
            num_blocks_to_withdraw: 60,
            num_blocks_to_unstake: 60,
        }
    }
}

#[near_bindgen]
impl Contract {
    /// Initializes contract
    pub(crate) fn new(owner_id: AccountId) -> Self {
        // let account_balance = env::account_balance();
        let last_height = env::block_height();
        Self {
            owner_id,
            last_block_height: env::block_height(),
            last_total_balance: env::account_balance(),
            last_withdraw:  last_height,
            last_unstake: last_height,
            num_blocks_to_withdraw: 60,
            num_blocks_to_unstake: 60,
        }
    }
    /// Init contract func
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        Self::new(owner_id)
    }

    /// Deposits the attached amount into the inner account of the predecessor and stakes it.
    #[payable]
    pub fn deposit_and_stake(&mut self) {
        self.internal_ping();
        let account_id = env::predecessor_account_id();
        let amount = self.internal_deposit();

        env::log_str(&format!(
            "@{} deposited {}. New balance is {}",
            account_id, amount, self.last_total_balance
        ));
    }


    /// Stakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough unstaked balance.
    pub fn stake(&mut self, amount: U128) {
        self.internal_ping();
        let account_id = env::predecessor_account_id();
        let amount: Balance = amount.into();
        self.last_total_balance += amount;

        env::log_str(&format!(
            "@{} staked {}. New balance is {}",
            account_id, amount, self.last_total_balance
        ));
    }

    /// Unstakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough staked balance.
    /// The new total unstaked balance will be available for withdrawal in four epochs.
    pub fn unstake(&mut self, amount: U128) {
        self.internal_ping();
        let amount: Balance = amount.into();
        assert!(
            self.last_total_balance > 0,
            "The contract doesn't have staked balance"
        );
        assert!(amount > 0, "Unstaking amount should be positive");
        assert!(
            self.last_unstake + self.num_blocks_to_unstake <  self.last_block_height,
            "The unstaked balance is not yet available due to unstaking delay"

        );
        self.last_unstake = env::block_height();
    }

    /// Withdraws the non staked balance for given account.
    /// It's only allowed if the `unstake` action was not performed in the four most recent epochs.
    pub fn withdraw(&mut self, amount: U128) {
        self.internal_ping();
        let account_id = env::predecessor_account_id();
        let amount: Balance = amount.into();
        assert!(
            self.last_total_balance > 0,
            "The contract doesn't have staked balance"
        );
        assert!(amount > 0, "Unstaking amount should be positive");
        assert!(
            self.last_withdraw + self.num_blocks_to_withdraw <  self.last_block_height,
            "The withdraw balance is not yet available due to withdraw delay"
        );
        self.last_withdraw = env::block_height();
        self.last_total_balance -= amount;
        Promise::new(account_id).transfer(amount);
    }
    /// UTILS
    /// Asserts that the method was called by the owner.
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }

    /// Returns account ID of the staking pool owner.
    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn set_settings(&mut self, num_unstake: u64, num_withdaw: u64) {
        self.assert_owner();
        self.num_blocks_to_unstake = num_unstake;
        self.num_blocks_to_withdraw = num_withdaw;
    }
}
