use rusqlite::Connection;

#[allow(unused_assignments)]
pub fn init_database(conn: &Connection) {

    fn upgrade_message(version: usize) {
        println!("Upgrading DB version {}, stand by...", version);
    }

    let mut db_version:i64 = conn.query_row("PRAGMA user_version",&[], |row| {row.get(0)})
        .expect("lookup db table version");
    if db_version == 0 {
        upgrade_message(0);
        let altered = conn.execute("ALTER TABLE transfers ADD COLUMN GNBR INTEGER NOT NULL DEFAULT 0", &[]).is_ok();
        if altered {
            conn.execute("UPDATE transfers SET GNBR = amount * (SELECT products.gateway
                    FROM products
                    WHERE products.id = transfers.ProductID )
                    WHERE EXISTS (
                    SELECT *
                    FROM products
                    WHERE products.id = transfers.ProductID
                )", &[])
                .expect("update producer entry in transfers table");
            conn.execute("PRAGMA user_version = 1", &[])
                .expect("alter db version");
            db_version = 1;
        } else {
            conn.execute("CREATE TABLE IF NOT EXISTS users (
                    id              INTEGER PRIMARY KEY AUTOINCREMENT,
                    name            TEXT NOT NULL,
                    NBR             INTEGER NOT NULL,
                    password        TEXT NOT NULL,
                    time_created    TEXT NOT NULL
                    )", &[])
                .expect("create users table");

            conn.execute("CREATE TABLE IF NOT EXISTS products (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    name        TEXT NOT NULL,
                    gateway     INTEGER NOT NULL,
                    benefit     INTEGER NOT NULL,
                    time_created    TEXT NOT NULL,
                    resabundance    INTEGER,
                    resabundancetun INTEGER,
                    prodpop     INTEGER,
                    consdem     INTEGER,
                    proddembalance  INTEGER,
                    conssubsat  INTEGER,
                    conssubsattun   INTEGER,
                    consobjben  INTEGER,
                    consobjbentun   INTEGER,
                    consbenefit  INTEGER,
                    socbenefit  INTEGER,
                    socbenefittun   INTEGER,
                    enveffect  INTEGER,
                    enveffecttun   INTEGER,
                    humaneffect INTEGER,
                    humaneffecttun  INTEGER,
                    envbenefit INTEGER
                )", &[])
                .expect("create products table");

            conn.execute("CREATE TABLE IF NOT EXISTS transfers (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    ConsumerID     INTEGER NOT NULL,
                    ProducerID INTEGER NOT NULL,
                    ProductID  INTEGER NOT NULL,
                    amount     INTEGER NOT NULL,
                    NBR        INTEGER NOT NULL,
                    time_created    TEXT NOT NULL,
                    GNBR       INTEGER NOT NULL
                )", &[])
                .expect("create withdrawals table");
            conn.execute("PRAGMA user_version = 2", &[])
                .expect("alter db version");
            db_version = 2;
        }

    }
    if db_version == 1 {
        upgrade_message(1);
        conn.execute("CREATE TABLE IF NOT EXISTS transfers2 (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    ConsumerID     INTEGER NOT NULL,
                    ProducerID INTEGER NOT NULL,
                    ProductID  INTEGER NOT NULL,
                    amount     INTEGER NOT NULL,
                    NBR        INTEGER NOT NULL,
                    time_created    TEXT NOT NULL,
                    GNBR       INTEGER NOT NULL
                )", &[]).expect("alter db modify column");
        conn.execute("INSERT INTO transfers2 SELECT * FROM transfers", &[]).expect("alter db modify column");
        conn.execute("DROP TABLE transfers", &[]).expect("alter db modify column");
        conn.execute("ALTER TABLE transfers2 RENAME TO transfers", &[]).expect("alter db modify column");

        conn.execute("ALTER TABLE products ADD COLUMN resabundance INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN resabundancetun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN prodpop INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN consdem INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN proddembalance INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN conssubsat INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN conssubsattun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN consobjben INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN consobjbentun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN consbenefit INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN socbenefit INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN socbenefittun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN enveffect INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN enveffecttun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN humaneffect INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN humaneffecttun INTEGER", &[]).expect("alter db add column");
        conn.execute("ALTER TABLE products ADD COLUMN envbenefit INTEGER", &[]).expect("alter db add column");

        conn.execute("PRAGMA user_version = 2", &[])
            .expect("alter db version");
        db_version = 2;
    }
    if db_version == 2 {
        upgrade_message(2);
        conn.execute("INSERT OR IGNORE INTO users (id, name, NBR, password, time_created)\
        VALUES (0, '-', 0, 0, datetime('now','localtime'))", &[])
            .expect("insert single entry into users table");

        conn.execute("ALTER TABLE transfers ADD COLUMN comment TEXT", &[]).expect("alter db add column");

        conn.execute("PRAGMA user_version = 3", &[])
            .expect("alter db version");
        db_version = 3;
    }
}