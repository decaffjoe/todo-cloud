# Todo-Cloud

Disclaimer: this is a project for the sake of practicing gcp and rust,  
this is not a good way to achieve multi-device editing 😅

Goals:

- Once done editing the `todo.md` file, encrypt and upload to  
  a GCP storage bucket
- Can visit a static site in the browser which decrypts and renders  
  the content from the bucket
- Everything is owned by the user:
  - cloud bucket
  - static site
  - source/binaries
- Everything should be configured in one initial CLI setup session

## Getting Setup

### GCP

- Create a project
- Download your authentication credentials e.g. `project-keys.json`
  - The output of `gcloud auth list` is a good double check
  - Confirming keys exist in `$HOME/.config/gcloud` is another good double check

### Terraform

- Must create `tf/terraform.tfvars` file that creates the variables  
  specified in `tf/variable.tf` e.g.

```tf
# tf/terraform.tfvars

gcp_project_id  = "some--id-fjiw322"
gcp_region      = "us-central1"
gcp_location    = "US"
gcp_bucket_name = "some-bucket-wjfiowej"
```

- Run `terraform apply`

### Binaries

- Clone this repo, create the upload binary from go or rust
  - `cd go && go build`, or
  - `cd rs && cargo build -r`
- Create an alias in your shell's config file e.g. `.zshrc`

```sh
alias todo = vim /path/to/todo.md && /path/to/upload/binary
```

- Create the configuration file `$HOME/.config/todo-cloud/todo-cloud.toml`

```toml
gcp_bucket_name = 'bucket-name'
gcp_creds_file_path = '$HOME/.config/gcloud/my-project-keys.json'
todo_file_path = '$HOME/Documents/todo.md'
encryption_passphrase = 'blahblahblah'
```

- All set now!
  - `source $HOME/.zshrc` or open a new terminal
  - `todo` -> note the console output and changes to your gcp bucket after
    exiting your editor
