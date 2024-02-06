// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Text,
        contact -> Text,
        cosigner -> Nullable<Text>,
        date_of_birth -> Nullable<Text>,
        license_number -> Text,
        license_expiration -> Nullable<Text>,
        date_added -> Nullable<Text>,
        date_modified -> Nullable<Text>,
        current_standing -> Nullable<Text>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    charge (id) {
        id -> Text,
        name -> Text,
        amount -> Text,
        date_effective -> Text,
    }
}

diesel::table! {
    creditor (id) {
        id -> Text,
        business_name -> Text,
        contact -> Text,
        filing_fees -> Text,
        date_added -> Nullable<Text>,
        date_modified -> Nullable<Text>,
        apr -> Text,
    }
}

diesel::table! {
    deal (id) {
        id -> Text,
        state -> Integer,
        date -> Text,
        account -> Text,
        inventory -> Text,
        creditor -> Nullable<Text>,
        cash -> Text,
        down -> Nullable<Text>,
        apr -> Text,
        finance -> Nullable<Text>,
        lien -> Nullable<Text>,
        pmt -> Nullable<Text>,
        term -> Text,
        tax_city -> Nullable<Text>,
        tax_state -> Nullable<Text>,
        tax_county -> Nullable<Text>,
        tax_rtd -> Nullable<Text>,
    }
}

diesel::table! {
    deal_charge (id) {
        deal -> Nullable<Text>,
        charge -> Nullable<Text>,
        date -> Nullable<Text>,
        note -> Nullable<Text>,
        id -> Text,
    }
}

diesel::table! {
    deal_salesman (id) {
        id -> Text,
        deal -> Text,
        salesman -> Text,
    }
}

diesel::table! {
    deal_trade (id) {
        id -> Text,
        deal -> Text,
        vin -> Text,
        value -> Text,
    }
}

diesel::table! {
    default_charge (id) {
        id -> Text,
        creditor -> Text,
        charge -> Text,
    }
}

diesel::table! {
    inventory (id) {
        id -> Text,
        vin -> Text,
        year -> Text,
        make -> Text,
        model -> Nullable<Text>,
        body -> Nullable<Text>,
        color -> Nullable<Text>,
        fuel -> Nullable<Text>,
        cwt -> Nullable<Text>,
        mileage -> Nullable<Text>,
        date_added -> Nullable<Text>,
        date_modified -> Nullable<Text>,
        picture -> Nullable<Text>,
        cash -> Nullable<Text>,
        credit -> Nullable<Text>,
        down -> Nullable<Text>,
        state -> Integer,
    }
}

diesel::table! {
    key (id) {
        id -> Text,
        hashed_password -> Nullable<Text>,
        user_id -> Text,
    }
}

diesel::table! {
    payment (id) {
        id -> Text,
        deal -> Text,
        date -> Text,
        amount -> Text,
    }
}

diesel::table! {
    person (id) {
        id -> Text,
        name_prefix -> Nullable<Text>,
        first_name -> Text,
        middle_initial -> Nullable<Text>,
        last_name -> Text,
        name_suffix -> Nullable<Text>,
        address_1 -> Text,
        address_2 -> Nullable<Text>,
        address_3 -> Nullable<Text>,
        city -> Text,
        state_province -> Text,
        zip_postal -> Text,
        zip_4 -> Nullable<Text>,
        country -> Text,
        phone_primary -> Text,
        phone_secondary -> Nullable<Text>,
        phone_tertiary -> Nullable<Text>,
        email_primary -> Nullable<Text>,
        email_secondary -> Nullable<Text>,
    }
}

diesel::table! {
    salesman (id) {
        id -> Text,
        person -> Text,
    }
}

diesel::table! {
    session (id) {
        id -> Text,
        user_id -> Text,
        active_expires -> BigInt,
        idle_expires -> BigInt,
        iv -> Nullable<Text>,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        username -> Text,
        email -> Text,
    }
}

diesel::joinable!(account -> person (contact));
diesel::joinable!(creditor -> person (contact));
diesel::joinable!(deal -> account (account));
diesel::joinable!(deal -> creditor (creditor));
diesel::joinable!(deal -> inventory (inventory));
diesel::joinable!(deal_charge -> charge (charge));
diesel::joinable!(deal_charge -> deal (deal));
diesel::joinable!(deal_salesman -> deal (deal));
diesel::joinable!(deal_trade -> deal (deal));
diesel::joinable!(default_charge -> charge (charge));
diesel::joinable!(key -> user (user_id));
diesel::joinable!(payment -> deal (deal));
diesel::joinable!(salesman -> person (person));
diesel::joinable!(session -> user (user_id));


diesel::allow_tables_to_appear_in_same_query!(
    account,
    charge,
    creditor,
    deal,
    deal_charge,
    deal_salesman,
    deal_trade,
    default_charge,
    inventory,
    key,
    payment,
    person,
    salesman,
    session,
    user,
);
