begin;
  select uuid_generate_v4() into id;
  select now() into start;

  select plan(13);

  insert into timer.oneshot_message(
    message_id,
    data,
    scheduled_at
  ) values (
    (select * from id), 'test', now() + interval '1 day'
  );

  select is(count(*)::integer, 1::integer, 'message inserted')
  from timer.oneshot_message
  where message_id::text = (select * from id)::text;

  select isnt(created_at, NULL, 'created_at autpmatically populated')
  from timer.oneshot_message
  where message_id::text = (select * from id)::text;

  select cmp_ok((select * from start), '<=', created_at, 'created_at >= test start time')
  from timer.oneshot_message
  where message_id::text = (select * from id)::text;

  select cmp_ok(now(), '>=', created_at, 'created_at <= test execution time')
  from timer.oneshot_message
  where message_id::text = (select * from id)::text;

  select isnt(updated_at, NULL, 'updated_at automatically populated')
  from timer.oneshot_message
  where message_id::text = (select * from id)::text;

  select cmp_ok((select * from start), '<=', updated_at, 'created_at >= test start time')
  from timer.oneshot_message
  where message_id::text = (select * from id)::text;

  select cmp_ok(now(), '>=', updated_at, 'created_at <= test execution time')
  from timer.oneshot_message
  where message_id::text = (select * from id)::text;

  prepare insert_duplicate_message as insert into timer.oneshot_message(
    message_id,
    data,
    scheduled_at
  ) values (
    (select * from id), 'test', now() + interval '1 day'
  );

  SELECT throws_ok(
    'insert_duplicate_message',
    '23505',
    'duplicate key value violates unique constraint "oneshot_message_pkey"',
    'insert message with duplicate message_id'
  );

  prepare insert_without_message_id as insert into timer.oneshot_message(
    data,
    scheduled_at
  ) values (
    'test', now() + interval '1 day'
  );

  SELECT throws_ok(
    'insert_without_message_id',
    '23502',
    'null value in column "message_id" of relation "oneshot_message" violates not-null constraint',
    'insert message without message_id'
  );

  prepare insert_without_data as insert into timer.oneshot_message(
    message_id,
    scheduled_at
  ) values (
    (select * from id), now() + interval '1 day'
  );

  SELECT throws_ok(
    'insert_without_data',
    '23502',
    'null value in column "data" of relation "oneshot_message" violates not-null constraint',
    'insert message without data'
  );

  prepare insert_without_scheduled_at as insert into timer.oneshot_message(
    message_id,
    data
  ) values (
    (select * from id), 'test'
  );

  SELECT throws_ok(
    'insert_without_scheduled_at',
    '23502',
    'null value in column "scheduled_at" of relation "oneshot_message" violates not-null constraint',
    'insert message without scheduled_at'
  );

  prepare insert_scheduled_at_past as insert into timer.oneshot_message(
    message_id,
    data,
    scheduled_at
  ) values (
    uuid_generate_v4(), 'test', now() - interval '1 day'
  );

  SELECT throws_ok(
    'insert_scheduled_at_past',
    '23514',
    'new row for relation "oneshot_message" violates check constraint "oneshot_message_schedulaed_at_future_check"',
    'insert message with scheduled_at in the past'
  );

  prepare insert_empty_data as insert into timer.oneshot_message(
    message_id,
    data,
    scheduled_at
  ) values (
    uuid_generate_v4(), '', now() + interval '1 day'
  );

  SELECT throws_ok(
    'insert_empty_data',
    '23514',
    'new row for relation "oneshot_message" violates check constraint "oneshot_message_data_nonempty_check"',
    'insert message with scheduled_at in the past'
  );

  select * from finish();
rollback;
