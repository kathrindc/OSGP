use crate::{database::establish_connection, schema::logon_sessions};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Identifiable, Insertable, Queryable, Serialize, AsChangeset)]
pub struct LogonSession {
    pub id: Uuid,
    pub user: i32,
    pub expires_at: DateTime<Utc>,
}

impl LogonSession {
    fn new(user: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            user,
            expires_at: Utc::now() + Duration::minutes(20),
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

    pub fn verify(&self) -> bool {
        Utc::now() < self.expires_at
    }

    pub fn refresh(&mut self) {
        let connection = &mut establish_connection();

        self.expires_at = Utc::now() + Duration::minutes(20);
        self.save_changes::<LogonSession>(connection)
            .expect("Database error while refreshing session");
    }
}