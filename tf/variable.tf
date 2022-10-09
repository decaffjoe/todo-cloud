variable "gcp_project_id" {
  type        = string
  description = "GCP Project Id"
}

variable "gcp_region" {
  type    = string
  default = "us-central1"
}

variable "gcp_location" {
  type    = string
  default = "US"
}

variable "gcp_bucket_name" {
  type = string
  default = "Must be globally unique- is also globally searchable. Do not include personal information!"
}