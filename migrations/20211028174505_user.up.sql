create table if not exists timer.user(
    user_id uuid primary key,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now()
);

create trigger user_moddatetime
    before update on timer.user
    for each row
    execute procedure moddatetime (updated_at);

alter table timer.oneshot_message_schedule
add column user_id uuid not null;

alter table timer.oneshot_message_schedule
add constraint oneshot_message_schedule_user_id
foreign key (user_id) references timer.user (user_id);
