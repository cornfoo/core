use primitives::Chain;
use std::str::FromStr;

uniffi::custom_type!(Chain, String, {
  remote,
  lower: |chain| chain.to_string(),
  try_lift: |s| Chain::from_str(&s)
      .map_err(|_| uniffi::deps::anyhow::Error::msg("Invalid Chain")),
});
