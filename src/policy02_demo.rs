mod document_handler;
mod policy_utils;

use policy_utils::{is_authorized, display_header};

const DOCUMENTS_FOLDER: &str = "documents_folder";
const DEFAULT_USER: &str = "Bob";
const POLICY_FILE: &str = "policy02.cedar";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_USER.to_string());

    display_header(&user, DOCUMENTS_FOLDER, POLICY_FILE);
    display_policy_rules();

    // Test editing a document in the documents folder
    test_action(&user, "edit")?;

    // Test creating a file in the documents folder
    test_action(&user, "create")?;

    // Test deleting a file from the documents folder
    test_action(&user, "delete")?;

    Ok(())
}

/// Display the policy rules for reference
fn display_policy_rules() {
    println!("Policy Rules:");
    println!("  • Bob & Steve can edit documents");
    println!("  • Only Steve can create files");
    println!("  • Only Steve can delete files\n");
}

/// Test if a user is authorized to perform an action on the resource
fn test_action(user: &str, action: &str) -> Result<(), Box<dyn std::error::Error>> {
    let authorized = is_authorized(user, action, DOCUMENTS_FOLDER, POLICY_FILE)?;

    // Display the result
    let status = if authorized { "✓ ALLOWED" } else { "✗ DENIED" };
    println!("{status}: {user} to {action} in {DOCUMENTS_FOLDER}");

    Ok(())
}
