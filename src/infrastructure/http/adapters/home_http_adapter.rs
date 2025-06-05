use axum::response::Html;

use crate::application::IHomeGetDataUseCase;
use crate::presentation::views::home_view;

pub struct HomeHttpAdapter<T> {
    home_use_case: T,
}

impl<T: IHomeGetDataUseCase> HomeHttpAdapter<T> {
    pub fn new(home_use_case: T) -> HomeHttpAdapter<T> {
        HomeHttpAdapter { home_use_case }
    }

    pub async fn execute(&self) -> Html<String> {
        let home_data = self.home_use_case.execute().await.unwrap();

        home_view(home_data).await
    }
}
