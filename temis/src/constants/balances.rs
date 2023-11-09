use near_workspaces::types::NearToken;

pub const EMPTY_BALANCE: NearToken = NearToken::from_near(0);

pub const USER_ACCOUNT_BALANCE: NearToken =
    NearToken::from_yoctonear(5_000_000_000_000_000_000_000_000);

pub const CONTRACT_ACCOUNT_BALANCE: NearToken =
    NearToken::from_yoctonear(20_000_000_000_000_000_000_000_000);
