select rowid, machine_user_id, domain, username, password, date_created, date_modified
from passwords
where machine_user_id = ?
