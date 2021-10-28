-- this test unfortunately cannot be run in scope of a single transation
-- becaus for some reason trigger for updated_at has no effect in that case
-- therefore some effects are commited to db and the manually cleaned up
select uuid_generate_v4() into first_user_id;

insert into timer.user(user_id) values ((select * from first_user_id));

select updated_at into first_updated_at
from timer.user where user_id = (select * from first_user_id);
  
begin;
  select plan(1);

  select uuid_generate_v4() into second_user_id;

  update timer.user set user_id = (select * from second_user_id)
  where user_id = (select * from first_user_id);

  select updated_at into second_updated_at
  from timer.user where user_id = (select * from second_user_id);

  select cmp_ok(
    (select * from first_updated_at), '<', (select * from second_updated_at),
    'updated_at changes automatically on row update'
  );

  select * from finish();
rollback;

-- cleanup
delete from timer.user where user_id = (select * from first_user_id);

drop table first_user_id;
drop table first_updated_at;
