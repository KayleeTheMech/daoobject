mod note;
pub use self::note::{TwitterNote, TwitterNoteDAO};

fn main() {
    let note_dao = TwitterNoteDAO::new("test.sqlite3".to_string());
    let note = note_dao.get(1, 3);
    note_dao.add(TwitterNote {
        user_id: 2,
        twitter_account_id: 3,
        text: "test test test".to_string(),
    });
    let exists = note_dao.exists(2, 3);

    note_dao.update(TwitterNote {
        user_id: 2,
        twitter_account_id: 3,
        text: "test2".to_string(),
    });
    let note2 = note_dao.get(2, 3);
    note_dao.delete(2, 3);
    println!("Tada")
}
