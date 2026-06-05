use std::borrow::Cow;

use underlay_auth::{AuthError, AuthResult};

use crate::TotpService;

impl TotpService {
    pub fn provisioning_uri(&self, account_name: &str, secret_base32: &str) -> String {
        let issuer = url_escape(&self.config.issuer);
        let account = url_escape(account_name);
        let label = format!("{}:{}", issuer, account);

        format!(
            "otpauth://totp/{}?secret={}&issuer={}&algorithm={}&digits={}&period={}",
            label,
            secret_base32,
            issuer,
            self.config.algorithm.as_str(),
            self.config.digits,
            self.config.period_seconds
        )
    }

    pub fn qr_svg(&self, otpauth_uri: &str) -> AuthResult<String> {
        let code = qrcode::QrCode::new(otpauth_uri.as_bytes())
            .map_err(|e| AuthError::Internal(format!("failed to build QR code: {e}")))?;

        let svg = code
            .render::<qrcode::render::svg::Color>()
            .min_dimensions(200, 200)
            .quiet_zone(true)
            .build();

        Ok(svg)
    }
}

fn url_escape(input: &str) -> Cow<'_, str> {
    if input
        .bytes()
        .all(|b| matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~'))
    {
        return Cow::Borrowed(input);
    }

    let mut out = String::new();
    for b in input.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }

    Cow::Owned(out)
}
