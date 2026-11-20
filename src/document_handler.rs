use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

/// Updates a document by appending a new entry with the user who made the update.
/// This demonstrates a document modification operation that would be authorized by policy01.cedar.
pub fn update_document(path: &Path, user: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(file, "Updated by {user}")
}

/// Reads and displays the current contents of a document.
pub fn read_document(path: &Path) -> io::Result<String> {
    std::fs::read_to_string(path)
}

/// Displays document update history with user information.
pub fn display_update_history(path: &Path) -> io::Result<()> {
    match read_document(path) {
        Ok(contents) => {
            println!("\n--- Document Update History ---");
            println!("{}", contents);
            println!("-------------------------------\n");
            Ok(())
        }
        Err(_) => {
            println!("\n[Document is empty or does not exist]\n");
            Ok(())
        }
    }
}
