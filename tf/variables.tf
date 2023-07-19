variable "access_key" {}
variable "secret_key" {}
variable "account_id" {}

variable "discord_token" {}
variable "channel_id" {}

variable "region" {
  default = "eu-west-1"
}

variable "tags" {
  default = {
    Application = "twir"
  }
}

variable "docker-image" {
  default = "ghcr.io/extrawurst/twir-bot"
}

variable "cidr" {
  description = "The CIDR block for the VPC."
  default     = "10.0.0.0/16"
}

variable "public_subnets" {
  description = "a list of CIDRs for public subnets in your VPC, must be set if the cidr variable is defined, needs to have as many elements as there are availability zones"
  default     = ["10.0.16.0/20"]
}

variable "availability_zones" {
  description = "a comma-separated list of availability zones, defaults to all AZ of the region, if set to something other than the defaults, both private_subnets and public_subnets have to be defined as well"
  default     = ["eu-west-1a"]
}
