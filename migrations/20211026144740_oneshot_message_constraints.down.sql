alter table oneshot_message_schedule
drop constraint oneshot_message_scheduled_at_future_check;

alter table oneshot_message_schedule
drop constraint oneshot_message_data_nonempty_check;
