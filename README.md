# Todo-Cloud

- Once done editing the `todo.md` file in vim, encrypt and upload to  
  a GCP storage bucket
- Visit static site to decrypt content from bucket and view on mobile

## Terraform

- Must create `tf/terraform.tfvars` file that creates the variables  
specified in `tf/variable.tf` e.g.

```tf
# tf/terraform.tfvars

gcp_project_id  = "some--id-fjiw322"
gcp_region      = "us-central1"
gcp_location    = "US"
gcp_bucket_name = "some-bucket-wjfiowej"
```
