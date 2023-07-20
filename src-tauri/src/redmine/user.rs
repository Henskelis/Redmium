//! Manage a **Redmine user**

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::utils::datetime;

/// `User` is a type that represents a **Redmine** user.
#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: u32,
  pub firstname: String,
  pub lastname: String,
  pub mail: String,
  pub login: String,
  pub admin: bool,
  pub status: Option<UserStatus>,

  #[serde(with = "datetime::serde_iso_8601")]
  pub last_login_on: NaiveDateTime,

  #[serde(with = "datetime::serde_iso_8601")]
  pub passwd_changed_on: NaiveDateTime,

  #[serde(with = "datetime::serde_iso_8601")]
  pub created_on: NaiveDateTime,

  #[serde(with = "datetime::serde_iso_8601")]
  pub updated_on: NaiveDateTime,
}

impl User {
  /// Build a `User` from a JSON string.
  ///
  /// The **Redmine API** does not return the `status` field when performing a bulk
  /// request to the **/users** endpoint. Because of this, the `status` property is
  /// optional in `User` and will be set to `None` if missing from the JSON input.
  /// Other fields are always returned and are therefore mandatory in the JSON
  /// input.
  ///
  /// # Example
  ///
  /// ```rust
  /// use redmium::redmine::user::{User, UserStatus};
  ///
  /// let json = r#"
  ///   {
  ///     "id": 1,
  ///     "login": "email@henskelis.fr",
  ///     "admin": true,
  ///     "firstname": "Hen",
  ///     "lastname": "SKELIS",
  ///     "mail": "email@henskelis.fr",
  ///     "created_on": "2015-10-30T12:09:31Z",
  ///     "updated_on": "2021-11-15T11:42:22Z",
  ///     "last_login_on": "2023-07-20T12:42:05Z",
  ///     "passwd_changed_on": "2015-10-30T12:09:31Z",
  ///     "status": 1
  ///   }
  /// "#;
  ///
  /// let user = User::build_from_json(json).unwrap();
  ///
  /// assert_eq!(user.login, "email@henskelis.fr");
  /// assert_eq!(user.admin, true);
  /// assert!(matches!(user.status, Some(UserStatus::Active)));
  /// assert_eq!(
  ///   user.created_on,
  ///   chrono::NaiveDate::from_ymd_opt(2015, 10, 30)
  ///     .unwrap()
  ///     .and_hms_opt(12, 9, 31)
  ///     .unwrap(),
  /// );
  /// ```
  ///
  /// # Errors
  ///
  /// The conversion can fail if the structure of the JSON input does not
  /// match the structure expected by `User`. It can also fail if the structure
  /// is correct but the implementation of `Deserialize` for `User` decides that
  /// something is wrong with the data like a required field missing or a number
  /// too big to fit in the expected primitive type.
  pub fn build_from_json(json: &str) -> Result<Self, serde_json::Error> {
    let user: Self = serde_json::from_str(json)?;
    Ok(user)
  }

  /// Build the JSON representation of the `User`.
  ///
  /// The `status` property can be `None` depending on how the `User` was constructed.
  /// In this case, the resulting JSON string will include a `status` field with a `null`
  /// value.
  ///
  /// # Example
  ///
  /// ```rust
  /// use chrono::NaiveDate;
  /// use redmium::redmine::user::{User, UserStatus};
  ///
  /// let user = User {
  ///   id: 1,
  ///   firstname: String::from("Hen"),
  ///   lastname: String::from("SKELIS"),
  ///   mail: String::from("email@henskelis.fr"),
  ///   login: String::from("email@henskelis.fr"),
  ///   admin: true,
  ///   status: None,
  ///   last_login_on: NaiveDate::from_ymd_opt(2023, 7, 20)
  ///     .unwrap()
  ///     .and_hms_opt(16, 23, 14)
  ///     .unwrap(),
  ///   passwd_changed_on: NaiveDate::from_ymd_opt(2015, 11, 5)
  ///     .unwrap()
  ///     .and_hms_opt(17, 25, 2)
  ///     .unwrap(),
  ///   created_on: NaiveDate::from_ymd_opt(2015, 10, 30)
  ///     .unwrap()
  ///     .and_hms_opt(12, 9, 31)
  ///     .unwrap(),
  ///   updated_on: NaiveDate::from_ymd_opt(2015, 11, 5)
  ///     .unwrap()
  ///     .and_hms_opt(17, 25, 2)
  ///     .unwrap(),
  /// };
  ///
  /// let expected_json = r#"
  ///   {
  ///     "id": 1,
  ///     "login": "email@henskelis.fr",
  ///     "admin": true,
  ///     "firstname": "Hen",
  ///     "lastname": "SKELIS",
  ///     "mail": "email@henskelis.fr",
  ///     "created_on": "2015-10-30T12:09:31Z",
  ///     "updated_on": "2015-11-05T17:25:02Z",
  ///     "last_login_on": "2023-07-20T16:23:14Z",
  ///     "passwd_changed_on": "2015-11-05T17:25:02Z",
  ///     "status": null
  ///   }
  /// "#;
  ///
  /// let actual_json = user.to_json();
  ///
  /// assert_eq!(
  ///   expected_json.parse::<serde_json::Value>().unwrap(),
  ///   actual_json.parse::<serde_json::Value>().unwrap(),
  /// );
  /// ```
  pub fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

/// `UserStatus` represents a **Redmine** user account status.
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserStatus {
  Anonymous,
  Active,
  Registered,
  Locked,
}
