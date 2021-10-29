begin;
  select now() into start;

  select plan(9);

  insert into timer.user(alias) values (('alias'));

  select is(count(*)::integer, 1::integer, 'user inserted')
  from timer.user where alias = 'alias';

  select isnt(user_id, NULL, 'user_id autpmatically populated')
  from timer.user where alias = 'alias';

  select isnt(created_at, NULL, 'created_at autpmatically populated')
  from timer.user where alias = 'alias';

  select cmp_ok((select * from start), '<=', created_at, 'created_at >= test start time')
  from timer.user where alias = 'alias';

  select cmp_ok(now(), '>=', created_at, 'created_at <= test execution time')
  from timer.user where alias = 'alias';

  select isnt(updated_at, NULL, 'updated_at automatically populated')
  from timer.user where alias = 'alias';

  select cmp_ok((select * from start), '<=', updated_at, 'created_at >= test start time')
  from timer.user where alias = 'alias';

  select cmp_ok(now(), '>=', updated_at, 'created_at <= test execution time')
  from timer.user where alias = 'alias';

  prepare insert_duplicate_user as insert into timer.user(alias) values (('alias'));

  SELECT throws_ok(
    'insert_duplicate_user',
    '23505',
    'duplicate key value violates unique constraint "user_alias_key"',
    'insert user with duplicate alias'
  );

  select * from finish();
rollback;
