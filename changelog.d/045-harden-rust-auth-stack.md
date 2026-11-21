Hardened Rust authentication. Password changes enforce a per-user attempt limit and verify the current password before checking the new one. Password requirements reflect the strength and blocklist checks. Repositories that enable atomic hash replacement can upgrade older hashes during successful login. Password breach checks reuse the service's HTTP client.

Refreshing a session for a suspended or deleted account revokes its access token. TOTP verification checks later counters when a code matches an earlier replayed counter. Role assignment rejects unknown roles, bearer extractors reject empty tokens, and the Postgres auth-state store can purge expired rows.

Breaking for consumers using the WebAuthn HTTP start helpers. Construct start inputs from server-loaded credentials. The passkey start callback receives a `PasskeyAuthenticationState`. Persist its variant and use the matching finish helper. Discoverable finish requires a trusted lookup keyed by both user ID and credential ID. Refresh any exhaustive matches of `RoleHierarchyError` to handle `UnknownRole`.

Google OAuth validates redirect URIs at construction. Use HTTPS or a loopback HTTP callback for local development. Sign-ins still resolve existing Google links by subject. Linking an existing local account by email requires `with_verified_email_auto_linking()` and a Google-verified email.
