use frame_support::sp_runtime::KeyTypeId;
pub const CERTUS_ORACLE: KeyTypeId = KeyTypeId(*b"certus");

// Importación de app_crypto para definir la clave pública, la firma y el par de claves de la autoridad.
pub mod sr25519 {
    mod app_sr25519 {
        // Importación de tipos de claves y firma del complemento sr25519 de sp_application_crypto.
        use crate::crypto::CERTUS_ORACLE;
        use sp_application_crypto::{app_crypto, sr25519};
        use sp_std::convert::TryFrom;
        // Definición de la aplicación de la criptografía SR25519.
        app_crypto!(sr25519, CERTUS_ORACLE);
    }

    // Uso de la macro with_pair para definir un tipo de par de claves para la autoridad.
    sp_application_crypto::with_pair! {
        /// Un par de claves de autoridad `Certus` que utiliza S/R 25519 como su criptografía.
        pub type AuthorityPair = app_sr25519::Pair;
    }

    // Definición de la firma de autoridad utilizando la criptografía SR25519.
    pub type AuthoritySignature = app_sr25519::Signature;

    // Definición de la identificación de la autoridad utilizando la criptografía SR25519.
    pub type AuthorityId = app_sr25519::Public;
}