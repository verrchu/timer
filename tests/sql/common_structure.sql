select plan(4);

select tables_are(
    'timer',
    array[
        'oneshot_message_schedule',
        'oneshot_message_progress',
        'user'
    ],
    'check defined tables'
);

select foreign_tables_are('timer', array[]::text[], 'check defined foreign tables');

select views_are('timer', array[]::text[], 'check defined views');

select materialized_views_are('timer', array[]::text[], 'check defined materialized views');

select * from finish();
