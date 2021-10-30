select plan(2);

select fk_ok(
    'timer', 'oneshot_message_schedule', 'user_id',
    'timer', 'user', 'user_id',
    'check timer.oneshot_message_schedule -> timer.user fk'
);

select fk_ok(
    'timer', 'oneshot_message_progress', 'message_id',
    'timer', 'oneshot_message_schedule', 'message_id',
    'check timer.oneshot_message_progress -> timer.oneshot_message_schedule fk'
);

select * from finish();
