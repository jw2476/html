use html::htmx::*;
use html::style::*;
use html::tangle::*;
use html::*;

struct User {
    username: String,
    password: String,
}

fn usernames(users: &[User]) -> Vec<P> {
    users
        .iter()
        .map(|user| &user.username)
        .map(|username| p(username))
        .collect::<Vec<_>>()
}

fn get_users() -> Vec<User> {
    vec![
        User {
            username: "Jw2476".to_string(),
            password: "owo".to_string(),
        },
        User {
            username: "cam".to_string(),
            password: "password123".to_string(),
        },
    ]
}

fn page() -> Html {
    let users = get_users();

    html((
        head((title("Hello, World!"), script("https://unpkg.com/htmx.org@1.9.10".to_string()))),
        body((
            div(usernames(&users))
                .padding(Edges::all(rem!(1.0)))
                .border(Edges::all(px!(1.0)))
                .get("https://google.com/")
                .trigger("click")
                .target(Target::This)
                .swap(Swap::Inner),
            image(
                "https://jw2476.dev/favicon.ico".to_string(),
                "icon".to_string(),
            ),
        )),
    ))
}

pub fn main() {
    println!("{}", page().render());
}
