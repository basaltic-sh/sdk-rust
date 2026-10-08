//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct KeyResponse {
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<Key>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Key {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags")]
    pub tags: Tags,

    #[serde(rename = "key_spec")]
    pub key_spec: KeySpec,

    #[serde(rename = "key_usage")]
    pub key_usage: KeyUsage,

    #[serde(rename = "state")]
    pub state: KeyState,
    /// Present and true on platform-owned envelope keys (credential master, JWT signer, …), which are visible but not yours to operate on. Omitted on customer keys. Console and CLI read it to hide the destructive actions.
    #[serde(rename = "system", default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    /// When deletion was requested. Null for active keys and historical pending deletions whose request time is unknown.
    #[serde(
        rename = "deleted_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub deleted_at: Option<crate::Nullable<String>>,
    /// Chosen whole-day window. Null for active keys and historical pending deletions whose window is unknown.
    #[serde(
        rename = "recovery_window_days",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub recovery_window_days: Option<crate::Nullable<i64>>,
    /// Set only while state=pending_deletion. The key (and its cryptographic material) is hard-deleted once now() reaches this timestamp; CancelKeyDeletion before then returns the key to state=disabled.
    #[serde(
        rename = "scheduled_purge_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub scheduled_purge_at: Option<crate::Nullable<String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type Tags = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeySpec {
    Aes256,
    Rsa2048,
    Rsa4096,
    EcdsaP256,
    Unknown(String),
}
impl KeySpec {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Aes256 => "aes-256",
            Self::Rsa2048 => "rsa-2048",
            Self::Rsa4096 => "rsa-4096",
            Self::EcdsaP256 => "ecdsa-p256",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for KeySpec {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeySpec {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "aes-256" => Self::Aes256,
            "rsa-2048" => Self::Rsa2048,
            "rsa-4096" => Self::Rsa4096,
            "ecdsa-p256" => Self::EcdsaP256,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyUsage {
    EncryptDecrypt,
    SignVerify,
    Unknown(String),
}
impl KeyUsage {
    pub fn as_str(&self) -> &str {
        match self {
            Self::EncryptDecrypt => "encrypt_decrypt",
            Self::SignVerify => "sign_verify",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for KeyUsage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeyUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "encrypt_decrypt" => Self::EncryptDecrypt,
            "sign_verify" => Self::SignVerify,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyState {
    Enabled,
    Disabled,
    PendingDeletion,
    Unknown(String),
}
impl KeyState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::PendingDeletion => "pending_deletion",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for KeyState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeyState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "enabled" => Self::Enabled,
            "disabled" => Self::Disabled,
            "pending_deletion" => Self::PendingDeletion,
            _ => Self::Unknown(value),
        })
    }
}

pub type CancelKeyDeletionResponse = KeyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateKeyRequestInput {
    /// Unique per account. Surfaces in the CRN (crn:kms:&lt;region&gt;:&lt;account&gt;:key/&lt;name&gt;) — letters, digits, dot, dash, underscore. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,

    #[serde(rename = "key_spec")]
    pub key_spec: KeySpecInput,
    /// Required for RSA specs (both encrypt_decrypt and sign_verify are valid). Defaults to encrypt_decrypt for AES, sign_verify for ECDSA.
    #[serde(rename = "key_usage", default, skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<KeyUsageInput>,
}
impl CreateKeyRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, key_spec: KeySpecInput) -> Self {
        Self {
            name,
            description: None,
            tags: None,
            key_spec,
            key_usage: None,
        }
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeySpecInput {
    Aes256,
    Rsa2048,
    Rsa4096,
    EcdsaP256,
    Unknown(String),
}
impl KeySpecInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Aes256 => "aes-256",
            Self::Rsa2048 => "rsa-2048",
            Self::Rsa4096 => "rsa-4096",
            Self::EcdsaP256 => "ecdsa-p256",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for KeySpecInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeySpecInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "aes-256" => Self::Aes256,
            "rsa-2048" => Self::Rsa2048,
            "rsa-4096" => Self::Rsa4096,
            "ecdsa-p256" => Self::EcdsaP256,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyUsageInput {
    EncryptDecrypt,
    SignVerify,
    Unknown(String),
}
impl KeyUsageInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::EncryptDecrypt => "encrypt_decrypt",
            Self::SignVerify => "sign_verify",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for KeyUsageInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeyUsageInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "encrypt_decrypt" => Self::EncryptDecrypt,
            "sign_verify" => Self::SignVerify,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateKeyBody = CreateKeyRequestInput;

pub type CreateKeyResponse = KeyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DecryptRequestInput {
    /// Base64-encoded ciphertext produced by Encrypt.
    #[serde(rename = "ciphertext")]
    pub ciphertext: String,
    /// Optional base64-encoded AAD. Must match what was supplied at Encrypt — different value fails the tag check.
    #[serde(rename = "aad", default, skip_serializing_if = "Option::is_none")]
    pub aad: Option<String>,
}
impl DecryptRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(ciphertext: String) -> Self {
        Self {
            ciphertext,
            aad: None,
        }
    }
}

pub type DecryptBody = DecryptRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DecryptResponse2 {
    /// Base64-encoded plaintext.
    #[serde(rename = "plaintext", default, skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
}

pub type DecryptResponse = DecryptResponse2;

pub type DisableKeyResponse = KeyResponse;

pub type EnableKeyResponse = KeyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptRequestInput {
    /// Base64-encoded plaintext.
    #[serde(rename = "plaintext")]
    pub plaintext: String,
    /// Optional base64-encoded additional authenticated data (AES-GCM AEAD). Must be supplied verbatim to Decrypt; mismatch fails the auth tag check. Ignored for asymmetric keys.
    #[serde(rename = "aad", default, skip_serializing_if = "Option::is_none")]
    pub aad: Option<String>,
}
impl EncryptRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(plaintext: String) -> Self {
        Self {
            plaintext,
            aad: None,
        }
    }
}

