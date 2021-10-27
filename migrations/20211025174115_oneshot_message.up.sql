create extension if not exists "uuid-ossp";

create schema if not exists timer;

create table if not exists timer.oneshot_message_schedule(
    message_id uuid primary key,
    content text not null,
    scheduled_at timestamp with time zone not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now()
);

create table if not exists timer.oneshot_message_progress(
    message_id uuid primary key references timer.oneshot_message_schedule,
    queued_at timestamp with time zone not null,
    sent_at timestamp with time zone
);
