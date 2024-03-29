use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    //"iss": "dummy-issuer",
    //"aud": "dummy-audience",
    //"exp": 1711748924,
    //"iat": 1711745324,
    //"sub": "dummy-sub",
    //"uid": "a175bce7-3e5b-4863-92a1-efc1991ae6fd:efgi5eh5pwij",
    pub claims: Claims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    //"brandId": "dummy-brand-id",
    //"egymAccountId": "dummy-egym-account-id",
    //"membershipId": "dummy-membership-id",
    #[serde(rename = "mmsMembershipIds")]
    pub user_ids: Vec<String>,
}
