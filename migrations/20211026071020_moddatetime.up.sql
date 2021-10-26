create extension if not exists moddatetime;

create trigger oneshot_message_moddatetime
    before update on oneshot_message
    for each row
    execute procedure moddatetime (updated_at);
