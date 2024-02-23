// @generated automatically by Diesel CLI.

pub mod eventus {
    diesel::table! {
        eventus.authority (id) {
            id -> Int4,
            #[max_length = 255]
            name_ -> Varchar,
            user_id -> Int4,
        }
    }

    diesel::table! {
        eventus.event_ (id) {
            id -> Int4,
            #[max_length = 255]
            name_ -> Varchar,
            #[max_length = 2048]
            description_ -> Varchar,
            creator -> Int4,
            creation_date -> Timestamptz,
            start_ -> Timestamptz,
            end_ -> Timestamptz,
            seats -> Int4,
            approved -> Bool,
        }
    }

    diesel::table! {
        eventus.subscription (id) {
            id -> Int4,
            user_id -> Int4,
            event_id -> Int4,
            creation_date -> Timestamptz,
            rating -> Nullable<Int4>,
            #[max_length = 2048]
            comment -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        eventus.user_ (id) {
            id -> Int4,
            #[max_length = 255]
            username -> Varchar,
            #[max_length = 255]
            password_ -> Varchar,
            active -> Bool,
        }
    }

    diesel::joinable!(authority -> user_ (user_id));
    diesel::joinable!(event_ -> user_ (creator));
    diesel::joinable!(subscription -> event_ (event_id));
    diesel::joinable!(subscription -> user_ (user_id));

    diesel::allow_tables_to_appear_in_same_query!(
        authority,
        event_,
        subscription,
        user_,
    );
}
