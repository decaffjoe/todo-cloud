terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "4.39.0"
    }
  }
}

provider "google" {
  project = var.gcp_project_id
  region  = var.gcp_region
}

resource "google_storage_bucket" "todo_bucket" {
  name          = var.gcp_bucket_name
  location      = var.gcp_location
  force_destroy = true
}

# Make bucket public
resource "google_storage_bucket_iam_member" "todo_bucket_member" {
  bucket = google_storage_bucket.todo_bucket.name
  role   = "roles/storage.legacyObjectReader"
  member = "allUsers"
}
