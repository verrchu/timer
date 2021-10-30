select plan(20);

select columns_are(
    'timer',
    'oneshot_message_schedule',
    array['message_id', 'user_id', 'content', 'scheduled_at', 'created_at', 'updated_at'],
    'check timer.oneshot_message_schedule table columns'
);

select indexes_are(
    'timer',
    'oneshot_message_schedule',
    array['oneshot_message_schedule_pkey'],
    'check timer.oneshot_message_schedule table indexes'
);

select triggers_are(
    'timer',
    'oneshot_message_schedule',
    array['oneshot_message_schedule_moddatetime'],
    'check timer.oneshot_message_schedule table triggers'
);

select col_is_pk(
    'timer', 'oneshot_message_schedule', 'message_id',
    'check timer.oneshot_message_schedule pk'
);

select col_is_fk(
    'timer', 'oneshot_message_schedule', 'user_id',
    'check timer.oneshot_message_schedule -> timer.user fk'
);

select col_not_null(
    'timer', 'oneshot_message_schedule', 'message_id',
    'check timer.oneshot_message_schedule message_id is not null'
);
select col_not_null(
    'timer', 'oneshot_message_schedule', 'user_id',
    'check timer.oneshot_message_schedule user_id is not null'
);
select col_not_null(
    'timer', 'oneshot_message_schedule', 'content',
    'check timer.oneshot_message_schedule content is nullable'
);
select col_not_null(
    'timer', 'oneshot_message_schedule', 'created_at',
    'check timer.oneshot_message_schedule created_at is nullable'
);
select col_not_null(
    'timer', 'oneshot_message_schedule', 'updated_at',
    'check timer.oneshot_message_schedule updated_at is nullable'
);

select col_type_is(
    'timer', 'oneshot_message_schedule', 'message_id', 'uuid',
    'check timer.oneshot_message_schedule message_id type'
);
select col_type_is(
    'timer', 'oneshot_message_schedule', 'user_id', 'uuid',
    'check timer.oneshot_message_schedule user_id type'
);
select col_type_is(
    'timer', 'oneshot_message_schedule', 'scheduled_at', 'timestamp with time zone',
    'check timer.oneshot_message_schedule scheduled_at type'
);
select col_type_is(
    'timer', 'oneshot_message_schedule', 'created_at', 'timestamp with time zone',
    'check timer.oneshot_message_schedule created_at type'
);
select col_type_is(
    'timer', 'oneshot_message_schedule', 'updated_at', 'timestamp with time zone',
    'check timer.oneshot_message_schedule updated_at type'
);

select col_default_is(
    'timer', 'oneshot_message_schedule', 'message_id', 'uuid_generate_v4()',
    'check timer.oneshot_message_schedule message_id default'
);
select col_hasnt_default(
    'timer', 'oneshot_message_schedule', 'user_id',
    'check timer.oneshot_message_schedule user_id has no default'
);
select col_hasnt_default(
    'timer', 'oneshot_message_schedule', 'scheduled_at',
    'check timer.oneshot_message_schedule scheduled_at has no default'
);
select col_default_is(
    'timer', 'oneshot_message_schedule', 'created_at', 'now()',
    'check timer.oneshot_message_schedule created_at default'
);
select col_default_is(
    'timer', 'oneshot_message_schedule', 'updated_at', 'now()',
    'check timer.oneshot_message_schedule updated_at has no default'
);

select * from finish();
