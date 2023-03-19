create table if not exists machine_users (
    id text primary key,
    name text not null,
    date_created text default current_timestamp not null,
    date_modified text
);

create trigger if not exists machine_users_after_update
    after update on machine_users
begin
    update users set date_modified = current_timestamp where id = new.id;
end
