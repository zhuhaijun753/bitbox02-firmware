// Copyright 2020 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{confirm, status, trinary_input_string};
use bitbox02::input::SafeInputString;

pub use trinary_input_string::CanCancel;

/// If `can_cancel` is `Yes`, the workflow can be cancelled.
/// If it is no, the result is always `Ok(())`.
///
/// Example:
/// ```no_run
/// let pw = enter("Enter password", true, CanCancel::No).await.unwrap();
/// // use pw.
/// ```
pub async fn enter(
    title: &str,
    special_chars: bool,
    can_cancel: CanCancel,
) -> Result<SafeInputString, super::cancel::Error> {
    let params = trinary_input_string::Params {
        title,
        hide: true,
        special_chars,
        longtouch: true,
        ..Default::default()
    };
    trinary_input_string::enter(&params, can_cancel, "").await
}

/// Prompt the user to enter a password twice. A warning is displayed
/// if the password has fewer than 4 chars. Returns `Err` if the two
/// passwords do not match, or if the user aborts at the warning.
///
/// Example:
/// ```no_run
/// let pw = enter_twice().await.unwrap();
/// // use pw.
pub async fn enter_twice() -> Result<SafeInputString, ()> {
    let password = enter("Set password", false, CanCancel::No)
        .await
        .expect("not cancelable");
    let password_repeat = enter("Repeat password", false, CanCancel::No)
        .await
        .expect("not cancelable");
    if password.as_str() != password_repeat.as_str() {
        status::status("Passwords\ndo not match", false).await;
        return Err(());
    }
    if password.as_str().len() < 4 {
        let params = confirm::Params {
            title: "WARNING",
            body: "Your password\n has fewer than\n 4 characters.\nContinue?",
            longtouch: true,
            ..Default::default()
        };

        if !confirm::confirm(&params).await {
            return Err(());
        }
    }
    status::status("Success", true).await;
    Ok(password)
}
