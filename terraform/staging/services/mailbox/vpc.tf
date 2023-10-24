# VPC info
resource "google_compute_network" "vpc" {
  project = var.project_id
  name                    = "${var.project_id}-vpc"
  auto_create_subnetworks = false
  routing_mode = "GLOBAL"
}

# Subnet
resource "google_compute_subnetwork" "subnet" {
  name          = "${var.project_id}-subnet"
  region        = var.region
  network       = google_compute_network.vpc.self_link
  project = var.project_id
  ip_cidr_range = "10.10.0.0/20"
  role = "ACTIVE"
  secondary_ip_range {
    range_name    = "pod-ranges"
    ip_cidr_range = "10.12.0.0/14"
  }
  secondary_ip_range {
    range_name    = "services-range"
    ip_cidr_range = "10.11.0.0/20"
  }
}

resource "google_compute_global_address" "private_ip_address" {
  project = var.project_id
  name          = "${var.project_id}-private-ip-address"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = google_compute_network.vpc.self_link
}

resource "google_service_networking_connection" "private_vpc_connection" {
  network                 = google_compute_network.vpc.self_link
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_ip_address.name]
}