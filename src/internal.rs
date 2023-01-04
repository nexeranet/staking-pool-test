use crate::*;

impl Contract {

    pub(crate) fn internal_deposit(&mut self) -> u128 {
        let amount = env::attached_deposit();
        self.last_total_balance += amount;
        amount
    }

    /// Distributes rewards after the new epoch. It's automatically called before every action.
    /// Returns true if the current  block height is different from the last epoch height.
    pub(crate) fn internal_ping(&mut self) {
        let block_height = env::block_height();
        if self.last_block_height == block_height {
            return;
        }
        self.last_block_height = block_height;
        // New total amount (both locked and unlocked balances).
        // NOTE: We need to subtract `attached_deposit` in case `ping` called from `deposit` call
        // since the attached deposit gets included in the `account_balance`, and we have not
        // accounted it yet.
        let total_balance =
            env::account_locked_balance() + env::account_balance() - env::attached_deposit();

        self.last_total_balance = total_balance;
    }
}
