use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

pub struct SolanaContext {
    pub rpc_url: String,
    pub payer: Keypair,
}

impl SolanaContext {
    pub fn new(rpc_url: String) -> Self {
        Self {
            rpc_url,
            payer: Keypair::new(),
        }
    }

    pub fn get_payer_pubkey(&self) -> Pubkey {
        use solana_sdk::signer::Signer;
        self.payer.pubkey()
    }
}
