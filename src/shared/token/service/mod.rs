mod partial;

pub use partial::*;

pub struct TokenService {
    pub access: PartialTokenService,
    pub refresh: PartialTokenService,
}
