# Electrify

A wasmCloud-based application running on Kubernetes via [kind](https://kind.sigs.k8s.io/), using ORAS to push WebAssembly components to a local OCI registry.

---

## Prerequisites

Make sure the following tools are installed before proceeding:

- [`kind`](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
- [`docker`](https://docs.docker.com/get-docker/)
- [`helm`](https://helm.sh/docs/intro/install/)
- [`kubectl`](https://kubernetes.io/docs/tasks/tools/)
- [`oras`](https://oras.land/docs/installation)
- [Rust](https://rustup.rs/) (to build `wash` and the components)
- `wash` v2 — built from the wasmCloud source (see below); the official installer release does **not** include the v2 runtime this project uses

---

## Clone the Repository

```bash
git clone https://github.com/rabelmervin/Electrify.git
cd Electrify
```

---

## Build the `wash` Binary (v2)

Clone the wasmCloud source and build `wash`, then place it at `wasm_components/build/wash`:

```bash
git clone https://github.com/wasmCloud/wasmCloud.git
cd wasmCloud
cargo build --release -p wash
mkdir -p ../wasm_components/build
cp target/release/wash ../wasm_components/build/wash
cd ..
```

> This takes several minutes on first build. The same binary serves as both the build tool (`wash build`) and the runtime host (`wash host`).

---

## Step-by-Step Setup

### Step 1 — Start the Local OCI Registry

Start a local Docker registry that will store your WebAssembly component images:

```bash
docker run -d --restart=always -p 5005:5000 --name electrify-registry registry:2
```

---

### Step 2 — Create the kind Cluster

Create a kind cluster pre-configured to mirror the local registry:

```bash
cat <<'EOF' | kind create cluster --name electrify --config /dev/stdin
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
containerdConfigPatches:
- |-
  [plugins."io.containerd.grpc.v1.cri".registry]
    [plugins."io.containerd.grpc.v1.cri".registry.mirrors]
      [plugins."io.containerd.grpc.v1.cri".registry.mirrors."localhost:5005"]
        endpoint = ["http://electrify-registry:5000"]
EOF
```

---

### Step 3 — Connect the Registry to the kind Network

Allow the kind cluster nodes to reach the local registry container:

```bash
docker network connect kind electrify-registry
```

---

### Step 4 — Install the wasmCloud Runtime Operator via Helm

Deploy the wasmCloud runtime operator into a dedicated `electrify` namespace:

```bash
helm install electrify \
  oci://ghcr.io/wasmcloud/charts/runtime-operator \
  --version v2-canary \
  --namespace electrify \
  --create-namespace \
  --set global.tls.enabled=false
```

---

### Step 5 — Wait for the Operator to be Ready

Verify the operator deployment rolls out successfully:

```bash
kubectl rollout status deployment/runtime-operator -n electrify
```

Check that all pods are running:

```bash
kubectl get pods -n electrify
```

---

### Step 6 — Build the WebAssembly Components

> Prebuilt `.wasm` artifacts ship in `wasm_components/build/`, so this step is optional on a fresh clone. Rebuild whenever you change the component code or the WIT contract.

Requires [Rust](https://rustup.rs/) with the `wasm32-wasip2` target (`rustup target add wasm32-wasip2`).

```bash
(cd wasm_components/order_processor && wash build --skip-fetch)
(cd wasm_components/invoice_renderer && wash build --skip-fetch)
```

Each component's `.wash/config.yaml` drives the build and copies the resulting `.wasm` into `wasm_components/build/` automatically.

---

### Step 7 — Push WebAssembly Components to the Registry

Push the compiled `.wasm` components to the local OCI registry using ORAS:

**Order Processor:**

```bash
oras push localhost:5005/electrify/order-processor:latest \
  wasm_components/build/order_processor.wasm:application/vnd.module.wasm.content.layer.v1+wasm \
  --plain-http --disable-path-validation
```

**Invoice Renderer:**

```bash
oras push localhost:5005/electrify/invoice-renderer:latest \
  wasm_components/build/invoice_renderer.wasm:application/vnd.module.wasm.content.layer.v1+wasm \
  --plain-http --disable-path-validation
```

---

### Step 8 — Apply Kubernetes Workload Manifests

Deploy the application workloads to the cluster:

```bash
kubectl apply -f k8s/workload.yaml
```

---

### Step 9 — Port-Forward NATS

Expose the NATS service locally so the wasmCloud host can connect to it:

```bash
kubectl port-forward -n electrify svc/nats 4223:4222
```

> Keep this running in a **separate terminal**.

---

### Step 10 — Start the wasmCloud Host

Open a **new terminal** and run the following steps:

**10a. Allow non-root port binding (required to bind to port 80):**

```bash
sudo sysctl net.ipv4.ip_unprivileged_port_start=80
```

**10b. Start the wasmCloud host:**

```bash
RUST_LOG=info ./wasm_components/build/wash host \
  --host-group default \
  --scheduler-nats-url nats://localhost:4223 \
  --data-nats-url nats://localhost:4223 \
  --http-addr 0.0.0.0:80 \
  --allow-insecure-registries
```

> Wait for the `Starting workload` log line — the operator needs ~30 seconds to place the workload on the host.

---

### Step 11 — Generate an Invoice

**In the browser:** open <http://electrify.localhost.direct/>, fill in the order form, and click **Generate Invoice**.

**Or via curl (JSON):**

```bash
curl -X POST http://electrify.localhost.direct/order \
  -H "Content-Type: application/json" \
  -d '{"customer-name":"Acme Corp","items":[{"name":"Widget A","price":49.99,"quantity":3},{"name":"Widget B","price":199.99,"quantity":1}],"discount-percent":10}'
```

The `order-processor` component validates the order and computes the totals, then calls the `invoice-renderer` component over its WIT interface to produce the invoice:

```
=======================================
              INVOICE
=======================================
Customer: Acme Corp
---------------------------------------
Subtotal: $349.96
Discount: -$35.00
Tax:      +$25.20
---------------------------------------
TOTAL:    $340.16
=======================================
```

