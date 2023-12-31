use crate::*;

// This will be included to the contract body
impl Contract {
    pub fn is_owner(&self) -> bool {
        env::predecessor_account_id() == self.owner
    }

    pub fn is_admin(&self) -> bool {
        self.admins.contains(&env::predecessor_account_id())
    }

    pub fn is_owner_or_admin(&self) -> bool {
        self.is_owner() || self.is_admin()
    }
}
