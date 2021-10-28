-- this test unfortunately cannot be run in scope of a single transation
-- becaus for some reason trigger for updated_at has no effect in that case
-- therefore some effects are commited to db and the manually cleaned up
select uuid_generate_v4() into tmp_message_id;
select uuid_generate_v4() into tmp_user_id;

insert into timer.user(user_id)
values ((select * from tmp_user_id));

insert into timer.oneshot_message_schedule(
  message_id,
  user_id,
  content,
  scheduled_at
) values (
  (select * from tmp_message_id),
  (select * from tmp_user_id),
  'test', now() + interval '1 day'
);

select updated_at into first_updated_at
from timer.oneshot_message_schedule
where message_id = (select * from tmp_message_id);
  
begin;
  select plan(1);

  update timer.oneshot_message_schedule
  set content = 'another_test'
  where message_id = (select * from tmp_message_id);

  select updated_at into second_updated_at
  from timer.oneshot_message_schedule
  where message_id = (select * from tmp_message_id);

  select cmp_ok(
    (select * from first_updated_at), '<', (select * from second_updated_at),
    'updated_at changes automatically on row update'
  );

  select * from finish();
rollback;

-- cleabup
delete from timer.oneshot_message_schedule
where message_id = (select * from tmp_message_id);

delete from timer.user
where user_id = (select * from tmp_user_id);

drop table tmp_message_id;
drop table tmp_user_id;
drop table first_updated_at;
