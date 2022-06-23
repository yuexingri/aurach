table! {
    aurach_user (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Varchar>,
    }
}

table! {
    aurach_task_info (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        status -> Integer,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(aurach_user, aurach_task_info, );
