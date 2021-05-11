use notify_rust::{Hint, Notification};

/// Notify the user about the alert
#[allow(dead_code)]
pub fn notify() {
    Notification::new()
        .summary("Firefox News")
        .body("This will almost look like a real firefox notification.")
        .icon("firefox")
        .show();

    // Notification::new()
    //     .summary("Category:email")
    //     .body(
    //         "This has nothing to do with emails.\nIt should not go away until you acknowledge it.",
    //     )
    //     .icon("thunderbird")
    //     .appname("thunderbird")
    //     .hint(Hint::Category("email".to_owned()))
    //     .hint(Hint::Resident(true)) // this is not supported by all implementations
    //     .timeout(0) // this however is
    //     .show();
}
