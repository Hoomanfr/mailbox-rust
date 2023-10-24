resource "google_kms_key_ring" "vault_keyring" {
  name     = "vault-keyring"
  location = "global"
}

resource "google_kms_crypto_key" "vault_crypto_key" {
  name            = "vault-crypto-key"
  key_ring        = google_kms_key_ring.vault_keyring.id
  rotation_period = "100000s"

  lifecycle {
    prevent_destroy = true
  }
}