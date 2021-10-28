begin;
  select uuid_generate_v4() into tmp_user_id;
  select now() into start;

  select plan(8);

  insert into timer.user(user_id) values ((select * from tmp_user_id));

  select is(count(*)::integer, 1::integer, 'message inserted')
  from timer.user where user_id = (select * from tmp_user_id);

  select isnt(created_at, NULL, 'created_at autpmatically populated')
  from timer.user where user_id = (select * from tmp_user_id);

  select cmp_ok((select * from start), '<=', created_at, 'created_at >= test start time')
  from timer.user where user_id = (select * from tmp_user_id);

  select cmp_ok(now(), '>=', created_at, 'created_at <= test execution time')
  from timer.user where user_id = (select * from tmp_user_id);

  select isnt(updated_at, NULL, 'updated_at automatically populated')
  from timer.user where user_id = (select * from tmp_user_id);

  select cmp_ok((select * from start), '<=', updated_at, 'created_at >= test start time')
  from timer.user where user_id = (select * from tmp_user_id);

  select cmp_ok(now(), '>=', updated_at, 'created_at <= test execution time')
  from timer.user where user_id = (select * from tmp_user_id);

  prepare insert_duplicate_user as insert into timer.user(user_id)
  values ((select * from tmp_user_id));

  SELECT throws_ok(
    'insert_duplicate_user',
    '23505',
    'duplicate key value violates unique constraint "user_pkey"',
    'insert message with duplicate message_id'
  );

  select * from finish();
rollback;
