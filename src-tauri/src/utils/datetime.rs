//! Date and time utilities

/// Handle the **Serialization** and the **Deserialization** of a
/// **DateTime** to/from an **ISO 8601** formatted string with `serde`.
pub mod serde_iso_8601 {
  const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

  use chrono::NaiveDateTime;
  use serde::{de, ser};

  /// Serialize a datetime into an **ISO 8601** formatted string.
  ///
  /// Intended for use with `serde`s `serialize_with` attribute.
  ///
  /// # Example:
  ///
  /// ```rust
  /// use chrono::{NaiveDate, NaiveDateTime};
  /// use serde::Serialize;
  /// use redmium::utils::datetime::serde_iso_8601;
  ///
  /// #[derive(Serialize)]
  /// struct S {
  ///   #[serde(serialize_with = "serde_iso_8601::serialize")]
  ///   datetime: NaiveDateTime,
  /// }
  ///
  /// let my_s = S {
  ///   datetime: NaiveDate::from_ymd_opt(2015, 5, 15)
  ///     .unwrap()
  ///     .and_hms_opt(10, 0, 0)
  ///     .unwrap(),
  /// };
  ///
  /// let serialized = serde_json::to_string(&my_s).unwrap();
  ///
  /// assert_eq!(r#"{"datetime":"2015-05-15T10:00:00Z"}"#, serialized);
  /// ```
  pub fn serialize<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    serializer.serialize_str(dt.format(DATETIME_FORMAT).to_string().as_str())
  }

  /// Deserialize a `NaiveDateTime` from an **ISO 6801** formatted string.
  ///
  /// Intended for use with `serde`s `deserialize_with` attribute.
  ///
  /// # Example:
  ///
  /// ```rust
  /// use chrono::{NaiveDate, NaiveDateTime};
  /// use serde::Deserialize;
  /// use redmium::utils::datetime::serde_iso_8601;
  ///
  /// #[derive(Deserialize, PartialEq, Debug)]
  /// struct S {
  ///   #[serde(deserialize_with = "serde_iso_8601::deserialize")]
  ///   datetime: NaiveDateTime
  /// }
  ///
  /// let my_s: S = serde_json::from_str(r#"{"datetime":"2015-05-15T10:00:00Z"}"#).unwrap();
  ///
  /// assert_eq!(
  ///   S {
  ///     datetime: NaiveDate::from_ymd_opt(2015, 5, 15)
  ///       .unwrap()
  ///       .and_hms_opt(10, 0, 0)
  ///       .unwrap(),
  ///   },
  ///   my_s,
  /// );
  /// ```
  pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    deserializer.deserialize_str(NaiveDateTimeVisitor)
  }

  struct NaiveDateTimeVisitor;

  impl<'de> de::Visitor<'de> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      formatter.write_str("a string in the ISO 8601 format")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      match NaiveDateTime::parse_from_str(v, DATETIME_FORMAT) {
        Ok(naive_datetime) => Ok(naive_datetime),
        Err(parse_error) => Err(E::custom(parse_error)),
      }
    }
  }
}
