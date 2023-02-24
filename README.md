# KUBERNETES-OBJECT-GENERATOR
A tool use to generated kubernetes simple object spec and status both [deployment](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/) and [service](https://kubernetes.io/docs/concepts/services-networking/service/) as sperate deployment.yaml and service.yaml

## Installation
### Remote
``` bash
cargo install --git https://github.com/momozahara/kubernetes-object-generator.git
```
### Local
``` bash
cargo install --path .
```

## Template
### Deployment
``` yaml
apiVersion: apps/v1
kind: Deployment

metadata:
  name: {name}-deployment
  labels:
    app: {label}

spec:
  selector:
    matchLabels:
      app: {label}
  replicas: 1
  template:
    metadata:
      labels:
        app: {label}
    spec:
      containers:
      - name: {cname}
        image: {image}
        imagePullPolicy: Always
        ports:
          -  containerPort: 3000
      imagePullSecrets: # optional
      - name: {secret}
```
### Service
``` yaml
apiVersion: v1
kind: Service

metadata:
  name: {name}-service
  labels:
    app: {label}

spec:
  selector:
    app: {label}
  type: {type}
  clusterIP:
  ports:
  - port: 3000
    targetPort: 3000
    nodePort: 30000
```