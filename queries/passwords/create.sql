create table passwords (
    id text primary key,
    user_id text not null,
    domain text not null,
    password text not null,
    date_created text default current_timestamp not null,
    date_modified text,
    foreign key(user_id) references users(id)
);

create trigger passwords_after_update
    after update on passwords
begin
    update passwords set date_modified = current_timestamp where id = new.id;
end
