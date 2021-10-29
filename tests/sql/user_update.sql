-- this test unfortunately cannot be run in scope of a single transation
-- becaus for some reason trigger for updated_at has no effect in that case
-- therefore some effects are commited to db and the manually cleaned up
insert into timer.user(alias) values (('alias1'));

select updated_at into first_updated_at
from timer.user where alias = 'alias1';
  
begin;
  select plan(1);

  update timer.user set alias = 'alias2'
  where user_id = (select user_id from timer.user where alias = 'alias1');

  select updated_at into second_updated_at
  from timer.user where alias = 'alias2';

  select cmp_ok(
    (select * from first_updated_at), '<', (select * from second_updated_at),
    'updated_at changes automatically on row update'
  );

  select * from finish();
rollback;

-- cleanup
delete from timer.user where alias = 'alias1';

drop table first_updated_at;
