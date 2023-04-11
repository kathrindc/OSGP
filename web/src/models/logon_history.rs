use crate::{database::establish_connection, schema::logon_history};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use super::LogonSession;

#[derive(Debug, Serialize, Insertable, Queryable)]
#[diesel(table_name = logon_history)]
pub struct LogonHistory {
    pub id: Uuid,
    pub user: i32,
    pub address: String,
    pub started_at: DateTime<Utc>,
}

impl LogonHistory {
    fn new(session: &LogonSession) -> Self {
        Self {
            id: session.id,
            user: session.user,
            address: session.address.clone(),
            started_at: session.started_at,
        }
    }

    pub fn store(session: &LogonSession) {
        let connection = &mut establish_connection();
        let history = LogonHistory::new(session);

        diesel::insert_into(logon_history::table)
            .values(&history)
            .execute(connection)
            .expect("Database error while storing logon history");
    }

    pub fn load_by_user(user: i32) -> Vec<LogonHistory> {
        let connection = &mut establish_connection();

        logon_history::table
            .filter(logon_history::dsl::user.eq(user))
            .get_results::<LogonHistory>(connection)
            .expect("Database error while loading logon history")
    }
}
