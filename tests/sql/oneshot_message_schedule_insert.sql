begin;
  select now() into start;

  insert into timer.user(alias)
  values (('alias'));

  select user_id into tmp_user_id
  from timer.user where alias = 'alias';

  select plan(14);

  -- TODO: would be nice to save id of created message
  -- directly form returning clause
  insert into timer.oneshot_message_schedule(
    user_id,
    content,
    scheduled_at
  ) values (
    (select * from tmp_user_id),
    'test',
    now() + interval '1 day'
  );

  select message_id into tmp_message_id
  from timer.oneshot_message_schedule
  where user_id = (select * from tmp_user_id);

  select is(count(*)::integer, 1::integer, 'message inserted')
  from timer.oneshot_message_schedule
  where message_id = (select * from tmp_message_id);

  select isnt(message_id, NULL, 'message_id automatically populated')
  from timer.oneshot_message_schedule
  where message_id = (select * from tmp_message_id);

  select isnt(created_at, NULL, 'created_at automatically populated')
  from timer.oneshot_message_schedule
  where message_id = (select * from tmp_message_id);

  select cmp_ok((select * from start), '<=', created_at, 'created_at >= test start time')
  from timer.oneshot_message_schedule
  where message_id = (select * from tmp_message_id);

  select cmp_ok(now(), '>=', created_at, 'created_at <= test execution time')
  from timer.oneshot_message_schedule
  where message_id = (select * from tmp_message_id);

  select isnt(updated_at, NULL, 'updated_at automatically populated')
  from timer.oneshot_message_schedule
  where message_id = (select * from tmp_message_id);

  select cmp_ok((select * from start), '<=', updated_at, 'created_at >= test start time')
  from timer.oneshot_message_schedule
  where message_id = (select * from tmp_message_id);

  select cmp_ok(now(), '>=', updated_at, 'created_at <= test execution time')
  from timer.oneshot_message_schedule
  where message_id = (select * from tmp_message_id);

  prepare insert_without_user_id as insert into timer.oneshot_message_schedule(
    message_id,
    content,
    scheduled_at
  ) values (
    uuid_generate_v4(), 'test', now() + interval '1 day'
  );

  SELECT throws_ok(
    'insert_without_user_id',
    '23502',
    'null value in column "user_id" of relation '
    '"oneshot_message_schedule" violates not-null constraint',
    'insert message without message_id'
  );

  prepare insert_without_content as insert into timer.oneshot_message_schedule(
    user_id,
    scheduled_at
  ) values (
    (select * from tmp_user_id),
    now() + interval '1 day'
  );

  SELECT throws_ok(
    'insert_without_content',
    '23502',
    'null value in column "content" of relation '
    '"oneshot_message_schedule" violates not-null constraint',
    'insert message without content'
  );

  prepare insert_without_scheduled_at as insert into timer.oneshot_message_schedule(
    user_id,
    content
  ) values (
    (select * from tmp_user_id),
    'test'
  );

  SELECT throws_ok(
    'insert_without_scheduled_at',
    '23502',
    'null value in column "scheduled_at" of relation '
    '"oneshot_message_schedule" violates not-null constraint',
    'insert message without scheduled_at'
  );

  prepare insert_scheduled_at_past as insert into timer.oneshot_message_schedule(
    message_id,
    user_id,
    content,
    scheduled_at
  ) values (
    uuid_generate_v4(),
    (select * from tmp_user_id),
    'test', now() - interval '1 day'
  );

  SELECT throws_ok(
    'insert_scheduled_at_past',
    '23514',
    'new row for relation "oneshot_message_schedule" violates '
    'check constraint "oneshot_message_scheduled_at_future_check"',
    'insert message with scheduled_at in the past'
  );

  prepare insert_empty_content as insert into timer.oneshot_message_schedule(
    message_id,
    user_id,
    content,
    scheduled_at
  ) values (
    uuid_generate_v4(),
    (select * from tmp_user_id),
    '', now() + interval '1 day'
  );

  SELECT throws_ok(
    'insert_empty_content',
    '23514',
    'new row for relation "oneshot_message_schedule" violates '
    'check constraint "oneshot_message_nonempty_content_check"',
    'insert message with empty content'
  );

  prepare insert_nonexistent_user_id as insert into timer.oneshot_message_schedule(
    message_id,
    user_id,
    content,
    scheduled_at
  ) values (
    uuid_generate_v4(),
    uuid_generate_v4(),
    'test', now() + interval '1 day'
  );

  SELECT throws_ok(
    'insert_nonexistent_user_id',
    '23503',
    'insert or update on table "oneshot_message_schedule" violates '
    'foreign key constraint "oneshot_message_schedule_user_id"',
    'insert message with nonexistent user_id'
  );

  select * from finish();
rollback;
