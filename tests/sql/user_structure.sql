select plan(16);

select columns_are(
    'timer',
    'user',
    array['user_id', 'alias', 'created_at', 'updated_at'],
    'check timer.user table columns'
);

select indexes_are(
    'timer',
    'user',
    array['user_pkey', 'user_alias_key'],
    'check timer.user table indexes'
);

select triggers_are(
    'timer',
    'user',
    array['user_moddatetime'],
    'check timer.user table triggers'
);

select col_is_pk(
    'timer',
    'user',
    'user_id',
    'check timer.user table pk'
);

select col_not_null('timer', 'user', 'user_id', 'check timer.user user_id is not null');
select col_not_null('timer', 'user', 'alias', 'check timer.user alias is not null');
select col_not_null('timer', 'user', 'created_at', 'check timer.user created_at is not null');
select col_not_null('timer', 'user', 'updated_at', 'check timer.user updated_at is not null');

select col_type_is('timer', 'user', 'user_id', 'uuid', 'check timer.user user_id type');
select col_type_is('timer', 'user', 'alias', 'text', 'check timer.user alias type');
select col_type_is('timer', 'user', 'created_at', 'timestamp with time zone', 'check timer.user created_at type');
select col_type_is('timer', 'user', 'updated_at', 'timestamp with time zone', 'check timer.user updated_at type');

select col_default_is('timer', 'user', 'user_id', 'uuid_generate_v4()', 'check timer.user user_id default');
select col_hasnt_default('timer', 'user', 'alias', 'check timer.user alias has not default');
select col_default_is('timer', 'user', 'created_at', 'now()', 'check timer.user created_at type');
select col_default_is('timer', 'user', 'updated_at', 'now()', 'check timer.user updated_at type');

select * from finish();
