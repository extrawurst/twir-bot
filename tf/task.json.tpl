[  
  {
    "name": "server",
    "image": "${api-image}",
    "essential": true,
    "logConfiguration": {
      "logDriver": "awslogs",
      "options": {
        "awslogs-region": "${region}",
        "awslogs-stream-prefix": "server",
        "awslogs-group": "${loggroup}"
      }
    },
    "cpu": 1,
    "environment": [
      {
        "name": "RUST_LOG",
        "value": "info"
      }
    ],
    "mountPoints": [],
    "volumesFrom": [],
    "dependsOn": []
  }
]