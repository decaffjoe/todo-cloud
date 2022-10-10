# Things to do

- create mobile-first static site to decrypt and render bucket content
- meta CLI setup to initialize everything in one go:
  - get config variables from user
  - create `tf/terraform.tfvars`
  - create `$HOME/.config/todo-cloud/todo-cloud.toml`
  - run `terraform apply`
  - build upload binary
  - add `todo` alias to shell config file
  - inject bucket name into static site source
  - build + deploy static site, output deployed URL
