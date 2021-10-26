do $$ 
declare
  message_id uuid := uuid_generate_v4();
begin 
  raise notice '%', message_id;
end $$
