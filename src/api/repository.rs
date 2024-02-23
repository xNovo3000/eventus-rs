pub mod user {
    
    use std::error::Error;

    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use diesel_async::AsyncPgConnection;

    use crate::api::model::User;
    use crate::api::schema::eventus::user_::dsl::*;

    pub async fn load_by_username(connection: &mut AsyncPgConnection, username_: &str) -> Result<Option<User>, Box<dyn Error>> {
        // Read user
        let maybe_user: Option<User> = user_
            .filter(username.eq(username_))
            .select(User::as_select())
            .first(connection).await
            .optional()?;
        // Success
        Ok(maybe_user)
    }

}