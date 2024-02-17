use secrecy::Secret;
use sqlx::{postgres::PgTypeInfo, Database, Decode, Encode, Postgres, Type};

use crate::domain::user::{Email, Password};

impl<'r> Decode<'r, Postgres> for Email {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let email = String::decode(value)?;
        Self::try_from(email)
            .map_err(|e| sqlx::error::BoxDynError::from(format!("failed to decode email: {:?}", e)))
    }
}

impl<'q> Encode<'q, Postgres> for Email {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.as_ref().encode_by_ref(buf)
    }
}

impl Type<Postgres> for Email {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_name("VARCHAR")
    }
}

impl<'r> Decode<'r, Postgres> for Password {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self::from_raw(Secret::new(String::decode(value)?)))
    }
}

impl<'q> Encode<'q, Postgres> for Password {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.as_ref().encode_by_ref(buf)
    }
}

impl Type<Postgres> for Password {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_name("TEXT")
    }
}
