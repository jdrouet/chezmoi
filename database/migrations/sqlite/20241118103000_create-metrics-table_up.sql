create table metrics (
    timestamp datetime not null,
    name text not null,
    tags jsonb not null,
    value jsonb not null
);
