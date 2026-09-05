-- Add migration script here

create table test (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT
);

SELECT version()
