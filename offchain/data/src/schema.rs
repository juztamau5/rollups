// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "OutputEnum"))]
    pub struct OutputEnum;
}

diesel::table! {
    inputs (index) {
        index -> Int4,
        msg_sender -> Bytea,
        tx_hash -> Bytea,
        block_number -> Int8,
        timestamp -> Timestamp,
        payload -> Bytea,
    }
}

diesel::table! {
    notices (input_index, index) {
        input_index -> Int4,
        index -> Int4,
        payload -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OutputEnum;

    proofs (input_index, output_index, output_enum) {
        input_index -> Int4,
        output_index -> Int4,
        output_enum -> OutputEnum,
        validity_input_index -> Int4,
        validity_output_index -> Int4,
        validity_output_hashes_root_hash -> Bytea,
        validity_vouchers_epoch_root_hash -> Bytea,
        validity_notices_epoch_root_hash -> Bytea,
        validity_machine_state_hash -> Bytea,
        validity_keccak_in_hashes_siblings -> Array<Nullable<Bytea>>,
        validity_output_hashes_in_epoch_siblings -> Array<Nullable<Bytea>>,
        context -> Bytea,
    }
}

diesel::table! {
    reports (input_index, index) {
        input_index -> Int4,
        index -> Int4,
        payload -> Bytea,
    }
}

diesel::table! {
    vouchers (input_index, index) {
        input_index -> Int4,
        index -> Int4,
        destination -> Bytea,
        payload -> Bytea,
    }
}

diesel::joinable!(notices -> inputs (input_index));
diesel::joinable!(reports -> inputs (input_index));
diesel::joinable!(vouchers -> inputs (input_index));

diesel::allow_tables_to_appear_in_same_query!(
    inputs, notices, proofs, reports, vouchers,
);
