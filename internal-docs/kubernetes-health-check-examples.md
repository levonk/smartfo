# Kubernetes Health Check Probes for Smartfo

This document provides examples of Kubernetes probe configurations for smartfo health checks.

## HTTP Probe Configuration

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: smartfo-daemon
spec:
  replicas: 1
  selector:
    matchLabels:
      app: smartfo
  template:
    metadata:
      labels:
        app: smartfo
    spec:
      containers:
      - name: smartfo
        image: smartfo:latest
        ports:
        - containerPort: 8080
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 30
          timeoutSeconds: 5
          successThreshold: 1
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 10
          timeoutSeconds: 5
          successThreshold: 1
          failureThreshold: 3
```

## Command Probe Configuration

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: smartfo-daemon
spec:
  replicas: 1
  selector:
    matchLabels:
      app: smartfo
  template:
    metadata:
      labels:
        app: smartfo
    spec:
      containers:
      - name: smartfo
        image: smartfo:latest
        livenessProbe:
          exec:
            command:
            - smartfo
            - health
            - check
          initialDelaySeconds: 5
          periodSeconds: 30
          timeoutSeconds: 5
          successThreshold: 1
          failureThreshold: 3
        readinessProbe:
          exec:
            command:
            - smartfo
            - health
            - check
          initialDelaySeconds: 5
          periodSeconds: 10
          timeoutSeconds: 5
          successThreshold: 1
          failureThreshold: 3
```

## Signal-Based Probe Configuration

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: smartfo-daemon
spec:
  replicas: 1
  selector:
    matchLabels:
      app: smartfo
  template:
    metadata:
      labels:
        app: smartfo
    spec:
      containers:
      - name: smartfo
        image: smartfo:latest
        livenessProbe:
          exec:
            command:
            - smartfo
            - health
            - check
            - --signal
          initialDelaySeconds: 5
          periodSeconds: 30
          timeoutSeconds: 5
          successThreshold: 1
          failureThreshold: 3
        readinessProbe:
          exec:
            command:
            - smartfo
            - health
            - check
            - --signal
          initialDelaySeconds: 5
          periodSeconds: 10
          timeoutSeconds: 5
          successThreshold: 1
          failureThreshold: 3
```

## Probe Configuration Parameters

### Liveness Probe
- **Purpose**: Determines if the container is running and responsive
- **Behavior**: If the probe fails, Kubernetes restarts the container
- **Recommended settings**:
  - `initialDelaySeconds: 5` - Wait 5 seconds before first check
  - `periodSeconds: 30` - Check every 30 seconds
  - `timeoutSeconds: 5` - Timeout after 5 seconds
  - `failureThreshold: 3` - Allow 3 consecutive failures before restart

### Readiness Probe
- **Purpose**: Determines if the container is ready to serve traffic
- **Behavior**: If the probe fails, Kubernetes removes the pod from service endpoints
- **Recommended settings**:
  - `initialDelaySeconds: 5` - Wait 5 seconds before first check
  - `periodSeconds: 10` - Check every 10 seconds (more frequent than liveness)
  - `timeoutSeconds: 5` - Timeout after 5 seconds
  - `failureThreshold: 3` - Allow 3 consecutive failures before marking unready

## Health Check Exit Codes

- **0**: Healthy - The daemon is operational and responsive
- **1**: Unhealthy - The daemon is not responding or has errors

## Security Considerations

- HTTP health check endpoint binds to `127.0.0.1` (localhost only) by default
- For production deployments, consider adding authentication to the HTTP endpoint
- Ensure the health check does not expose sensitive information
- Keep health checks lightweight to avoid impacting daemon performance

## Troubleshooting

### Health Check Failing
1. Check daemon logs: `kubectl logs <pod-name>`
2. Verify daemon is running: `kubectl exec <pod-name> -- ps aux | grep smartfo`
3. Test health check manually: `kubectl exec <pod-name> -- smartfo health check`
4. Check for resource constraints: `kubectl describe pod <pod-name>`

### Slow Health Check Responses
- Increase `timeoutSeconds` in probe configuration
- Check daemon performance metrics
- Verify network connectivity within the pod

### Intermittent Failures
- Increase `failureThreshold` to allow transient failures
- Check for resource contention (CPU/memory)
- Review daemon logs for error patterns
