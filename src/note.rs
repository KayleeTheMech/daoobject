use rusqlite::params;
use rusqlite::Connection;

pub struct TwitterNote {
    pub user_id: u32,
    pub twitter_account_id: u32,
    pub text: String,
}

pub struct TwitterNoteDAO {
    connection: Connection,
}
const INSERT_NOTE_SQL: &str =
    "INSERT INTO notes (user_id, twitter_account_id, text) values (?1, ?2, ?3)";
const SELECT_NOTE_SQL: &str = "SELECT * from notes WHERE user_id=?1 AND twitter_account_id=?2;";
const DELETE_NOTE_SQL: &str = "DELETE from notes WHERE user_id=?1 AND twitter_account_id=?2;";
const UPDATE_NOTE_SQL: &str =
    "UPDATE notes SET text = ?3 WHERE user_id=?1 AND twitter_account_id=?2;";
const EXISTS_NOTE_SQL: &str =
    "SELECT EXISTS(SELECT * from notes WHERE user_id=?1 AND twitter_account_id=?2);";

impl TwitterNoteDAO {
    pub fn new(db_name: String) -> Self {
        if let Ok(conn) = Connection::open(&db_name) {
            Self { connection: conn }
        } else {
            panic!("Couldn't get a DB connection")
        }
    }

    pub fn exists(&self, user_id: u32, twitter_account_id: u32) -> Option<bool> {
        let mut statement = self.connection.prepare(EXISTS_NOTE_SQL).unwrap();
        let query = statement.query(params![&user_id, &twitter_account_id]);
        match query {
            Ok(mut rows) => match rows.next().unwrap() {
                Some(row) => Some(row.get_unwrap(0)),
                _ => None,
            },
            Err(_err) => {
                println!("Failure to execute query.");
                None
            }
        }
    }

    pub fn get(&self, user_id: u32, twitter_account_id: u32) -> Option<TwitterNote> {
        let mut statement = self.connection.prepare(SELECT_NOTE_SQL).unwrap();
        let query = statement.query(params![&user_id, &twitter_account_id]);
        match query {
            Ok(mut rows) => match rows.next().unwrap() {
                Some(row) => Some(TwitterNote {
                    user_id: row.get_unwrap(0),
                    twitter_account_id: row.get_unwrap(1),
                    text: row.get_unwrap(2),
                }),
                _ => None,
            },
            Err(_err) => {
                println!("Failure to execute query.");
                None
            }
        }
    }

    pub fn add(&self, note: TwitterNote) {
        let mut statement = self.connection.prepare(INSERT_NOTE_SQL).unwrap();
        let query = statement.execute(params![&note.user_id, &note.twitter_account_id, &note.text]);
        match query {
            Ok(updated) => println!("Success"),
            Err(err) => println!("Fail"),
        }
    }

    pub fn update(&self, note: TwitterNote) {
        let mut statement = self.connection.prepare(UPDATE_NOTE_SQL).unwrap();
        let query = statement.execute(params![&note.user_id, &note.twitter_account_id, &note.text]);
        match query {
            Ok(updated) => println!("Success"),
            Err(err) => println!("Fail"),
        }
    }

    pub fn delete(&self, user_id: u32, twitter_account_id: u32) {
        let mut statement = self.connection.prepare(DELETE_NOTE_SQL).unwrap();
        let query = statement.execute(params![&user_id, &twitter_account_id]);
        match query {
            Ok(updated) => println!("Success"),
            Err(err) => println!("Fail"),
        }
    }
}
