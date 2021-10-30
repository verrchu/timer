select plan(14);

select columns_are(
    'timer',
    'oneshot_message_progress',
    array['message_id', 'queued_at', 'processed_at'],
    'check timer.oneshot_message_progress table columns'
);

select indexes_are(
    'timer',
    'oneshot_message_progress',
    array['oneshot_message_progress_message_id_key'],
    'check timer.oneshot_message_progress table indexes'
);

select triggers_are(
    'timer',
    'oneshot_message_progress',
    array[]::text[],
    'check timer.oneshot_message_progress table triggers'
);

select hasnt_pk(
    'timer', 'oneshot_message_progress',
    'check timer.oneshot_message_progress has no pk'
);

select col_is_fk(
    'timer', 'oneshot_message_progress', 'message_id',
    'check timer.oneshot_message_progress -> timer.oneshot_message_shcedule fk'
);

select col_not_null(
    'timer', 'oneshot_message_progress', 'message_id',
    'check timer.oneshot_message_progress message_id is not null'
);
select col_not_null(
    'timer', 'oneshot_message_progress', 'queued_at',
    'check timer.oneshot_message_progress queued_at is not null'
);
select col_is_null(
    'timer', 'oneshot_message_progress', 'processed_at',
    'check timer.oneshot_message_progress processed_at is nullable'
);

select col_type_is(
    'timer', 'oneshot_message_progress', 'message_id', 'uuid',
    'check timer.oneshot_message_progress message_id type'
);
select col_type_is(
    'timer', 'oneshot_message_progress', 'queued_at', 'timestamp with time zone',
    'check timer.oneshot_message_progress queued_at type'
);
select col_type_is(
    'timer', 'oneshot_message_progress', 'processed_at', 'timestamp with time zone',
    'check timer.oneshot_message_progress processed_at type'
);

select col_hasnt_default(
    'timer', 'oneshot_message_progress', 'message_id',
    'check timer.oneshot_message_progress message_id has no default'
);
select col_hasnt_default(
    'timer', 'oneshot_message_progress', 'queued_at',
    'check timer.oneshot_message_progress queued_at has no default'
);
select col_hasnt_default(
    'timer', 'oneshot_message_progress', 'processed_at',
    'check timer.oneshot_message_progress processed_at has no default'
);

select * from finish();
