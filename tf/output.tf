output "cd_user_id" {
  description = "use for AWS_ACCESS_KEY_ID in ci"
  value       = aws_iam_access_key.cd_user.id
}

output "cd_user_secret" {
  description = "use for AWS_SECRET_ACCESS_KEY in ci"
  value       = aws_iam_access_key.cd_user.secret
  sensitive   = true
}
