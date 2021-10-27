alter table oneshot_message
add constraint oneshot_message_schedulaed_at_future_check
check (
    scheduled_at > now() + interval '1 minute'
);

alter table oneshot_message
add constraint oneshot_message_data_nonempty_check
check (
    char_length(data) > 1
);