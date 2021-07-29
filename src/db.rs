use rocket_sync_db_pools::rusqlite::Connection;

#[allow(unused_assignments)]
pub fn init_database(conn: &Connection) {

    fn upgrade_message(version: usize) {
        println!("Upgrading DB version {}, stand by...", version);
    }

    let mut db_version:i64 = conn.query_row("PRAGMA user_version",[], |row| {row.get(0)})
        .expect("lookup db table version");
    if db_version == 0 {
        upgrade_message(0);
        conn.execute("CREATE TABLE users (
                    id              INTEGER PRIMARY KEY,
                    name            TEXT NOT NULL,
                    NBR             REAL NOT NULL,
                    password        TEXT,
                    time_created    TEXT NOT NULL
                    )", [])
            .expect("create table");

        conn.execute("CREATE TABLE products (
                    id          INTEGER PRIMARY KEY,
                    name        TEXT NOT NULL,
                    gateway     REAL NOT NULL,
                    benefit     REAL NOT NULL,
                    time_created    TEXT NOT NULL,
                    resabundance    REAL,
                    beneficiaries  REAL,
                    producers  REAL,
                    ccs  REAL,
                    conssubben  REAL,
                    cco  REAL,
                    consobjben  REAL,
                    ceb  REAL,
                    envben  REAL,
                    chb  REAL,
                    humanben REAL
                )", [])
            .expect("create table");

        conn.execute("CREATE TABLE transfers (
                    id          INTEGER PRIMARY KEY,
                    ConsumerID     INTEGER NOT NULL,
                    ProducerID INTEGER NOT NULL,
                    ProductID  INTEGER NOT NULL,
                    amount     REAL NOT NULL,
                    NBR        REAL NOT NULL,
                    GNBR       REAL NOT NULL,
                    time_created    TEXT NOT NULL,
                    comment    TEXT
                )", [])
            .expect("create table");
        conn.execute("INSERT OR IGNORE INTO users (id, name, NBR, password, time_created)
        VALUES (0, '-', 0, 0, datetime('now','localtime'))", [])
            .expect("insert single entry into table");

        conn.execute("CREATE TABLE user_products (
                    ProductID          INTEGER,
                    UserID         INTEGER,
                    gateway     REAL NOT NULL,
                    benefit     REAL NOT NULL,
                    time_created    TEXT NOT NULL,
                    resabundance    REAL,
                    beneficiaries  REAL,
                    producers  REAL,
                    ccs  REAL,
                    conssubben  REAL,
                    cco  REAL,
                    consobjben  REAL,
                    ceb  REAL,
                    envben  REAL,
                    chb  REAL,
                    humanben REAL,
                    PRIMARY KEY ( ProductID, UserID)
                )", [])
            .expect("create table");

        db_version = 4;

    }
    if db_version < 4 { panic!("DB upgrade not implemented!")}
    if db_version == 4 {
        upgrade_message(4);
        conn.execute("ALTER TABLE users ADD COLUMN fame REAL NOT NULL DEFAULT 0", []).expect("alter db add column");
        conn.execute("UPDATE users SET fame = NBR", []).expect("update users fame");
        conn.execute("PRAGMA user_version = 5", [])
            .expect("alter db version");
        db_version = 5;
    }

}
