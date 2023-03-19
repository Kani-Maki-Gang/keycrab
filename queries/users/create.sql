create table users (
    id text primary key,
    name text not null,
    date_created text default current_timestamp not null,
    date_modified text
);

create trigger users_after_update
    after update on users
begin
    update users set date_modified = current_timestamp where id = new.id;
end
