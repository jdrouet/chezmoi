create table metrics (
    timestamp datetime not null,
    name text not null,
    tags text not null,
    value_kind text not null,
    value_count integer
);