pub type EncryptBody = EncryptRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct EncryptResponse2 {
    /// Base64-encoded ciphertext. Opaque — store verbatim. For AES-GCM the layout is nonce(12) || ct || tag; for RSA-OAEP the standard PKCS#1 RSAES output.
    #[serde(
        rename = "ciphertext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ciphertext: Option<String>,
    /// CRN of the key the ciphertext was sealed under.
    #[serde(rename = "key_crn", default, skip_serializing_if = "Option::is_none")]
    pub key_crn: Option<String>,
}

pub type EncryptResponse = EncryptResponse2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GenerateDataKeyRequestInput {
    /// Size of the generated data key in bytes: 16 (AES-128), 32 (AES-256, the default) or 64 (HMAC-SHA512). No other size is supported — the HSM mints data keys at those three widths only, and any other value fails the operation.
    #[serde(
        rename = "number_of_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_bytes: Option<i64>,
}

pub type GenerateDataKeyBody = GenerateDataKeyRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GenerateDataKeyResponse2 {
    /// Base64-encoded plaintext data key. Use immediately, then drop — callers must NOT persist this. Re-derive it on demand by calling Decrypt with the stored ciphertext.
    #[serde(rename = "plaintext", default, skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<String>,
    /// Base64-encoded data key wrapped under the KMS key. Safe to store at rest alongside the data the key protects.
    #[serde(
        rename = "ciphertext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ciphertext: Option<String>,
}

pub type GenerateDataKeyResponse = GenerateDataKeyResponse2;

pub type GetKeyResponse = KeyResponse;

