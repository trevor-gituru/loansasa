// @generated automatically by Diesel CLI.

diesel::table! {
    user_details (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        account_address -> Varchar,
        #[max_length = 255]
        private_key -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    wallets (id) {
        id -> Int4,
        #[max_length = 255]
        starknet_address -> Varchar,
        #[max_length = 255]
        public_key -> Varchar,
        #[max_length = 255]
        private_key -> Varchar,
        user_id -> Nullable<Int4>,
    }
}

diesel::joinable!(user_details -> users (user_id));
diesel::joinable!(wallets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_details,
    users,
    wallets,
);
