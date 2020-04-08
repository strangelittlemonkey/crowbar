use crate::providers::okta::response::Links;
use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase", tag = "factorType")]
pub enum Factor {
    #[serde(rename_all = "camelCase")]
    Push {
        id: String,
        provider: FactorProvider,
        status: Option<FactorStatus>,
        profile: PushFactorProfile,
        #[serde(rename = "_links")]
        links: Option<HashMap<String, Links>>,
    },
    #[serde(rename_all = "camelCase")]
    Sms {
        id: String,
        provider: FactorProvider,
        status: Option<FactorStatus>,
        profile: SmsFactorProfile,
        #[serde(rename = "_links")]
        links: Option<HashMap<String, Links>>,
    },

    #[serde(rename = "token:software:totp", rename_all = "camelCase")]
    Totp {
        id: String,
        provider: FactorProvider,
        status: Option<FactorStatus>,
        profile: TokenFactorProfile,
        #[serde(rename = "_links")]
        links: Option<HashMap<String, Links>>,
    },
    WebAuthn {
        id: String,
        provider: FactorProvider,
        status: Option<FactorStatus>,
        profile: WebAuthnFactorProfile,
        #[serde(rename = "_links")]
        links: Option<HashMap<String, Links>>,
        #[serde(rename = "_embedded")]
        embedded: Option<FactorChallenge>,
    },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "")]
pub struct FactorChallenge {
    pub challenge: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactorProvider {
    Okta,
    Google,
    Fido,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactorStatus {
    NotSetup,
    PendingActivation,
    Enrolled,
    Active,
    Inactive,
    Expired,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FactorVerification {
    pass_code: String,
    next_pass_code: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmsFactorProfile {
    phone_number: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PushFactorProfile {
    credential_id: String,
    device_type: String,
    name: String,
    platform: String,
    version: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallFactorProfile {
    phone_number: String,
    phone_extension: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuestionFactorProfile {
    question: String,
    question_text: String,
    answer: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenFactorProfile {
    credential_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebFactorProfile {
    credential_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebAuthnFactorProfile {
    credential_id: String,
    authenticator_name: String,
}

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Factor::Push { ref profile, .. } => write!(f, "Okta Verify Push to {}", profile.name),
            Factor::Sms { ref profile, .. } => write!(f, "Okta SMS to {}", profile.phone_number),
            Factor::Totp {
                // Okta identifies any other TOTP provider as "Google"
                provider: FactorProvider::Google,
                ..
            } => write!(f, "Software TOTP"),
            Factor::Totp { .. } => write!(f, "Okta Verify TOTP"),
            Factor::WebAuthn { ref profile, .. } => {
                write!(f, "WebAuthn with {}", profile.authenticator_name)
            }
        }
    }
}
