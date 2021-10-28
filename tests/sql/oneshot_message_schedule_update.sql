create extension if not exists pgtap;

-- this test unfortunately cannot be run in scope of a single transation
-- becaus for some reason trigger for updated_at has no effect in that case
-- therefore some effects are commited to db and the manually cleaned up
select uuid_generate_v4() into id;

insert into timer.oneshot_message_schedule(
  message_id,
  content,
  scheduled_at
) values (
  (select * from id), 'test', now() + interval '1 day'
);

select updated_at into first_updated_at
from timer.oneshot_message_schedule
where message_id = (select * from id);
  
begin;
  select plan(1);

  update timer.oneshot_message_schedule
  set content = 'another_test'
  where message_id = (select * from id);

  select updated_at into second_updated_at
  from timer.oneshot_message_schedule
  where message_id = (select * from id);

  select cmp_ok(
    (select * from first_updated_at), '<', (select * from second_updated_at),
    'updated_at changes automatically on row update'
  );

  select * from finish();
rollback;

-- cleabup
delete from timer.oneshot_message_schedule
where message_id = (select * from id);

drop table if exists id;
drop table if exists first_updated_at;
