terraform {
  backend "gcs" {
    bucket = "terraform-state-ywkewq7z8a"
    prefix = "staging/services/mailbox"
  }
}