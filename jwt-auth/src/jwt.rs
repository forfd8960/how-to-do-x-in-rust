use jwt_simple::{claims::Claims, prelude::*};

use crate::models::user::User;

const JWT_DURATION: u64 = 64 * 64 * 24 * 7;
const JWT_ISS: &str = "how-to-jwt-auth";
const JWT_AUD: &str = "jwt-user";

#[derive(Clone)]
pub struct EncodingKey(Ed25519KeyPair);

#[derive(Debug, Clone)]
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, jwt_simple::Error> {
        let claims: JWTClaims<User> = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));

        let claims = claims.with_issuer(JWT_ISS).with_audience(JWT_AUD);
        self.0.sign(claims)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    pub fn verify(&self, token: &str) -> Result<User, jwt_simple::Error> {
        let opts = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            ..Default::default()
        };

        let claims = self.0.verify_token::<User>(token, Some(opts))?;
        Ok(claims.custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::User;
    use anyhow::Result;

    #[test]
    fn test_generate_keys() -> Result<()> {
        let encoding_pem = include_str!("../keys/private.pem");
        let decoding_pem = include_str!("../keys/public.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

        let user = User::new("AlexZ".to_string(), "alex@example.com".to_string());

        let token = ek.sign(user.clone())?;
        println!("sign token: {:?}", token);

        let verify_user = dk.verify(&token)?;
        println!("verify_user: {:?}", verify_user);

        assert_eq!(user, verify_user);
        Ok(())
    }
}
