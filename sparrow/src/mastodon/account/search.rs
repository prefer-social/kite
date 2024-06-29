use crate::mastodon::account::Account;

impl Account {
    async fn search(arg: String) {

        // Determine form of arg
        // @foo@bar.com or foo.bar.com
        // https://bar.com/@foo htps://bar.com/users/foo

        // (2) call webfinger.




    }

    async fn lookup_webfinger(acct: String) {

    }


}




// Acct: The Webfinger account URI. Equal to username for local users, or username@domain for remote users.
// Url: The location of the user’s profile page
//

// acct: foo@bar.com <- The Webfinger account URI.
// url: https://bar.com/@foo <- The location of the user's profile page. "text/html",
// uri: https://bar.com/users/foo  <- Self. "application/activity+json",