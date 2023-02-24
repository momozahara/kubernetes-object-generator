# KUBERNETES-OBJECT-GENERATOR
A tool use to generate kubernetes simple object spec and status both [deployment](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/) and [service](https://kubernetes.io/docs/concepts/services-networking/service/) as sperate deployment.yaml and service.yaml

## Installation
### Remote
``` bash
cargo install --git https://github.com/momozahara/kubernetes-object-generator.git
```
### Local
``` bash
cargo install --path .
```

## Usage
Type binary name in the folder you want to generate file to
``` bash
kube-gen
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
          -  containerPort: 3000 # to edit
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
  clusterIP: 0.0.0.0 # to edit
  ports:
  - port: 3000 # to edit
    targetPort: 3000 # to edit
    nodePort: 30000 # to edit
```