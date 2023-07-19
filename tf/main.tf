terraform {
  required_version = ">= 1.5.3"

  cloud {
    organization = "twir"

    workspaces {
      name = "twir"
    }
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.22"
    }
  }
}

provider "aws" {
  allowed_account_ids = [var.account_id]
  region              = var.region
  access_key          = var.access_key
  secret_key          = var.secret_key
}

resource "aws_cloudwatch_log_group" "loggroup" {
  name = "server"
  tags = var.tags
}
