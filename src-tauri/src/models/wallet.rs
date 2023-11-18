use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct WalletResponse {
    pub account_id: String,
    pub premium_balance: i64,
    pub total_purchased_coins: i64,
}
