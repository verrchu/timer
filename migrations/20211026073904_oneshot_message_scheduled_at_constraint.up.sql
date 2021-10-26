alter table oneshot_message
add constraint oneshot_message_schedulaed_at_check
check (
    scheduled_at > now() + interval '1 minute'
);
