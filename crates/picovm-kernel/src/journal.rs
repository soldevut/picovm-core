use rusqlite::{Connection, Result};

pub struct MemoryJournal {
    conn: Connection,
}

impl MemoryJournal {
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS agent_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                action_type TEXT NOT NULL,
                payload_json TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn record_event(&self, action_type: &str, payload_json: &str) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
            
        self.conn.execute(
            "INSERT INTO agent_events (timestamp, action_type, payload_json) VALUES (?1, ?2, ?3)",
            (now, action_type, payload_json),
        )?;
        Ok(())
    }
}
