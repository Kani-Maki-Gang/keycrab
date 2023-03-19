select id, user_id, domain, password, date_created, date_modified
from passwords
where user_id = ?
