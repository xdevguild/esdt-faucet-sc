#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait EsdtFaucet {
    #[init]
    fn init(&self) {}

    #[payable("*")]
    #[endpoint(setLimit)]
    fn set_limit(&self, max_amount_per_tx: BigUint) {
        let payment = self.call_value().single_esdt();
        require!(
            payment.token_identifier.is_valid_esdt_identifier(),
            "The token should be a valid fungible ESDT"
        );

        self.max_tokens_per_tx(&EgldOrEsdtTokenIdentifier::parse(
            payment.token_identifier.into_managed_buffer(),
        ))
        .set(max_amount_per_tx);
    }

    #[endpoint(claim)]
    fn claim(&self, token_id: EgldOrEsdtTokenIdentifier, amount: BigUint) {
        require!(
            token_id.is_valid(),
            "The token identifier is not valid. Should be a valid ESDT token id."
        );
        require!(token_id.is_esdt(), "The token should be a fungible ESDT");

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let current_balance = self.blockchain().get_sc_balance(&token_id, 0);

        require!(current_balance > amount, "The amount you required is too much. There is no such amount of the token on smart contract");
        require!(
            current_timestamp > self.previously_claimed_timestamp(&caller).get() + (24 * 60),
            "You can claim only once per 24 hours"
        );

        self.previously_claimed_timestamp(&caller)
            .set(current_timestamp);

        let max_per_tx = self.max_tokens_per_tx(&token_id).get();

        require!(
            max_per_tx >= amount,
            "There is a max limit per transaction. It is {}",
            max_per_tx
        );

        self.send()
            .direct_esdt(&caller, &token_id.unwrap_esdt(), 0, &amount);
    }

    #[storage_mapper("previouslyClaimedTimestamp")]
    fn previously_claimed_timestamp(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getMaxTokensPerTransaction)]
    #[storage_mapper("maxTokensPerTx")]
    fn max_tokens_per_tx(&self, token_id: &EgldOrEsdtTokenIdentifier)
        -> SingleValueMapper<BigUint>;
}
