mod document_handler;
mod policy_utils;

use std::path::Path;
use document_handler::{update_document, display_update_history};
use policy_utils::{is_authorized, display_header};

const DOCUMENT: &str = "document.txt";
const DEFAULT_USER: &str = "Alice";
const POLICY_FILE: &str = "policy01.cedar";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_USER.to_string());

    display_header(&user, DOCUMENT, POLICY_FILE);
    
    // Check authorization using policy01.cedar
    if is_authorized(&user, "read", DOCUMENT, POLICY_FILE)? {
        // If authorized, update the document
        update_document(Path::new(DOCUMENT), &user)?;
        println!("✓ Document updated by {user}\n");
    } else {
        println!("✗ {user} is not authorized to update this document\n");
    }

    // Display the document's update history
    display_update_history(Path::new(DOCUMENT))?;

    Ok(())
}
