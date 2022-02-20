resource "aws_security_group" "ecs_tasks" {
  name        = "ecs-tasks-sg"
  description = "only allow egress from ecs"
  vpc_id      = aws_vpc.main.id
  tags        = var.tags

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}
