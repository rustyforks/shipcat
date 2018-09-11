FROM alpine:3.7

ENV KUBEVER=1.10.1 \
    HELMVER=2.10.0 \
    HELMDIFFVER="2.9.0%2B1" \
    KUBEVALVER=0.7.2 \
    KUBETESTVER=0.1.1 \
    VAULTVER=0.11.1 \
    HOME=/config \
    SSL_CERT_DIR=/etc/ssl/certs/

# Add core dependencies of validation
RUN apk update && \
    apk add --no-cache curl ca-certificates make bash jq git python3 unzip && \
    apk add --no-cache libffi-dev g++ python3-dev openssl-dev && \
    pip3 install --upgrade pip && \
    pip3 install yamllint yq && \
    pip3 install semver jira && \
    apk del libffi-dev g++ python3-dev openssl-dev

# Install shipcat (built for musl outside)
ADD shipcat.x86_64-unknown-linux-musl /usr/local/bin/shipcat

# Install kubectl (see https://aur.archlinux.org/packages/kubectl-bin )
ADD https://storage.googleapis.com/kubernetes-release/release/v${KUBEVER}/bin/linux/amd64/kubectl /usr/local/bin/kubectl

# Install everything
# NB: skipping https://github.com/garethr/kubetest because alpine dylibs fail
RUN set -x && \
    chmod +x /usr/local/bin/kubectl && \
    curl -sSL https://storage.googleapis.com/kubernetes-helm/helm-v${HELMVER}-linux-amd64.tar.gz | tar xz -C /usr/local/bin --strip-components=1 && \
    curl -sSL https://github.com/garethr/kubeval/releases/download/${KUBEVALVER}/kubeval-linux-amd64.tar.gz | tar xvz -C /usr/local/bin && \
    curl -sSL https://releases.hashicorp.com/vault/${VAULTVER}/vault_${VAULTVER}_linux_amd64.zip > vault.zip && \
    unzip vault.zip && mv vault /usr/local/bin && \
    #curl -sSL https://github.com/garethr/kubetest/releases/download/${KUBETESTVER}/kubetest-linux-amd64.tar.gz | tar xzv -C /usr/local/bin && \
    # Create non-root user
    adduser kubectl -Du 1000 -h /config && \
    \
    # Basic check it works.
    kubectl version --client && \
    shipcat --version && \
    helm version -c && \
    kubeval --version
    #kubetest -h

# Setup helm and plugins
# Currently the version pinning mechanism in helm plugin does not work for tags with + in them
# See https://github.com/databus23/helm-diff/issues/50
# Also cannot sanity check installation because it tries to talk to the cluster
RUN set -x && \
    helm init -c && \
    curl -sSL https://github.com/databus23/helm-diff/releases/download/v${HELMDIFFVER}/helm-diff-linux.tgz | tar xvz -C $(helm home)/plugins

# Hack to have diff in the bin sub-directory
RUN mkdir $(helm home)/plugins/diff/bin && \
    cp $(helm home)/plugins/diff/diff $(helm home)/plugins/diff/bin/

# Install kong-configurator deps
ADD kong-configurator kong-configurator
RUN pip3 install -r kong-configurator/requirements.txt

USER kubectl
