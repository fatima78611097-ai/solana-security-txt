#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "SPL Token Program",
    project_url: "https://solana.com",
    contacts: "email:security@anza.xyz,link:https://github.com/solana-labs/solana/security/advisories/new",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/solana-program/token",
    source_revision: "4fadd55",
    source_release: "3.4.0",
    auditors: "Peer Review (2022-08-04)",
    acknowledgements: "https://github.com/solana-labs/solana/security/advisories",
    expiry: "2027-12-31"
}
