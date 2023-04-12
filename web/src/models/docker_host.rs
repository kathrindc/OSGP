use std::str::FromStr;

use crate::{database::establish_connection, schema::docker_hosts};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use shiplift::{Docker, Uri};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Identifiable, AsChangeset)]
pub struct DockerHost {
    pub id: Uuid,
    pub name: String,
    pub connection_type: String,
    pub connection_address: String,
}

pub enum PingResult {
    Online(String),
    Offline(String),
}

impl DockerHost {
    pub fn new(name: String, connection_type: String, connection_address: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            connection_type,
            connection_address,
        }
    }

    pub fn create(name: String, connection_type: String, connection_address: String) -> DockerHost {
        let connection = &mut establish_connection();
        let host = DockerHost::new(name, connection_type, connection_address);

        diesel::insert_into(docker_hosts::table)
            .values(&host)
            .get_result::<DockerHost>(connection)
            .expect("Database error while creating docker host")
    }

    pub fn update(&self) {
        let connection = &mut establish_connection();

        self.save_changes::<DockerHost>(connection)
            .expect("Database error while updating docker host");
    }

    pub fn delete(&self) {
        let connection = &mut establish_connection();

        diesel::delete(self)
            .execute(connection)
            .expect("Database error while deleting docker host");
    }

    pub async fn retrieve_status(&self) -> PingResult {
        let remote = self
            .get_remote()
            .expect("Invalid remote configuration for docker host");

        match remote.ping().await {
            Ok(result) => PingResult::Online(result),
            Err(error) => PingResult::Offline(format!("{}", error)),
        }
    }

    pub fn get_remote(&self) -> Option<Docker> {
        if self.connection_type == "unix" {
            return Some(Docker::unix(&self.connection_address));
        }

        if self.connection_type == "ip" {
            let address = self.connection_address.clone();
            let uri = Uri::from_str(&address).expect("Invalid docker URI");

            return Some(Docker::host(uri));
        }

        None
    }
}
