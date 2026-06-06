mod core;
mod encoding;
mod http;
mod storage;

use underlay_auth::AuthResult;
use url::Url;
use webauthn_rs::prelude::{Webauthn, WebauthnBuilder};

use crate::{WebAuthnConfig, WebAuthnError};

#[derive(Debug, Clone)]
pub struct WebAuthnService {
    pub(crate) inner: Webauthn,
}

impl WebAuthnService {
    pub fn new(config: WebAuthnConfig) -> AuthResult<Self> {
        let origin = Url::parse(config.rp_origin()).map_err(|_| WebAuthnError::InvalidConfig)?;

        let builder = WebauthnBuilder::new(config.rp_id(), &origin)
            .map_err(|_| WebAuthnError::InvalidConfig)?
            .rp_name(config.rp_name());

        let inner = builder.build().map_err(|_| WebAuthnError::InvalidConfig)?;

        Ok(Self { inner })
    }
}
