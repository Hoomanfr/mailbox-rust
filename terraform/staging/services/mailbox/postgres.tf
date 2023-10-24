resource "random_id" "db_name_suffix" {
  byte_length = 4
}
# cnstr: postgresql://admin_ritgsrtzhz:8ybrlc67xd4ulw6k2jle@localhost:5432/mailbox
resource "google_sql_database_instance" "instance" {
  name             = "${var.project_name}-${random_id.db_name_suffix.hex}"
  database_version = "POSTGRES_14"
  region           = var.region
  deletion_protection = false
  settings {
    tier = "db-f1-micro"
    ip_configuration {
      ipv4_enabled = true
      private_network = google_compute_network.vpc.self_link
      require_ssl = false
    }
  }
  depends_on = [google_service_networking_connection.private_vpc_connection]
}

resource "google_sql_user" "users" {
  name     = var.pg_username
  password = var.pg_password
  instance = google_sql_database_instance.instance.name
}