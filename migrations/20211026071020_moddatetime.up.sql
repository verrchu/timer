create extension if not exists moddatetime;

create trigger oneshot_message_schedule_moddatetime
    before update on oneshot_message_schedule
    for each row
    execute procedure moddatetime (updated_at);
