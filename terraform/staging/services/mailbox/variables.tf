variable "project_id" {
  type        = string
  description = "The Google project ID"
}

variable "project_name" {
  type        = string
  description = "The Google project name"
}

variable "region" {
  type        = string
  description = "The Google project ID"
}

variable "zone" {
  type        = string
  description = "The Google project ID"
}

variable "bucket" {
  type        = string
  description = "The Google project ID"
}

variable "prefix" {
  type        = string
  description = "The Google project ID"
}

variable "gke_username" {
  default     = ""
  description = "gke username"
}

variable "gke_password" {
  default     = ""
  description = "gke password"
}

variable "gke_num_nodes" {
  default     = 2 # Use 3 for production this is just for testing
  description = "number of gke nodes"
}

variable "pg_username" {
  type = string
  description = "postgres username"
}

variable "pg_password" {
  type = string
  description = "postgres password"
}