pub type GetKeyResource = Key;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetKeyScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<KeyStateInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyStateInput {
    Enabled,
    Disabled,
    PendingDeletion,
    Unknown(String),
}
impl KeyStateInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::PendingDeletion => "pending_deletion",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for KeyStateInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeyStateInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "enabled" => Self::Enabled,
            "disabled" => Self::Disabled,
            "pending_deletion" => Self::PendingDeletion,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListKeysParameters {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<KeyStateInput>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListKeysQuery = ListKeysParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct KeyListResponse {
    #[serde(rename = "keys", default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<Key>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PaginationMeta {
    /// Total number of items
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Number of items per page
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque cursor for the next page. Pass it back as the `marker` query parameter; treat it as a token, not a value to parse.
    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// Whether there are more items
    #[serde(rename = "has_more", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

pub type ListKeysResponse = KeyListResponse;

pub type ListKeysItem = Key;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ScheduleKeyDeletionRequestInput {
    /// How long the key sits in pending_deletion before it is hard-deleted. Matches AWS KMS bounds; the deletion can be cancelled at any point inside the window.
    #[serde(
        rename = "recovery_window_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_window_days: Option<i64>,
}

pub type ScheduleKeyDeletionBody = ScheduleKeyDeletionRequestInput;

pub type ScheduleKeyDeletionResponse = KeyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SignRequestInput {
    /// Base64-encoded message to sign. The service hashes it via SHA-256 server-side, so pass the raw payload — do not pre-hash.
    #[serde(rename = "message")]
    pub message: String,

    #[serde(
        rename = "signing_algorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_algorithm: Option<SigningAlgorithmInput>,
}
impl SignRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(message: String) -> Self {
        Self {
            message,
            signing_algorithm: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigningAlgorithmInput {
    RSASSAPSSSHA256,
    RSASSAPKCS1V15SHA256,
    ECDSASHA256,
    Unknown(String),
}
impl SigningAlgorithmInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::RSASSAPSSSHA256 => "RSASSA_PSS_SHA_256",
            Self::RSASSAPKCS1V15SHA256 => "RSASSA_PKCS1_V1_5_SHA_256",
            Self::ECDSASHA256 => "ECDSA_SHA_256",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SigningAlgorithmInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SigningAlgorithmInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "RSASSA_PSS_SHA_256" => Self::RSASSAPSSSHA256,
            "RSASSA_PKCS1_V1_5_SHA_256" => Self::RSASSAPKCS1V15SHA256,
            "ECDSA_SHA_256" => Self::ECDSASHA256,
            _ => Self::Unknown(value),
        })
    }
}

pub type SignBody = SignRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SignResponse2 {
    /// Base64-encoded signature. RSA-PSS: PKCS#1 octet string with saltLen=hashLen=32. ECDSA: ASN.1 DER (r,s) tuple per ANSI X9.62.
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    #[serde(
        rename = "signing_algorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_algorithm: Option<SigningAlgorithm>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigningAlgorithm {
    RSASSAPSSSHA256,
    RSASSAPKCS1V15SHA256,
    ECDSASHA256,
    Unknown(String),
}
impl SigningAlgorithm {
    pub fn as_str(&self) -> &str {
        match self {
            Self::RSASSAPSSSHA256 => "RSASSA_PSS_SHA_256",
            Self::RSASSAPKCS1V15SHA256 => "RSASSA_PKCS1_V1_5_SHA_256",
            Self::ECDSASHA256 => "ECDSA_SHA_256",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SigningAlgorithm {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SigningAlgorithm {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "RSASSA_PSS_SHA_256" => Self::RSASSAPSSSHA256,
            "RSASSA_PKCS1_V1_5_SHA_256" => Self::RSASSAPKCS1V15SHA256,
            "ECDSA_SHA_256" => Self::ECDSASHA256,
            _ => Self::Unknown(value),
        })
    }
}

pub type SignResponse = SignResponse2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateKeyRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateKeyBody = UpdateKeyRequestInput;

pub type UpdateKeyResponse = KeyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VerifyRequestInput {
    /// Base64-encoded original message.
    #[serde(rename = "message")]
    pub message: String,
    /// Base64-encoded signature produced by Sign.
    #[serde(rename = "signature")]
    pub signature: String,

    #[serde(
        rename = "signing_algorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_algorithm: Option<SigningAlgorithmInput>,
}
impl VerifyRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(message: String, signature: String) -> Self {
        Self {
            message,
            signature,
            signing_algorithm: None,
        }
    }
}

pub type VerifyBody = VerifyRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VerifyResponse2 {
    /// True if the signature verifies against the supplied message under the key's public half. A clean mismatch returns false with no error; backend / parameter failures surface as a normal error response.
    #[serde(
        rename = "signature_valid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signature_valid: Option<bool>,
}

pub type VerifyResponse = VerifyResponse2;
