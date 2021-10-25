create extension if not exists "uuid-ossp";

create schema if not exists timer;

create table if not exists timer.oneshot_message(
    message_id uuid primary key,
    data text not null,
    planned_at timestamp with time zone not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now()
);

create table if not exists timer.oneshot_message_progress(
    message_id uuid primary key references timer.oneshot_message,
    queued_for_send_at timestamp with time zone,
    send_at timestamp with time zone
);