use anyhow::Error;
use yew::callback::Callback;
use yew::services::fetch::Response;

pub type RawFetchResponse<T> = Response<Result<T, Error>>;
pub type RawFetchCallback<T> = Callback<RawFetchResponse<T>>;
