table! {
    bank (id) {
        id -> Int4,
        name -> Varchar,
        url -> Nullable<Varchar>,
        code -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    books (id) {
        id -> Int4,
        book_type -> Int2,
        bank_id -> Int4,
        book_id -> Varchar,
        ref_acc_id -> Nullable<Varchar>,
        book_name -> Varchar,
        balance -> Float8,
        available -> Float8,
        user_id -> Varchar,
        pwd -> Varchar,
        mobile_number -> Varchar,
        is_transfer_auto -> Int2,
        customer_id -> Int4,
        book_group_id -> Int4,
        is_default -> Bool,
        status -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    casino (id) {
        id -> Int4,
        name -> Varchar,
        detail -> Nullable<Text>,
        domain -> Varchar,
        secure_code -> Varchar,
        suggestion -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    casino_user (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        reference_id -> Nullable<Varchar>,
        login_session -> Nullable<Varchar>,
        customer_id -> Int4,
        casino_id -> Int4,
        left_id -> Int4,
        right_id -> Int4,
        lv -> Int2,
        is_get_bonus -> Bool,
        bonus_percent -> Nullable<Float8>,
        bonus_amount -> Nullable<Float8>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;

    client_apps (id) {
        id -> Uuid,
        app_name -> Varchar,
        package_name -> Varchar,
        detail -> Nullable<Text>,
        api_key -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    credit_info (customer_id) {
        customer_id -> Int4,
        balance -> Float8,
        available_balance -> Float8,
        before_balance -> Float8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    customer (id) {
        id -> Int4,
        phone_number -> Varchar,
        password -> Varchar,
        login_session -> Nullable<Varchar>,
        full_name -> Varchar,
        user_name -> Varchar,
        detail -> Nullable<Text>,
        status -> Int2,
        user_refer -> Varchar,
        sup_id -> Int4,
        is_no_bonus -> Bool,
        keywords -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    customer_book (id) {
        id -> Int4,
        bank_id -> Int4,
        book_id -> Varchar,
        book_name -> Varchar,
        book_id_deposit -> Varchar,
        status -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        update_by -> Int4,
        customer_id -> Int4,
        book_type -> Int2,
        book_group_id -> Int4,
        user_id -> Varchar,
        pwd -> Varchar,
        mobile_number -> Varchar,
        is_transfer_auto -> Int2,
        is_default -> Bool,
    }
}

table! {
    customer_config (customer_id) {
        customer_id -> Int4,
        check_credit -> Int2,
        win_limit -> Float8,
        is_online -> Bool,
        suspend -> Int2,
        is_single_bet -> Bool,
        is_vip -> Bool,
    }
}

table! {
    customer_promotion (id) {
        id -> Int4,
        customer_id -> Int4,
        promotion_id -> Nullable<Int4>,
        status -> Int2,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    line_info (customer_id) {
        customer_id -> Int4,
        line_id -> Nullable<Varchar>,
        uid -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    note (id) {
        id -> Int4,
        note_type -> Varchar,
        mobile -> Varchar,
        #[sql_name = "note"]
        note_ -> Text,
        status -> Int2,
        customer_id -> Int4,
        withdraw -> Float8,
        withdraw_bank_id -> Nullable<Int4>,
        withdraw_acc_id -> Nullable<Varchar>,
        withdraw_status -> Int2,
        withdraw_count -> Int2,
        channel -> Int2,
        channel_desc -> Nullable<Varchar>,
        send_manual_transfer -> Int2,
        record_by -> Int4,
        update_by -> Int4,
        notice -> Int2,
        slip_log_id -> Nullable<Varchar>,
        tg_start_time -> Nullable<Timestamp>,
        tg_end_time -> Nullable<Timestamp>,
        withdraw_start_time -> Nullable<Timestamp>,
        withdraw_end_time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    note_detail (id) {
        id -> Int4,
        acc_date -> Date,
        note_id -> Int4,
        casino_id -> Nullable<Int4>,
        agent_id -> Varchar,
        user_id -> Varchar,
        amount -> Float8,
        bonus -> Float8,
        before_balance_amount -> Float8,
        before_credit_amount -> Float8,
        before_transfer_amount -> Float8,
        after_balance_amount -> Float8,
        after_credit_amount -> Float8,
        after_transfer_amount -> Float8,
        process_start_time -> Nullable<Timestamp>,
        process_end_time -> Nullable<Timestamp>,
        process_status -> Int2,
        response -> Nullable<Text>,
        statement_id -> Int4,
        promotion_id -> Int4,
        record_by -> Int4,
        update_by -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    notifications (id) {
        id -> Int4,
        customer_id -> Int4,
        title -> Varchar,
        body -> Varchar,
        image -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        read_status -> Int2,
        send_status -> Int2,
        sent_response -> Nullable<Text>,
        token_device -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    promotion (id) {
        id -> Int4,
        name -> Varchar,
        bonus -> Nullable<Int4>,
        free_credit -> Nullable<Int4>,
    }
}

table! {
    request_otps (id) {
        id -> Int4,
        phone_number -> Varchar,
        otp_number -> Varchar,
        req_status -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    settings (id) {
        id -> Int4,
        name -> Varchar,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    slip_log (id) {
        id -> Varchar,
        customer_id -> Int4,
        bank_code -> Int4,
        reference_no -> Nullable<Varchar>,
        account_id -> Nullable<Varchar>,
        account_from -> Nullable<Varchar>,
        amount -> Float8,
        time -> Nullable<Varchar>,
        filename -> Varchar,
        translated -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    statements (id) {
        id -> Int4,
        acc_date -> Date,
        bank_id -> Int4,
        acc_id -> Varchar,
        customer_id -> Int4,
        amount -> Float8,
        balance -> Float8,
        status -> Int2,
        update_by -> Varchar,
        approved -> Varchar,
        from_channel -> Varchar,
        trans_id -> Int4,
        reference_no -> Varchar,
        lw_id -> Int4,
        note_id -> Int4,
        reason -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::users::UserExternalIDPMapping;

    users (id) {
        id -> Varchar,
        customer_id -> Int4,
        external_idp_id -> Varchar,
        external_idp -> UserExternalIDPMapping,
        email -> Nullable<Varchar>,
        display_name -> Nullable<Varchar>,
        sign_session -> Nullable<Text>,
        user_id -> Nullable<Varchar>,
        device_token -> Nullable<Text>,
        id_token -> Nullable<Text>,
        refresh_token -> Nullable<Text>,
        data -> Nullable<Text>,
        is_active -> Bool,
        is_internal -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    wallet (id) {
        id -> Int4,
        casino_user_id -> Int4,
        balance -> Float8,
        status -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    wallet_statements (id) {
        id -> Int4,
        wallet_id -> Int4,
        note_id -> Int4,
        source_id -> Int4,
        target_id -> Int4,
        amount -> Float8,
        balance -> Float8,
        target_type -> Int2,
        source_type -> Int2,
        status -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(credit_info -> customer (customer_id));
joinable!(customer_book -> bank (bank_id));
joinable!(customer_book -> customer (customer_id));
joinable!(customer_config -> customer (customer_id));
joinable!(customer_promotion -> promotion (promotion_id));
joinable!(line_info -> customer (customer_id));
joinable!(wallet -> casino_user (casino_user_id));
joinable!(wallet_statements -> wallet (wallet_id));

allow_tables_to_appear_in_same_query!(
    bank,
    books,
    casino,
    casino_user,
    client_apps,
    credit_info,
    customer,
    customer_book,
    customer_config,
    customer_promotion,
    line_info,
    note,
    note_detail,
    notifications,
    promotion,
    request_otps,
    settings,
    slip_log,
    statements,
    users,
    wallet,
    wallet_statements,
);
