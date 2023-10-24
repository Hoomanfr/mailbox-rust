resource "google_service_account" "hooman-domain-sa" {
  account_id   = "hooman-domain-sa"
  display_name = "Hooman Domain Service Account"
}
# https://cloud.google.com/iam/docs/understanding-roles
resource "google_project_iam_member" "hooman-domain-sa-acr-role" {
  project = var.project_id
  role = "roles/artifactregistry.admin"
  member = "serviceAccount:${google_service_account.hooman-domain-sa.email}"
}

resource "google_project_iam_member" "hooman-domain-sa-token-creator-role" {
  project = var.project_id
  role = "roles/iam.serviceAccountTokenCreator"
  member = "serviceAccount:${google_service_account.hooman-domain-sa.email}"
}

resource "google_project_iam_member" "hooman-domain-sa-storage-admin-role" {
  project = var.project_id
  role = "roles/storage.admin"
  member = "serviceAccount:${google_service_account.hooman-domain-sa.email}"
}

resource "google_project_iam_member" "hooman-domain-sa-storage-object-admin-role" {
  project = var.project_id
  role = "roles/storage.objectAdmin"
  member = "serviceAccount:${google_service_account.hooman-domain-sa.email}"
}

resource "google_project_iam_member" "hooman-domain-sa-sql-client-role" {
  project = var.project_id
  role = "roles/cloudsql.client"
  member = "serviceAccount:${google_service_account.hooman-domain-sa.email}"
}