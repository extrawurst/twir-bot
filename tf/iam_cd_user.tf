
resource "aws_iam_user" "cd_user" {
  name          = "cd-user"
  force_destroy = true
  tags          = var.tags
}

resource "aws_iam_access_key" "cd_user" {
  user = aws_iam_user.cd_user.name
}

data "aws_iam_policy_document" "cd_user" {
  version = "2012-10-17"
  statement {
    sid       = ""
    effect    = "Allow"
    actions   = ["ecs:UpdateService"]
    resources = [aws_ecs_service.api.id]
  }

  statement {
    sid       = ""
    effect    = "Allow"
    actions   = ["ecs:ListServices"]
    resources = ["*"]
  }
}

resource "aws_iam_user_policy" "cd_user" {
  name = "ecs_deploy_right"
  user = aws_iam_user.cd_user.name

  policy = data.aws_iam_policy_document.cd_user.json
}
