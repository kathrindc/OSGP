use crate::{database::establish_connection, schema::logon_sessions};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

static TWENTY_MINUTES: i64 = 60 * 20;

#[derive(Debug, Identifiable, Insertable, Queryable, Serialize)]
pub struct LogonSession {
    pub id: Uuid,
    pub user: i32,
}

impl LogonSession {
    fn new(user: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            user,
        }
    }

    pub fn load_by_id(id: Uuid) -> Option<LogonSession> {
        let connection = &mut establish_connection();

        logon_sessions::table
            .find(id)
            .first::<LogonSession>(connection)
            .optional()
            .expect("Database error while finding session")
    }

    pub fn begin(user: i32) -> LogonSession {
        let connection = &mut establish_connection();
        let session = LogonSession::new(user);

        diesel::insert_into(logon_sessions::table)
            .values(session)
            .get_result::<LogonSession>(connection)
            .expect("Database error while beginning session")
    }

    pub fn verify(self) -> bool {
        todo!();
    }

    pub fn refresh(self) {
        todo!();
    }
}
