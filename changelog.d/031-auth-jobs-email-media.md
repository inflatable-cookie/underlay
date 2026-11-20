Hardened Google account linking and refresh-token logout, made recurring job scheduling and stalled-job dead lettering atomic, preserved custom SMTP and SES headers with input validation, fixed opportunistic STARTTLS, and rejected zero-sized alpha thumbnails.

This changes the Rust auth adapter contracts. Google callback repositories must implement `GoogleOAuthCredentialRepository` to update the encrypted secret atomically by credential ID and Google subject. Enforce one Google link per user. An initial sign-in without offline access stores `NO_GOOGLE_REFRESH_TOKEN` in the existing secret field. Handle that marker as an absent refresh token.

Session repositories must implement `revoke_session_if_current` with a single conditional write that checks the active state, owner, fingerprint, token ID, and version. The rotation write must also require an active session. Deployments using stalled-job recovery must have the existing jobs dead-letter migration applied.
