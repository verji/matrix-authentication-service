// Copyright 2023 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::{extract::Query, Form};
use axum_extra::extract::PrivateCookieJar;
use mas_keystore::Encrypter;
use mas_storage::{BoxClock, BoxRepository};
use oauth2_types::requests::RPInitiatedLogoutRequest;

pub async fn get(
    clock: BoxClock,
    repo: BoxRepository,
    cookie_jar: PrivateCookieJar<Encrypter>,
    Query(request): Query<RPInitiatedLogoutRequest>,
) {
    handle(clock, repo, cookie_jar, request).await
}

pub async fn post(
    clock: BoxClock,
    repo: BoxRepository,
    cookie_jar: PrivateCookieJar<Encrypter>,
    Form(request): Form<RPInitiatedLogoutRequest>,
) {
    handle(clock, repo, cookie_jar, request).await
}

pub async fn handle(
    clock: BoxClock,
    mut repo: BoxRepository,
    cookie_jar: PrivateCookieJar<Encrypter>,
    request: RPInitiatedLogoutRequest,
) {
    todo!()
}
