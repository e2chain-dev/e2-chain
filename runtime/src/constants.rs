pub mod currency {
    use node_primitives::Balance;

    pub const MILLICENTS: Balance = 1_000_000_000;
    pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
    pub const DPR: Balance = 100 * CENTS;
    pub const DOLLARS: Balance = DPR;
}
