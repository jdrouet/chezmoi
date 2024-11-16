use axum::response::Html;
use chezmoi_client::view::prelude::View;

pub(super) async fn handle() -> Html<String> {
    Html(chezmoi_client::view::home::View::default().render())
}
