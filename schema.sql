create table public.speaker
(
    id         integer primary key,
    name       varchar   not null,
    occupation varchar[] not null
);

insert into speaker
values (1, 'John Doe', '{"engineer"}'::text[]),
       (2, 'Alice Doe', '{"manager"}'::text[]),
       (3, 'Camille Doe', '{"manager", "engineer"}'::text[]);
