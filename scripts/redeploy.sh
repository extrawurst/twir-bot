#!/bin/sh
export AWS_ACCESS_KEY_ID=$1
export AWS_SECRET_ACCESS_KEY=$2
ECS_CLUSTER=ecs-cluster
ECS_SERVICE_ARN=$(aws ecs list-services --region=eu-west-1 --cluster=${ECS_CLUSTER} --output=text | head -1 | awk '{print $2}')
aws ecs update-service --region=eu-west-1 --service=${ECS_SERVICE_ARN} --cluster=${ECS_CLUSTER} --force-new-deployment > log.txt