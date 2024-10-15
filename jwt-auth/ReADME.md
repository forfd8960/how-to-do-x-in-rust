# Generate and Veify JWT with `jwt-simple`

## Generate private and public key

```sh
openssl genpkey -algorithm ed25519 -out private.pem

✗ cat private.pem
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEICOsPn3KRn4b6dSDm6BsWTMPLxr4DsydA72X2A7xhPHj
-----END PRIVATE KEY-----


openssl pkey -in private.pem -pubout -out public.pem
```

## Vertify

```rust
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
```
