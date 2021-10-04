elrond_wasm::imports!();

const BLOCK_RAND_SEED_LEN: usize = 48;
pub type BlockRandomSeed = Box<[u8; BLOCK_RAND_SEED_LEN]>;

const USIZE_BYTES: usize = 4;

pub struct Random<CA: CryptoApi> {
    api: CA,
    pub seed: H256,
    pub index: usize,
}

impl<CA: CryptoApi> Random<CA> {
    #[allow(clippy::boxed_local)]
    pub fn from_seeds(
        api: CA,
        prev_block_seed: BlockRandomSeed,
        current_block_seed: BlockRandomSeed,
    ) -> Self {
        let mut summed_seeds = BlockRandomSeed::new([0u8; BLOCK_RAND_SEED_LEN]);
        for i in 0..BLOCK_RAND_SEED_LEN {
            summed_seeds[i] = prev_block_seed[i].wrapping_add(current_block_seed[i]);
        }

        Self {
            seed: api.sha256(&summed_seeds[..]),
            index: 0,
            api,
        }
    }

    pub fn from_hash(api: CA, hash: H256, index: usize) -> Self {
        Self {
            seed: hash,
            index,
            api,
        }
    }

    pub fn next_usize(&mut self) -> usize {
        if self.index + USIZE_BYTES > H256::len_bytes() {
            self.hash_seed();
        }

        let bytes = &self.seed.as_bytes()[self.index..(self.index + USIZE_BYTES)];
        let rand = usize::top_decode(bytes).unwrap_or_default();

        self.index += USIZE_BYTES;

        rand
    }

    /// Range is [min, max)
    pub fn next_usize_in_range(&mut self, min: usize, max: usize) -> usize {
        let rand = self.next_usize();

        if min >= max {
            min
        } else {
            min + rand % (max - min)
        }
    }
}

impl<CA: CryptoApi> Random<CA> {
    fn hash_seed(&mut self) {
        self.seed = self.api.sha256(self.seed.as_bytes());
        self.index = 0;
    }
}
