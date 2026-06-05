pub(super) use super::super::*;

use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::session::{SessionState, SessionStore};
use underlay_core::Uuid;

#[derive(Debug, Default)]
pub(super) struct MemoryStore {
    sessions: Mutex<HashMap<Uuid, SessionState>>,
}

#[async_trait]
impl SessionStore for MemoryStore {
    async fn get_session(&self, session_id: &Uuid) -> JwtResult<Option<SessionState>> {
        Ok(self.sessions.lock().await.get(session_id).cloned())
    }

    async fn create_session(&self, session: &SessionState) -> JwtResult<()> {
        self.sessions
            .lock()
            .await
            .insert(session.id, session.clone());
        Ok(())
    }

    async fn update_session(&self, session: &SessionState) -> JwtResult<()> {
        self.sessions
            .lock()
            .await
            .insert(session.id, session.clone());
        Ok(())
    }

    async fn rotate_session_if_current(
        &self,
        session: &SessionState,
        expected_refresh_token_fingerprint: &str,
        expected_refresh_token_id: Uuid,
        expected_refresh_token_version: u32,
    ) -> JwtResult<bool> {
        let mut sessions = self.sessions.lock().await;
        let Some(current) = sessions.get_mut(&session.id) else {
            return Ok(false);
        };

        if !current.is_active
            || current.refresh_token_fingerprint != expected_refresh_token_fingerprint
            || current.refresh_token_id != expected_refresh_token_id
            || current.refresh_token_version != expected_refresh_token_version
        {
            return Ok(false);
        }

        *current = session.clone();
        Ok(true)
    }

    async fn delete_session(&self, session_id: &Uuid) -> JwtResult<()> {
        self.sessions.lock().await.remove(session_id);
        Ok(())
    }

    async fn revoke_all_user_sessions(&self, user_id: &Uuid) -> JwtResult<u64> {
        let mut sessions = self.sessions.lock().await;
        let before = sessions.len();
        sessions.retain(|_, s| &s.user_id != user_id);
        Ok((before - sessions.len()) as u64)
    }

    async fn get_user_sessions(&self, user_id: &Uuid) -> JwtResult<Vec<SessionState>> {
        Ok(self
            .sessions
            .lock()
            .await
            .values()
            .filter(|s| &s.user_id == user_id)
            .cloned()
            .collect())
    }
}
