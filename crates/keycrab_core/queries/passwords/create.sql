/* For this table the identification will be done using the automatic row id */
create table if not exists passwords (
    machine_user_id text not null,
    domain text not null,
    username text not null,
    password text not null,
    date_created text default current_timestamp not null,
    date_modified text,
    foreign key(machine_user_id) references machine_users(id)
);

create trigger if not exists passwords_after_update
    after update on passwords
begin
    update passwords set date_modified = current_timestamp where rowid = new.rowid;
end
