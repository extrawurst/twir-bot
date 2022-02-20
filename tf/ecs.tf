data "template_file" "taskdefinitions" {
  template = file("./task.json.tpl")
  vars = {
    region    = var.region
    loggroup  = aws_cloudwatch_log_group.loggroup.name
    api-image = var.docker-image
  }
}

resource "aws_ecs_task_definition" "service" {
  family       = "server"
  network_mode = "awsvpc"
  # task_role_arn            = aws_iam_role.ecs_task_role.arn
  execution_role_arn       = aws_iam_role.ecs_task_execution_role.arn
  cpu                      = "256"
  memory                   = "512"
  requires_compatibilities = ["FARGATE"]
  container_definitions    = data.template_file.taskdefinitions.rendered
  tags                     = var.tags
}

resource "aws_ecs_cluster" "api" {
  name = "ecs-cluster"
  tags = var.tags
}

resource "aws_ecs_service" "api" {
  name                 = "server"
  tags                 = var.tags
  cluster              = aws_ecs_cluster.api.id
  task_definition      = aws_ecs_task_definition.service.arn
  launch_type          = "FARGATE"
  force_new_deployment = true

  desired_count                      = 1
  deployment_minimum_healthy_percent = 0
  deployment_maximum_percent         = 100

  network_configuration {
    security_groups  = [aws_security_group.ecs_tasks.id]
    subnets          = aws_subnet.public.*.id
    assign_public_ip = true
  }

  depends_on = [
    aws_iam_role_policy_attachment.ecs_task_execution_role,
  ]
}
