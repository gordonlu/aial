use super::*;

static DB_CONNS: OnceLock<Mutex<HashMap<i64, Arc<Mutex<rusqlite::Connection>>>>> = OnceLock::new();
fn db_conns() -> &'static Mutex<HashMap<i64, Arc<Mutex<rusqlite::Connection>>>> {
    DB_CONNS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_open_memory(path_ptr: i64) -> i64 {
    let path = lock!(strs()).get(&path_ptr).cloned().unwrap_or_else(|| ":memory:".to_string());
    let conn = match rusqlite::Connection::open(&path) { Ok(c) => c, Err(e) => { eprintln!("[aial-rt] db open error: {}", e); return -1; } };
    conn.execute("CREATE TABLE IF NOT EXISTS messages (id INTEGER PRIMARY KEY AUTOINCREMENT, session TEXT, role TEXT, content TEXT, ts INTEGER DEFAULT (unixepoch()))", []).ok();
    let handle = alloc();
    let conn_arc = Arc::new(Mutex::new(conn));
    lock!(db_conns()).insert(handle, conn_arc);
    handle
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_save_message(db: i64, session_ptr: i64, role_ptr: i64, content_ptr: i64) {
    let strs = lock!(strs());
    let session = strs.get(&session_ptr).cloned().unwrap_or_default();
    let role = strs.get(&role_ptr).cloned().unwrap_or_default();
    let content = strs.get(&content_ptr).cloned().unwrap_or_default();
    drop(strs);
    if let Some(conn) = lock!(db_conns()).get(&db).cloned() {
        let c = lock!(conn);
        c.execute("INSERT INTO messages (session, role, content) VALUES (?1, ?2, ?3)", rusqlite::params![session, role, content]).ok();
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_load_messages(db: i64, session_ptr: i64, limit: i64) -> i64 {
    let session = lock!(strs()).get(&session_ptr).cloned().unwrap_or_default();
    let json = if let Some(conn) = lock!(db_conns()).get(&db).cloned() {
        let c = lock!(conn);
        let mut stmt = match c.prepare("SELECT role, content, ts FROM messages WHERE session=?1 ORDER BY id ASC LIMIT ?2") { Ok(s) => s, Err(_) => { return alloc_empty(); } };
        let rows: Vec<String> = stmt.query_map(rusqlite::params![session, limit], |row| {
            let r: String = row.get(0)?;
            let ct: String = row.get(1)?;
            let ts: i64 = row.get(2)?;
            Ok(format!(r#"{{"role":"{}","content":"{}","ts":{}}}"#, r, ct.replace('"', r#"\""#).replace('\n', r#"\n"#), ts))
        }).ok().map(|m| m.filter_map(|r| r.ok()).collect::<Vec<String>>()).unwrap_or_default();
        format!("[{}]", rows.join(","))
    } else { "[]".to_string() };
    let ptr = alloc(); lock!(strs()).insert(ptr, json); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_load_messages_since(db: i64, session_ptr: i64, ts: i64) -> i64 {
    let session = lock!(strs()).get(&session_ptr).cloned().unwrap_or_default();
    let json = if let Some(conn) = lock!(db_conns()).get(&db).cloned() {
        let c = lock!(conn);
        let mut stmt = match c.prepare("SELECT role, content, ts FROM messages WHERE session=?1 AND ts>=?2 ORDER BY id ASC") { Ok(s) => s, Err(_) => { return alloc_empty(); } };
        let rows: Vec<String> = stmt.query_map(rusqlite::params![session, ts], |row| {
            let r: String = row.get(0)?;
            let ct: String = row.get(1)?;
            let t: i64 = row.get(2)?;
            Ok(format!(r#"{{"role":"{}","content":"{}","ts":{}}}"#, r, ct.replace('"', r#"\""#).replace('\n', r#"\n"#), t))
        }).ok().map(|m| m.filter_map(|r| r.ok()).collect::<Vec<String>>()).unwrap_or_default();
        format!("[{}]", rows.join(","))
    } else { "[]".to_string() };
    let ptr = alloc(); lock!(strs()).insert(ptr, json); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_close_memory(db: i64) {
    lock!(db_conns()).remove(&db);
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_last_error() -> i64 {
    let msg = LAST_ERROR.get().and_then(|e| Some(lock!(e).clone())).unwrap_or_default();
    let ptr = alloc();
    lock!(strs()).insert(ptr, msg);
    ptr
}
