resource "google_artifact_registry_repository" "hoomanfr" {
  location      = var.region
  repository_id = "hoomanfr"
  description   = "hoomanfr docker repository"
  format        = "DOCKER"
}