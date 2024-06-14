-- Add down migration script here
create table seq (
    sender_id  bigint not null,
    chat_type int not null,
    receiver_id bigint not null,
    primary key (sender_id, chat_type, receiver_id)
);
